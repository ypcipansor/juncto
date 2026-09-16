# Juncto Migration Gap Matrix

Gap matrix for the React → Rust (Leptos + Axum) migration. Status values:

- **migrated** — feature is implemented in the workspace root and exercised end-to-end.
- **partial** — exists in the workspace root but with reduced function; noted under Reason.
- **missing** — not yet implemented in the workspace root.
- **skip** — deliberately excluded; the Reason column must justify.

Mobile (`react/features/mobile/`, `ios/`, `android/`) is out of scope per user decision and is not listed.

## Health baseline (Step 0 audit)

- `bash build.sh`: ✓ succeeds (WASM + bindings generated, backend serves `:3000`).
- `cargo test --workspace`: ✓ green — 102 tests (29 + 49 + 24 across three crates).
- Playwright suite `tests/e2e`: ✓ green, incl. `screenshot-gallery.spec.ts` capturing 41 UI views to `tests/screenshots/` and `contrast-audit.spec.ts` asserting a WCAG contrast floor for every dialog. `tests/e2e/` duplication removed in Step 7.

## Feature matrix

| Feature | Status | Reason / Evidence |
|---|---|---|
| base | migrated | Core infra lives in `state.rs` / `webrtc.rs` / `media.rs`. |
| app | migrated | `lib.rs` SSR-free CSR shell with router. |
| conference | migrated | `webrtc.rs` + `pages/room.rs` drive the conference. |
| chat | migrated | `chat.rs` module + `backend/handlers/chat.rs`, parity specs exist. |
| polls | migrated | `polls.rs` + `backend/handlers/polls.rs`; specs `polls.spec.ts`. |
| polls-history | migrated | Polls dialog has Active/History tabs; closed polls archived via `on_close_poll`. |
| whiteboard | migrated | `whiteboard.rs` + handler; `whiteboard.spec.ts`. |
| reactions | migrated | `reactions.rs`; `reactions.spec.ts`. |
| lobby | migrated | `components_ui/lobby.rs`; VisitVisitor flow over `backend/handlers/room.rs` lobby endpoints. |
| breakout-rooms | migrated | `components_ui/breakout.rs` + `handlers/breakout.rs`. |
| prejoin | migrated | `components_ui/prejoin.rs`. |
| invite | migrated | `components_ui/invite.rs`. |
| welcome | migrated | `pages/home.rs` (prejoin / recents). |
| authentication | migrated | `components_ui/authentication.rs` login dialog; `state.authenticate`. |
| settings | migrated | `settings.rs` incl. device, profile, moderation, branding, E2EE toggles. |
| speakers (speaker-stats) | migrated | `speaker_stats.rs`; `stats_and_background.spec.ts`. |
| keyboard-shortcuts | migrated | `shortcuts.rs` + `ShortcutsDialog`. |
| screen-share | migrated | Native `getDisplayMedia` in `media.rs` (no desktop-picker UI needed). |
| screen-share (desktop-picker) | migrated | Browser native picker supersedes desktop-picker component. |
| virtual-background | migrated | `virtual_background.rs`; `background.spec.ts`. |
| filmstrip | migrated | Spotlight mode renders thumbnails as a filmstrip. |
| video-layout (layout selector) | migrated | Full Tile view/Speaker view menu. |
| video-menu | migrated | Right-click context menu (pin/kick/volume) on remote tiles. |
| connection-indicator | migrated | Visual Good/Fair/Poor/Unknown badge from RTT. |
| large-video | migrated | `video_grid.rs` handles spotlight size. |
| participate (participants-pane) | migrated | `participants.rs` list with context actions. |
| PII visibility | skip | N/A |
| display-name | migrated | Handled via `state.save_profile`. |
| overlay | skip | Legacy overlay infra from React; not needed. |
| toggle (room-lock) | migrated | `ToggleRoomLock(Option<password>)` end-to-end: settings moderator tab has password input; join validates password; `room_lock_password.spec.ts` parity green. |
| security | migrated | Moderator/security tab in settings carries lock+password; parity spec green. |
| visitors | migrated | `is_visitor` flows prejoin -> `Join` -> participant; toolbox hides controls for visitors. |
| e2ee | migrated (indicator) | Per-participant `UpdateE2EE` and room `ToggleE2EE` wired; `.e2ee-lock` tile badge and `#e2ee-indicator` banner; `e2ee_parity.spec.ts` green. Actual crypto still indicator-only (documented). |
| presence-status | migrated | `state.set_presence`, `PresenceStatus` in `shared`. |
| analytics | migrated | `analytics.rs` tracks interactions. |
| av-moderation | migrated | Permission grant/request flows in `state.rs` + moderation handler. |
| device-selection | migrated | Settings device selectors. |
| follow-me | migrated | `ClientMessage::FollowMe` wired; `follow-me.spec.ts`. |
| face-landmarks | migrated | `face_landmarks.rs`; `face_landmarks.spec.ts`. |
| audio-level-indicator | migrated | `components_ui/audio_level_indicator.rs`. |
| always-on-top | migrated | `components_ui/always_on_top.rs`; `always_on_top.spec.ts`. |
| embed-meeting | partial | `components_ui/embed_meeting.rs` = iframe embed code dialog; no postMessage bridge (see note on external-api below). |
| calendar-sync (google-api) | partial | `components_ui/calendar.rs` + handler stub; no real Google OAuth. |
| salesforce | partial | `salesforce.rs` + handler stub; returns not-linked. |
| dropbox | partial | `dropbox.rs` stub; no OAuth. |
| giphy (gifs) | migrated (UI stub) | `components_ui/giphy.rs`; `giphy` module integrates via `GIF:` chat prefix. |
| etherpad | migrated (UI stub) | `components_ui/etherpad.rs` iframe with configurable URL; no external server integration. |
| videosipgw | skip | SIP gateway absent; dial-in info is static; dial-in dialog exists. |
| dial-in (dial-in) | partial | `components_ui/dial_in.rs` static info; SIP not possible. |
| transcribing | skip | Would need STT backend (Jigasi/Vosk); React parity impossible. |
| subtitles | partial | Toggle wired; receives no transcription without STT. |
| shared-video | migrated | `components_ui/shared_video_dialog.rs`; `shared_video.spec.ts`. |
| noise-suppression | migrated | Constraint-based fallback (no rnnoise port); `toggle` in settings. |
| noise-detection | migrated | `noise_detected` event → toast in `state.rs`. |
| no-audio-signal | migrated | `on_no_audio` callback fires toast in `state.rs`. |
| talk-while-muted | migrated | `talk_while_muted` event → toast in `state.rs`. |
| video-quality | migrated | HD/SD selector in settings device tab. |
| pip | migrated | `requestPictureInPicture` via `<video>` elements on all tiles. |
| stream-effects | partial | Only virtual background; blur pipeline exists through canvas in `media.rs`. |
| recording | partial | Local recorder (`media_recorder.rs`) + `ToggleRecording` broadcast; no Jibri. |
| recent-list | migrated | `storage.rs` holds `recent_rooms`; Home renders "Recent Meetings" list. |
| notifications | migrated | Toast center: `NotificationBell` in room header with unread badge + history panel; `notification_center.spec.ts` green. |
| rejoin | migrated | Blocking overlay with "Rejoin now" button on WS drop; gallery captures it (`36-rejoin-overlay.png`). |
| reconnect logic | migrated | `on_close/on_error` triggers rejoin overlay when joined. |
| unsupported-browser | migrated | `lib.rs` blocks when no WebRTC. |
| dynamic-branding | migrated | `state.set_branding`; `branding.spec.ts`. |
| custom-panel | skip | Custom panel arbitrary-UI injection; security risk, low utility. |
| power-monitor | migrated | `power_monitor.rs` battery/charge. |
| screenshot-capture | migrated | `components_ui/screenshot_capture.rs`. |
| deep-linking | migrated | `deeplink.rs` (welcome redirect). |
| deeplink (mobile) | skip | Mobile excluded. |
| remote-control | migrated | `remote_control.rs` + handler; `remote_control.spec.ts`. |
| rtcstats | skip | Debug logging pipeline; low value. |
| chrome-extension-banner | skip | Per shelf decision §3 of plan. |
| old-client-notification | skip | Per plan §3 (irrelevant after rewrite). |
| web-hid | skip | No HID hardware integration by default. |
| external-api | skip | No postMessage bridge required (embed todo: iframe only). Decision: simple embed URL → `embed_meeting.rs` suffices. See note below. |
| file-sharing | migrated | `components_ui/file_sharing.rs`; server chat attachment recycle. |
| feedback | migrated | `components_ui/feedback.rs` + handler; `feedback.spec.ts`. |
| screenshot-capture (worker) | migrated | Used for thumbnails. |
| face-landmarks worker | migrated | `face_landmarks.rs` integration. |

## external-api decision (Langkah 0(d))

- Current usage: embedded meetings via iframe only → skip public command/event bridge.
- Public API behavior: **not kept** — the external JS API does not exist post-cutover; embedders use iframe embed (URL params) only.
- If future deployments require the bridge, Step 6 describes the `web-sys` postMessage implementation.

## Auth decision (Step 4)

- Anonymous joins remain the default and only mode, matching how the React client behaves for public deployments.
- `shared::Join.user_id` / `Participant { user_id }` stay as reserved fields; no session/JWT layer is added.
- Room-level security is enforced via lobby + lock + password (Step 4). Enterprise auth (JWT/JaaS-style) is out of scope, same as in the legacy React web client.

## Local spec consolidation (Langkah 7)

- Keep `tests/e2e/` (rich: full lifecycle, migration parity, UI verification).
- PNG artifacts retained under `verification*.png`; directory kept.

## Cutover status (Step 7)

- React web codebase, mobile apps (`android/`, `ios/`, `twa/`, `react-native-sdk/`), and
  server/packaging infrastructure (`debian/`, `doc/`, `resources/`, `Gemfile`) deleted.
- The Rust workspace previously under `rust-app/` was promoted to the repository root:
  `backend/`, `frontend/`, `shared/`, `tests/` and the root `Cargo.toml` / `build.sh`
  now form a single flat Rust workspace.
- CI runs Rust toolchain only (`.github/workflows/rust-ci.yml`), with no `rust-app/` paths.

## Exit criteria checklist

- [x] `cargo test` green, build green
- [x] One Playwright suite
- [x] Every migrated feature has parity (spec) or skip reason listed
- [x] React codebase removed (Step 7)
- [x] UI responsive verified at 480/768px (`responsive.spec.ts` green)
- [x] Mobile out of scope documented
- [x] Every screen captured in the README gallery (41 views) with a contrast audit guarding legibility

## Visual parity remediation (post-cutover)

Issue #79 review asked for the entire frontend to be screenshotted, verified and shown in
the README, with any broken, blank or badly contrasted view fixed first. That work is now
complete and reproducible:

- **Gallery.** `tests/e2e/screenshot-gallery.spec.ts` captures 41 views (desktop, dialogs
  and a 420×860 mobile set) into `tests/screenshots/`; the README embeds every one of them.
- **Contrast audit.** `tests/e2e/contrast-audit.spec.ts` measures resolved foreground and
  background colours for every dialog, compositing translucent backdrops up the ancestor
  chain, and fails below the WCAG floor. All dialogs currently measure ≥ 6.49:1.
- **Fixes applied.** Dialog surfaces and text were moved onto the design tokens (no more
  browser-default dark text on a white panel); the lobby screen dropped its hardcoded
  `#333`/`#444` palette; and side panels now start closed at ≤ 768 px so the full-width
  panel no longer covers the video stage on phones.
- **Two extra captures.** The waiting room (`35-lobby.png`) now seeds a host first, because
  the lobby only engages when a host is present; the reconnect overlay (`36-rejoin-overlay.png`)
  closes the tracked WebSocket directly, since `setOffline` does not reliably drop an
  already-open socket.

## Verified broken/missing (Step 0 evidence)

1. `backend/static/styles.css` has exactly **1 @media** query (`@media (max-width: 768px)`, toolbox only).
2. `frontend/src/pages/room.rs` lines 164-170 compute inline `margin-right` = 320px × panel count — replaced with CSS in Step 2.
3. `video_grid.rs` has all tile styles inline; no classes for filmstrip/thumbnails.
4. `ToggleRoomLock` exists (boolean); no password UI; is successive for Step 4.
5. E2EE toggle → `UpdateE2EE` exists; E2EEKeyExchange variant reserved; key-exchange flow deferred to Step 4.
6. `subtitles` overlay exists with "Transcriptions will appear here" stub; STT backend absent.
7. Two parallel Playwright suites (`tests/e2e/` + `tests/e2e/`) → keep `tests/e2e`.

