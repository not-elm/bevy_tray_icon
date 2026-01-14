# Repository Guidelines

## Project Structure & Module Organization
This is a Rust library crate for integrating system tray icons with Bevy. Core library code lives in `src/`, with submodules under `src/resource/` and `src/plugin/`. Example apps and demo media are in `examples/` (see `examples/window_visible.rs` and `examples/window_visible.gif`). Static assets used for demos live in `assets/`. Build artifacts land in `target/` and should not be committed.

## Build, Test, and Development Commands
- `cargo build`: Compile the library.
- `cargo test`: Run unit tests (if present).
- `cargo run --example window_visible`: Launch the demo example.
- `cargo fmt`: Format Rust sources using rustfmt.
- `cargo clippy`: Run lint checks; prefer fixing warnings before submitting.

## Coding Style & Naming Conventions
Use standard Rust 2021 style. Prefer 4-space indentation, `snake_case` for functions/modules, and `CamelCase` for types. Keep modules focused and small; public APIs should live in `src/lib.rs` and be re-exported as needed. Avoid adding unnecessary dependencies and keep `bevy` features minimal (see `Cargo.toml`).

## Testing Guidelines
There is no dedicated `tests/` directory currently. If you add tests, place unit tests in the same module under `#[cfg(test)]` and name tests descriptively (e.g., `creates_menu_item`). For integration tests, add a `tests/` directory and use filenames like `menu_item.rs`. Run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines
Recent history shows short, action-focused commit messages like `add: change logs` and `update: README.md`, sometimes with PR references (e.g., `(#2)`). Follow that pattern: start with a clear verb and keep it concise. PRs should include:
- A brief description of the change and its motivation.
- Any relevant example commands or screenshots/GIFs (especially for UI/demo updates).
- Notes about compatibility with Bevy versions if impacted.

## Notes
This crate targets Bevy `0.18` and uses the `tray-icon` crate. If you change compatibility, update `README.md` and `CHANGELOG.md` accordingly.
