# SignPath Foundation: the application for TotalMail

Free code signing for the Windows installers ([signpath.org/apply](https://signpath.org/apply), a form reviewed by hand). Prepared on 2026-11-14 and kept until the project has some reputation: the form asks for proof that it is used. The contact fields (name, email) are filled in by the maintainer when sending it.

## Before sending

- Collect reputation: downloads of the releases (GitHub shows them per file), stars, and mentions (a post on the Total.js blog or Telegram, the App Store page of the native app).
- On the download page (`totalmail.totaljs.es/#download`), a **Code signing policy** section with: "Free code signing provided by SignPath.io, certificate by SignPath Foundation", the team (authors, reviewers, approvers: the maintainers of github.com/totaljs-es) and a link to the privacy policy. SignPath asks for it on the download page.
- Multi-factor authentication on GitHub for everyone in the team (and later on SignPath).

## The answers

| Field | Answer |
| --- | --- |
| Project Name | TotalMail |
| Repository URL | https://github.com/totaljs-es/totalmail-app |
| Homepage URL | https://totalmail.totaljs.es |
| Download URL | https://totalmail.totaljs.es/#download (once it mentions SignPath) |
| Privacy Policy URL | https://totalmail.totaljs.es/privacy/ |
| Wikipedia URL | (none) |
| Tagline | The webmail of your self-hosted TotalMail server in an app of its own for Windows, Linux and Android. |
| Category | the closest to email / communication |
| Maintainer Type | the closest to a small company or open source group |
| Build System | GitHub Actions |
| Company Name | Spanish Total.js Platform |
| Primary Discovery Channel | as it happened (the source: an AI assistant while preparing the release) |

**Description**

TotalMail is an open source email client that people host on their own server (https://github.com/totaljs-es/totalmail): it connects existing IMAP and SMTP accounts, with a web app and native apps. This repository builds its desktop and Android app: a small Tauri wrapper that shows the web app of the user's own server in a window of its own, with system notifications. Links and sign-in with Google or Microsoft open in the browser, and the pages of the server can reach only the notification commands of the app. The installers are built and published by GitHub Actions from a version tag.

**Reputation**

TotalMail is built on Total.js (https://github.com/totaljs/framework, 4,300+ stars), and its native app for Mac, iPhone and iPad is published on the App Store. The server is open source at https://github.com/totaljs-es/totalmail, with Docker images at ghcr.io/totaljs-es/totalmail. Add at the time of sending: download counts of the releases, stars, and links to posts or community discussions.

## The checkboxes

The maintainer ticks them when sending: the Code of Conduct (required), and storing personal data (required). "Other communications from SignPath" stays unticked.

## After acceptance

Connect the release workflow to SignPath (its GitHub action submits the unsigned installers and returns them signed after a manual approval), set the product name and version in the Windows metadata (Tauri does), and remove the SmartScreen note from the release notes, the README and the website.
