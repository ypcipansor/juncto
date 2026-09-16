# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **Cutover note:** The React/Webpack web codebase, mobile apps, and server/packaging
> infrastructure have been deleted. The repository root is now a single Rust workspace:
> a Leptos (WASM) frontend plus an Axum backend, with shared types in the `shared` crate.
> Build with `bash build.sh`, test with `cargo test --workspace`, and run the parity
> suite at `tests/e2e`. See `MIGRATION.md` for the migration history.

## Repository Layout

- `backend/` — Axum server. `backend/src/main.rs` builds the `Router`, serves the WASM
  frontend from `frontend/pkg` (via `ServeDir`/`ServeFile` fallback) and static assets
  from `backend/static`. Feature handlers live in `backend/src/handlers/`.
- `frontend/` — Leptos client-side-rendered WASM app. `frontend/src/lib.rs` mounts the
  `Home`/`Room` routes. Feature modules live directly under `frontend/src/` (e.g.
  `chat.rs`, `polls.rs`, `whiteboard.rs`, `settings.rs`) and UI widgets under
  `frontend/src/components_ui/`.
- `shared/` — Serde types shared between frontend and backend (`shared/src/lib.rs`).
- `tests/e2e/` — Playwright end-to-end parity suite.
- `build.sh`, `Makefile`, `Cargo.toml` — workspace build tooling and manifest.

## Development Commands

Run all of these from the repository root.

### Build
```sh
bash build.sh                 # builds the WASM frontend to frontend/pkg + copies index.html
cargo run -p backend          # serves the app on :3000
```

### Test
```sh
cargo test --workspace        # unit tests
cd tests/e2e && npx playwright test   # end-to-end parity suite
```

### Format / Lint
```sh
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

## Architecture Overview

### Workspace
The root `Cargo.toml` declares the workspace with `members = ["backend", "frontend",
"shared"]` and `resolver = "2"`. Inter-crate dependencies use `path = "../shared"` style
relative paths, which stay valid because the directory layout under the root is preserved.

### Frontend (Leptos CSR)
- `frontend/src/lib.rs` is the mount point and router (no SSR).
- State is managed in `frontend/src/state.rs` with handlers in `state_handlers.rs`.
- WebRTC/media logic lives in `webrtc.rs` and `media.rs`.
- i18n is hardcoded (EN/ID) in `i18n.rs`; settings persist via `localStorage` in `storage.rs`.
- Feature modules: `chat.rs`, `polls.rs`, `whiteboard.rs`, `reactions.rs`, `settings.rs`,
  `participants.rs`, `remote_control.rs`, `virtual_background.rs`, `analytics.rs`, etc.
- UI components under `frontend/src/components_ui/`.

### Backend (Axum)
- `backend/src/main.rs` creates a `broadcast` channel managing participants, polls,
  whiteboard actions, chat history, breakout rooms, remote-control sessions, and more.
- WebSocket messaging is handled in `backend/src/handlers/ws.rs`; route handlers live per
  feature in `backend/src/handlers/`.
- Static paths are resolved against `CARGO_MANIFEST_DIR` so the binary serves the frontend
  regardless of the working directory.

### Shared Types
- `shared/src/lib.rs` defines the serde types exchanged over the WebSocket (participants,
  polls, draw actions, room configs, chat messages, server messages).

## Code Style and Standards

### Conventional Commits
Follow [Conventional Commits](https://www.conventionalcommits.org) with scopes:
```
feat(chat): description
fix(webrtc): description
docs(readme): description
```
Available types: build, chore, ci, docs, feat, fix, perf, refactor, revert, style, test.

### Rust Conventions
- Run `cargo fmt` and `cargo clippy --workspace -- -D warnings` before committing.
- Use clear, purposeful module organization; add unit tests alongside logic where valuable.
- Keep `no_std`-style purity in `shared` only where it is already established.

## Testing and Quality Assurance

- The single Playwright suite is `tests/e2e` and is serial (`workers: 1`); it boots the app
  from the repo root via the `webServer` config.
- CI (`.github/workflows/rust-ci.yml`) runs `checks` (fmt, clippy, tests) and `e2e`
  (build + Playwright) jobs sequentially.
- `MIGRATION.md` documents the feature migration gap matrix and parity status.

## External Resources
- [Juncto Handbook](https://juncto.github.io/handbook/) - Comprehensive documentation
- [Community Forum](https://community.juncto.org/) - Ask questions and get support
- [Architecture Guide](https://juncto.github.io/handbook/docs/architecture) - System overview
- [Contributing Guidelines](https://juncto.github.io/handbook/docs/dev-guide/dev-guide-contributing/) - Detailed contribution process
