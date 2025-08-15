# Hacksawdio

An offline-first, keyboard-driven, album-oriented, no-frills music client for Jellyfin,
created with Tauri, Typescript, and Rust.

## Screenshots

![screenshot showing search results and music player](/screenshots/screenshots.png?raw=true "Search Results and Music Player")

## Keyboard Shortcuts

Vim inspired keyboard shortcuts are used throughout the app. The following keys are used:

- Search Screen
  - `ctrl+s` - Toggle search screen
  - `/` - Focus search input
  - `esc` - Blur search input
  - `j` - Move down in results
  - `k` - Move up in results
  - `h` - Go to previous page
  - `l` - Go to next page
  - `Enter` - Download or play selected item
  - `ctrl+o` - Toggle online search
- Player Screen
  - `ctrl+p` - Toggle player screen
  - `j` - Skip to next track
  - `k` - Skip to previous track
  - `space` - Play or pause

## Running the App

```
npm install
npm run tauri dev
```
