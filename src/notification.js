// Runs in every page of the window before its own scripts (initialization_script in src-tauri/src/lib.rs). On the pages of
// the server it puts a Notification that the app shows as a notification of the system: the web view of Android has none,
// and the one of Windows and Linux depends on the system. The page of the webmail uses it as it would in a browser.
// __PERMISSION__ is the state when the window was opened: granted, denied or default.
(function() {
	var here = location.protocol;
	if (here === 'tauri:' || location.hostname === 'tauri.localhost' || (here !== 'https:' && here !== 'http:'))
		return;
	var invoke = function(cmd, args) {
		var ipc = window.__TAURI_INTERNALS__;
		return ipc ? ipc.invoke(cmd, args) : Promise.reject(new Error('no ipc'));
	};
	var permission = '__PERMISSION__';
	function AppNotification(title, options) {
		options = options || {};
		this.title = String(title || '');
		this.body = options.body == null ? '' : String(options.body);
		this.tag = options.tag || '';
		this.onclick = this.onclose = this.onerror = this.onshow = null;
		var self = this;
		if (permission !== 'granted')
			return;
		invoke('notify', { title: this.title, body: this.body || null }).then(
			function() { self.onshow && self.onshow(); },
			function(err) { self.onerror && self.onerror(err); }
		);
	}
	AppNotification.prototype.close = function() {};
	AppNotification.prototype.addEventListener = function(name, fn) { this['on' + name] = fn; };
	AppNotification.prototype.removeEventListener = function(name) { this['on' + name] = null; };
	Object.defineProperty(AppNotification, 'permission', { get: function() { return permission; } });
	AppNotification.requestPermission = function(callback) {
		return invoke('notify_permission').then(
			function(value) { permission = value; callback && callback(value); return value; },
			function() { callback && callback(permission); return permission; }
		);
	};
	// The state of the system can be newer than the one the window was opened with (and on Android it is not known then).
	invoke('notify_state').then(function(value) { permission = value; }, function() {});
	try {
		Object.defineProperty(window, 'Notification', { value: AppNotification, configurable: true, writable: true });
	} catch (e) {
		window.Notification = AppNotification;
	}
})();
