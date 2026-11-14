# TotalMail App

The webmail of your [TotalMail](https://github.com/totaljs-es/totalmail) server in an app of its own, for **Windows**, **Linux** and **Android**. On iPhone, iPad and Mac, TotalMail has its native app in the App Store.

TotalMail is self-hosted: the app needs the address of a TotalMail server, which it asks for the first time it opens.

## Download

Every version is in [Releases](https://github.com/totaljs-es/totalmail-app/releases), with the SHA-256 of each file:

| System | File |
| --- | --- |
| Windows 10 and 11 | `TotalMail-<version>-windows-x64-setup.exe` (or the `.msi`) |
| Android 7 or newer | `TotalMail-<version>-android.apk` |
| Linux | `.deb` (Debian, Ubuntu), `.rpm` (Fedora, openSUSE) or `.AppImage` |

- **Windows:** the installer is not signed yet. The first time, SmartScreen says that it protected your PC: choose **More info → Run anyway**. It installs for your user only and needs no administrator.
- **Android:** open the APK on the phone; Android asks to allow installing apps from that source (the browser or the file manager). Updates are installed the same way, over the old version.
  On Android the server must use `https`: Android does not let apps load plain `http` pages, so a server on the local network without HTTPS (`http://192.168.…`) opens on Windows, Linux and the Apple app but not on Android.

## What it does

- Shows the webmail of your server in its own window, with its icon: everything the webmail does, the app does, and every update of the server reaches it by itself.
- New mail arrives as a notification of the system (turn it on in the webmail: Settings → Preferences).
- Links in messages, and the sign-in with Google or Microsoft to add an account, open in your browser. Google does not allow signing in inside an app, and the account appears in TotalMail when you come back.
- **TotalMail → Use another server…** in the menu (Windows, Linux) changes the server. On Android, the app offers another server when the saved one does not answer; to change it otherwise, clear the storage of the app.

## Security

- Only your server is shown inside the app. Any other address opens in the browser, and the app's own page accepts only `https` (plain `http` only on the local network, as the Apple app).
- The pages of your server can call only the functions of notifications (show one, read and ask for the permission), and only from the address you entered (`src-tauri/src/lib.rs`, `grant`). They cannot read files, run programs or reach anything else of the system.
- The address of the server is the only thing the app keeps; the sign-in is the cookie of the webmail, as in a browser.

## Development

Needs Node.js 22 and Rust (stable).

```bash
npm ci
npm run dev        # the app, on this machine
npm test           # the tests of the Rust part
npm run build      # the installers of this system, in src-tauri/target/release/bundle
```

- `src/`: the page of the app (the address of the server) and `notification.js`, which every page of the server gets before its own scripts.
- `src-tauri/src/lib.rs`: the window, what stays inside it, the menu and the commands.
- `src-tauri/capabilities/app.json`: what the page of the app may call. The server gets its own capability at run time.

Android: `npx tauri android init` and `npx tauri android dev`, with the Android SDK, the NDK and `NDK_HOME`.

## Releasing

1. Change `version` in `package.json` and add the version to `CHANGELOG.md`.
2. Commit, tag `v<version>` and push the tag. The `Release` workflow tests, builds Windows, Linux and Android and publishes the release with the SHA-256 of each file.

The Android key goes in the secrets of the repository (`ANDROID_KEYSTORE`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`; see the top of `.github/workflows/release.yml`). Keep a copy of the keystore somewhere safe: Android installs an update only when it is signed with the same key.

## Code signing

The Windows installers are not signed yet. The plan is the free signing of [SignPath Foundation](https://signpath.org) for open source projects, which signs builds made by this repository's workflow. It asks for a project that is already released, so it comes after the first versions.

## License

MIT
