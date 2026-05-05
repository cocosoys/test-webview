# Test WebView

<p align="center">
  <strong>A WebView desktop app tech demo based on Tauri 2.x + Vue 3</strong>
</p>

<p align="center">
  English | <a href="./README.md">中文</a>
</p>

---

## 📖 Overview

Test WebView is a cross-platform WebView desktop application demo built with **Tauri 2.x + Vue 3 + TypeScript**. It demonstrates how to use modern web technologies to replace traditional desktop app development, covering common desktop features such as system tray, native menus, frameless windows, i18n, and theme switching.

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🖼️ Frameless Window | Custom title bar with drag, minimize/maximize/close |
| 🎨 Theme Switching | Light / Dark / Follow System — three modes |
| 🌐 Internationalization | Chinese & English, based on vue-i18n, extensible |
| 📌 System Tray | Minimize to tray, context menu, double-click to restore |
| 📋 Native Menu | File / View / Help menu bar, menu events drive routing |
| 📁 File System | Config file read/write, folder selection dialog |
| ⚙️ Settings Panel | Language, theme, close behavior, config path settings |
| 🚪 Close Behavior | Choose "Minimize to Tray / Quit / Ask Every Time", with remember option |
| 📊 Dashboard | System info display, quick action shortcuts |
| 💾 YAML Config | Auto-generated config file with version management and persistence |

## 🛠️ Tech Stack

| Layer | Technology |
|-------|------------|
| **Backend** | Rust · Tauri 2.x |
| **Frontend** | Vue 3 · TypeScript · Vite |
| **Styling** | TailwindCSS 4 |
| **State** | Pinia |
| **Routing** | Vue Router 4 |
| **i18n** | vue-i18n 11 |
| **Icons** | Lucide Icons |
| **Config** | serde_yaml |

## 📁 Project Structure

```
test-webview/
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── lib.rs           # Tauri commands, lifecycle
│   │   ├── config.rs        # YAML config management
│   │   ├── tray.rs          # System tray
│   │   └── menu.rs          # Native menu
│   ├── icons/               # App icons
│   ├── capabilities/        # Tauri 2 permissions
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                     # Vue frontend
│   ├── components/
│   │   ├── layout/          # Layout (title bar, sidebar)
│   │   ├── dashboard/       # Dashboard
│   │   └── settings/        # Settings (theme, language, close behavior, config path, about)
│   ├── i18n/                # zh-CN / en-US language packs
│   ├── router/              # Route config
│   ├── stores/              # Pinia stores
│   ├── styles/              # Global styles + TailwindCSS
│   ├── App.vue
│   └── main.ts
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://rustup.rs/) ≥ 1.86
- [pnpm](https://pnpm.io/) / npm

### Install Dependencies

```bash
npm install
```

### Development

```bash
npm run tauri dev
```

### Build for Production

```bash
# Build without installer
npm run tauri build -- --no-bundle

# Build with installer (Windows: .msi / .exe)
npm run tauri build
```

Output is located at `src-tauri/target/release/bundle/`.

## ⚙️ Configuration

The app auto-generates a default config at `{APP_DATA}/config.yaml` on first launch:

```yaml
version: "1.0"
locale: zh-CN
theme: system
close_action: ask
```

| Field | Description | Values |
|-------|-------------|--------|
| `version` | Config file format version | `"1.0"` |
| `locale` | Interface language | `zh-CN`, `en-US` |
| `theme` | Theme mode | `light`, `dark`, `system` |
| `close_action` | Window close behavior | `ask`, `minimize`, `quit` |

## 🔌 Tauri Commands

| Command | Description |
|---------|-------------|
| `get_config` | Read config (returns JSON) |
| `save_config` | Save config (accepts JSON, writes YAML) |
| `get_system_info` | Get system info (OS / arch / family) |
| `get_app_version` | Get config version string |

## 📚 Tutorials

Learn each module from scratch, see [Tutorial Index](./docs/README.md) (in Chinese):

| # | Tutorial | Description |
|---|----------|-------------|
| 01 | [Project Setup](./docs/01-project-setup.md) | Create Tauri + Vue 3 project, configure TailwindCSS, Router, Pinia, i18n |
| 02 | [Custom Titlebar](./docs/02-custom-titlebar.md) | Frameless window titlebar, drag and window controls |
| 03 | [System Tray](./docs/03-system-tray.md) | Tray icon, context menu, double-click event |
| 04 | [Native Menu](./docs/04-native-menu.md) | File/View/Help menus, menu events drive routing |
| 05 | [Config Management](./docs/05-config-management.md) | YAML config read/write, defaults, frontend-backend communication |
| 06 | [Theme Switching](./docs/06-theme-switching.md) | Light/Dark/System themes, CSS variables approach |
| 07 | [Internationalization](./docs/07-internationalization.md) | vue-i18n setup, Chinese/English switching |
| 08 | [Close Behavior](./docs/08-close-behavior.md) | Window close interception, minimize-to-tray/quit/ask |
| 09 | [Build & Bundle](./docs/09-build-and-bundle.md) | Dev debugging, production build, installer generation |

## 📄 License

[MIT License](./LICENSE)
