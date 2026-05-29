# AsciiDiff

A desktop application for comparing rendered AsciiDoc files across git branches, commits, and tags. Built with Rust + Tauri v2 + Svelte 5.

![Main View](docs/screenshots/main-view.png)

## Features

- **Side-by-side diff** of rendered AsciiDoc documents between any two git refs (branches, tags, commits)
- **Word-level diff highlighting** shows exactly what changed
- **Split and Unified** layout modes with independent **Source** toggle
- **Resizable split** — drag the divider to adjust panel widths
- **File tree sidebar** grouped by change type (Modified, Added, Deleted)
- **AsciiDoc rendering** including headings, code blocks, admonitions, tables, lists
- **`include::` directive resolution** from git object store (recursive, max depth 8)
- **Sync scroll** between left and right panels
- **Keyboard shortcuts** for fast navigation
- **Desktop-native feel** — no context menu, no browser shortcuts, native window decorations
- **Dark and light themes** with live-preview settings

## Screenshots

### Branch Selection

![Branch Modal](docs/screenshots/branch-modal.png)

### Settings

![Settings](docs/screenshots/settings-modal.png)

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 20+
- Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

### Development

```bash
make dev
```

Or manually:

```bash
cd frontend
npm install
npm run tauri dev
```

### Build

```bash
# Linux AppImage (requires Docker/Podman)
./scripts/build-appimage.sh

# Or directly (produces platform-native bundles)
cd frontend && npx tauri build
```

### Test

```bash
make test          # Run all tests (Rust + Playwright)
make test-repo     # Create test git repo with sample AsciiDoc files
```

The test repo creates 3 branches (`main`, `feature/v2`, `hotfix/auth-fix`) and 2 tags (`v1.4.0`, `v2.0.0`) with nested includes, multiple file types, and realistic diff scenarios.

## Architecture

```
frontend/
├── src/                    # Svelte 5 frontend
│   ├── App.svelte          # Main app shell
│   └── lib/
│       ├── components/     # UI components (Toolbar, Sidebar, ContentArea, etc.)
│       └── stores/         # App state + settings (localStorage persistence)
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs          # Tauri commands + include resolution
│   │   ├── git.rs          # git2 operations (list refs, diff trees, read files)
│   │   └── render.rs       # AsciiDoc renderer + diff engine
│   └── Cargo.toml
├── tests/                  # Playwright e2e tests (21 tests)
└── package.json
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop runtime | [Tauri v2](https://v2.tauri.app/) |
| Backend | Rust (git2 0.21, similar 3.1) |
| Frontend | Svelte 5 + Vite 8 |
| Diff engine | [similar](https://crates.io/crates/similar) (line + word-level) |
| Git operations | [git2](https://crates.io/crates/git2) |
| Testing | Playwright (e2e) + cargo test (31 unit tests) |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+B` | Open branch selector |
| `Ctrl+S` | Swap branches |
| `Ctrl+E` | Toggle sidebar |
| `Ctrl+,` | Open settings |
| `Ctrl+↑/↓` | Navigate between diffs |
| `Esc` | Close modals |

## License

MIT
