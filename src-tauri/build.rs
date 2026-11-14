fn main() {
	// The commands of the app have permissions of their own (allow-<name>): the page of the app gets saved, connect and
	// forget (capabilities/app.json); the server gets only notify, notify-state and notify-permission, at run time (src/lib.rs).
	tauri_build::try_build(
		tauri_build::Attributes::new()
			.app_manifest(tauri_build::AppManifest::new().commands(&["saved", "connect", "forget", "notify", "notify_permission", "notify_state"])),
	)
	.expect("failed to run the build script");
}
