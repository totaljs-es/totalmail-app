// TotalMail App: the webmail of a TotalMail server in a window of its own, on Windows, Linux, macOS and Android.
//
// The window starts on the page of the app (../src), which asks for the address of the server once. Then it shows the
// server's own pages: the app adds no mail logic of its own, so every improvement of the webmail reaches it by itself.
// Only the server is shown inside the app; any other address opens in the system browser. The server's pages can call
// exactly two commands of the app (a notification and the permission for them), granted to that address alone.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::ipc::CapabilityBuilder;
use tauri::webview::WebviewWindowBuilder;
use tauri::{AppHandle, Manager, Url, WebviewUrl};
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tauri_plugin_opener::OpenerExt;

const MAIN: &str = "main";

// The address of the server, once it answered like a TotalMail.
#[derive(Default)]
struct Server(Arc<Mutex<Option<Url>>>);

// What the person typed, as the address of a server: https unless it is on the local network, nothing but the origin.
fn normalize(input: &str) -> Result<Url, &'static str> {
	let text = input.trim();
	if text.is_empty() {
		return Err("invalid");
	}
	let text = if text.contains("://") { text.to_string() } else { format!("https://{text}") };
	let mut url = Url::parse(&text).map_err(|_| "invalid")?;
	let host = url.host_str().unwrap_or("").to_lowercase();
	if host.is_empty() {
		return Err("invalid");
	}
	match url.scheme() {
		"https" => {}
		"http" if local(&host) => {}
		"http" => return Err("https"),
		_ => return Err("invalid"),
	}
	let _ = url.set_username("");
	let _ = url.set_password(None);
	url.set_path("/");
	url.set_query(None);
	url.set_fragment(None);
	Ok(url)
}

// The same rule as the Apple app: plain http only on this machine or a private network.
fn local(host: &str) -> bool {
	let host = host.trim_start_matches('[').trim_end_matches(']');
	if host == "localhost" || host == "::1" || host.ends_with(".local") || !host.contains('.') {
		return true;
	}
	let parts: Vec<u8> = host.split('.').filter_map(|p| p.parse().ok()).collect();
	parts.len() == 4
		&& (parts[0] == 127
			|| parts[0] == 10
			|| (parts[0] == 192 && parts[1] == 168)
			|| (parts[0] == 172 && (16..=31).contains(&parts[1])))
}

fn same_origin(a: &Url, b: &Url) -> bool {
	a.origin() == b.origin()
}

// The pages of the app itself: tauri://localhost, or http://tauri.localhost on Windows and Android.
fn is_app_page(url: &Url) -> bool {
	url.scheme() == "tauri" || url.host_str() == Some("tauri.localhost")
}

fn app_page() -> Url {
	let base = if cfg!(any(windows, target_os = "android")) { "http://tauri.localhost/" } else { "tauri://localhost/" };
	Url::parse(base).unwrap()
}

fn is_server(current: &Mutex<Option<Url>>, url: &Url) -> bool {
	current.lock().unwrap().as_ref().is_some_and(|server| same_origin(server, url))
}

fn outside(app: &AppHandle, url: &Url) {
	if matches!(url.scheme(), "http" | "https" | "mailto" | "tel") {
		let _ = app.opener().open_url(url.as_str(), None::<&str>);
	}
}

fn saved_path(app: &AppHandle) -> Option<std::path::PathBuf> {
	app.path().app_config_dir().ok().map(|dir| dir.join("server.txt"))
}

// Does the address answer like a TotalMail? Any version will do: the app shows the server's own pages.
async fn check(url: &Url) -> Result<(), &'static str> {
	let client = reqwest::Client::builder().timeout(Duration::from_secs(12)).build().map_err(|_| "unreachable")?;
	let response = client.get(url.join("health/").unwrap()).send().await.map_err(|_| "unreachable")?;
	if !response.status().is_success() {
		return Err("nottm");
	}
	let body: serde_json::Value = response.json().await.map_err(|_| "nottm")?;
	if body.get("status").and_then(|s| s.as_str()) == Some("ok") { Ok(()) } else { Err("nottm") }
}

// The server's pages may call `notify` and `notify_permission`, and nothing else.
fn grant(app: &AppHandle, url: &Url) {
	let origin = url.origin().ascii_serialization();
	let identifier = format!("server-{}", origin.replace(|c: char| !c.is_ascii_alphanumeric(), "-"));
	let capability = CapabilityBuilder::new(identifier)
		.remote(format!("{origin}/*"))
		.windows([MAIN, "print-*"])
		.permission("allow-notify")
		.permission("allow-notify-permission");
	let _ = app.add_capability(capability);
}

// The address saved last time, for the page of the app.
#[tauri::command]
fn saved(app: AppHandle) -> Option<String> {
	let text = std::fs::read_to_string(saved_path(&app)?).ok()?;
	normalize(&text).ok().map(|url| url.to_string())
}

// Checks the address, keeps it and shows the server. The error is a key that the page translates.
#[tauri::command]
async fn connect(app: AppHandle, server: tauri::State<'_, Server>, address: String) -> Result<(), String> {
	let url = normalize(&address).map_err(str::to_string)?;
	check(&url).await.map_err(str::to_string)?;
	if let Some(path) = saved_path(&app) {
		if let Some(dir) = path.parent() {
			let _ = std::fs::create_dir_all(dir);
		}
		let _ = std::fs::write(path, url.as_str());
	}
	grant(&app, &url);
	*server.0.lock().unwrap() = Some(url.clone());
	if let Some(window) = app.get_webview_window(MAIN) {
		window.navigate(url).map_err(|e| e.to_string())?;
	}
	Ok(())
}

// Forgets the server and goes back to the page of the app.
#[tauri::command]
fn forget(app: AppHandle, server: tauri::State<'_, Server>) {
	forget_server(&app, &server);
}

fn forget_server(app: &AppHandle, server: &Server) {
	if let Some(path) = saved_path(app) {
		let _ = std::fs::remove_file(path);
	}
	*server.0.lock().unwrap() = None;
	if let Some(window) = app.get_webview_window(MAIN) {
		let _ = window.navigate(app_page().join("index.html?change=1").unwrap());
	}
}

// A notification of the system, for the Notification of the web page (see src/notification.js).
#[tauri::command]
fn notify(app: AppHandle, title: String, body: Option<String>) -> Result<(), String> {
	let mut builder = app.notification().builder().title(title.chars().take(200).collect::<String>());
	if let Some(body) = body {
		builder = builder.body(body.chars().take(500).collect::<String>());
	}
	builder.show().map_err(|e| e.to_string())
}

#[tauri::command]
fn notify_permission(app: AppHandle) -> String {
	let state = match app.notification().permission_state() {
		Ok(PermissionState::Granted) => Ok(PermissionState::Granted),
		Ok(PermissionState::Denied) => Ok(PermissionState::Denied),
		_ => app.notification().request_permission(),
	};
	permission_name(state.ok()).to_string()
}

fn permission_name(state: Option<PermissionState>) -> &'static str {
	match state {
		Some(PermissionState::Granted) => "granted",
		Some(PermissionState::Denied) => "denied",
		_ => "default",
	}
}

#[cfg(desktop)]
fn menu(app: &AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
	use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
	let other = MenuItem::with_id(app, "other", "Use another server…", true, None::<&str>)?;
	let reload = MenuItem::with_id(app, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
	let main = Submenu::with_items(
		app,
		"TotalMail",
		true,
		&[&other, &reload, &PredefinedMenuItem::separator(app)?, &PredefinedMenuItem::quit(app, None)?],
	)?;
	// The shortcuts of copy and paste work through this menu on macOS.
	let edit = Submenu::with_items(
		app,
		"Edit",
		true,
		&[
			&PredefinedMenuItem::undo(app, None)?,
			&PredefinedMenuItem::redo(app, None)?,
			&PredefinedMenuItem::separator(app)?,
			&PredefinedMenuItem::cut(app, None)?,
			&PredefinedMenuItem::copy(app, None)?,
			&PredefinedMenuItem::paste(app, None)?,
			&PredefinedMenuItem::select_all(app, None)?,
		],
	)?;
	Menu::with_items(app, &[&main, &edit])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_notification::init())
		.manage(Server::default())
		.invoke_handler(tauri::generate_handler![saved, connect, forget, notify, notify_permission])
		.setup(|app| {
			let handle = app.handle().clone();
			let current = app.state::<Server>().0.clone();

			// What Notification.permission says when a page loads; requestPermission() asks the system.
			let permission = permission_name(app.notification().permission_state().ok());
			let script = include_str!("../../src/notification.js").replace("__PERMISSION__", permission);

			let navigating = current.clone();
			let opener = handle.clone();
			#[allow(unused_mut)]
			let mut builder = WebviewWindowBuilder::new(app, MAIN, WebviewUrl::App("index.html".into()))
				.initialization_script(script.clone())
				// Inside the window: the app's own page and the server. Anything else opens in the system browser, the
				// sign-in of Google and Microsoft too (Google refuses to sign in inside the web view of an app).
				.on_navigation(move |url| {
					if is_app_page(url) || matches!(url.scheme(), "about" | "data" | "blob") || is_server(&navigating, url) {
						return true;
					}
					outside(&opener, url);
					false
				});

			// One menu for the app: the menu bar of macOS, and the one of each window on Windows and Linux.
			#[cfg(desktop)]
			handle.set_menu(menu(&handle)?)?;

			#[cfg(desktop)]
			{
				// A window that the server opens (printing a message) is another window of the app, with the same
				// sign-in; the links of a message open in the system browser.
				let windows = handle.clone();
				let opening = current.clone();
				builder = builder
					.title("TotalMail")
					.inner_size(1280.0, 820.0)
					.min_inner_size(380.0, 520.0)
					.on_new_window(move |url, _features| {
						if is_server(&opening, &url) {
							let label = format!("print-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
							let _ = WebviewWindowBuilder::new(&windows, label, WebviewUrl::External(url))
								.title("TotalMail")
								.inner_size(900.0, 900.0)
								.initialization_script(script.clone())
								.build();
						} else {
							outside(&windows, &url);
						}
						tauri::webview::NewWindowResponse::Deny
					});
			}

			builder.build()?;
			Ok(())
		})
		.on_menu_event(|app, event| match event.id().as_ref() {
			"other" => forget_server(app, &app.state::<Server>()),
			"reload" => {
				if let Some(window) = app.get_webview_window(MAIN) {
					let _ = window.eval("location.reload()");
				}
			}
			_ => {}
		})
		.run(tauri::generate_context!())
		.expect("error while running TotalMail");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn an_address_becomes_the_origin_of_the_server() {
		assert_eq!(normalize("mail.example.com").unwrap().as_str(), "https://mail.example.com/");
		assert_eq!(
			normalize(" https://user:pw@mail.example.com:8443/inbox?x=1#y ").unwrap().as_str(),
			"https://mail.example.com:8443/"
		);
		assert_eq!(normalize("http://192.168.1.20:8000").unwrap().as_str(), "http://192.168.1.20:8000/");
		assert_eq!(normalize("http://localhost:8000").unwrap().as_str(), "http://localhost:8000/");
	}

	#[test]
	fn plain_http_is_refused_outside_the_local_network() {
		assert_eq!(normalize("http://mail.example.com"), Err("https"));
		assert_eq!(normalize("http://8.8.8.8"), Err("https"));
		assert_eq!(normalize("ftp://mail.example.com"), Err("invalid"));
		assert_eq!(normalize(""), Err("invalid"));
		assert_eq!(normalize("https://"), Err("invalid"));
	}

	#[test]
	fn only_the_server_is_inside_the_app() {
		let server = normalize("mail.example.com").unwrap();
		let current = Mutex::new(Some(server));
		assert!(is_server(&current, &Url::parse("https://mail.example.com/print/1/").unwrap()));
		assert!(!is_server(&current, &Url::parse("https://mail.example.com.evil.test/").unwrap()));
		assert!(!is_server(&current, &Url::parse("http://mail.example.com/").unwrap()));
		assert!(!is_server(&current, &Url::parse("https://accounts.google.com/").unwrap()));
		assert!(!is_server(&Mutex::new(None), &Url::parse("https://mail.example.com/").unwrap()));
	}
}
