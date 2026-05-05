# Test WebView

<p align="center">
  <strong>基于 Tauri 2.x + Vue 3 的 WebView 桌面应用技术验证原型</strong>
</p>

<p align="center">
  <a href="./README_EN.md">English</a> | 中文
</p>

---

## 📖 项目简介

Test WebView 是一个使用 **Tauri 2.x + Vue 3 + TypeScript** 构建的跨平台 WebView 桌面应用示例。它展示了如何用现代 Web 技术替代传统桌面应用开发模式，涵盖了系统托盘、原生菜单、无边框窗口、国际化、主题切换等常见桌面应用特性。

## ✨ 功能特性

| 功能 | 描述 |
|------|------|
| 🖼️ 无边框窗口 | 自定义标题栏，支持拖拽、最小化/最大化/关闭 |
| 🎨 主题切换 | 亮色 / 暗色 / 跟随系统，三档切换 |
| 🌐 国际化 | 中/英双语，基于 vue-i18n，可扩展更多语言 |
| 📌 系统托盘 | 最小化到托盘，右键菜单，双击恢复窗口 |
| 📋 原生菜单 | 文件 / 视图 / 帮助菜单栏，菜单事件驱动路由 |
| 📁 文件系统 | 配置文件读写，文件夹选择对话框 |
| ⚙️ 设置面板 | 语言、主题、关闭行为、配置路径等设置 |
| 🚪 关闭行为 | 可选"最小化到托盘 / 退出程序 / 每次询问"，支持记忆选择 |
| 📊 仪表盘 | 系统信息展示、快速操作入口 |
| 💾 YAML 配置 | 配置文件自动生成与持久化，含版本号管理 |

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| **后端** | Rust · Tauri 2.x |
| **前端** | Vue 3 · TypeScript · Vite |
| **样式** | TailwindCSS 4 |
| **状态管理** | Pinia |
| **路由** | Vue Router 4 |
| **国际化** | vue-i18n 11 |
| **图标** | Lucide Icons |
| **配置** | serde_yaml |

## 📁 项目结构

```
test-webview/
├── src-tauri/               # Rust 后端
│   ├── src/
│   │   ├── main.rs          # 入口
│   │   ├── lib.rs           # Tauri 命令、生命周期
│   │   ├── config.rs        # YAML 配置管理
│   │   ├── tray.rs          # 系统托盘
│   │   └── menu.rs          # 原生菜单
│   ├── icons/               # 应用图标
│   ├── capabilities/        # Tauri 2 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                     # Vue 前端
│   ├── components/
│   │   ├── layout/          # 布局组件（标题栏、侧边栏）
│   │   ├── dashboard/       # 仪表盘
│   │   └── settings/        # 设置页面（主题、语言、关闭行为、配置路径、关于）
│   ├── i18n/                # 中/英语言包
│   ├── router/              # 路由配置
│   ├── stores/              # Pinia 状态管理
│   ├── styles/              # 全局样式 + TailwindCSS
│   ├── App.vue
│   └── main.ts
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 🚀 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://rustup.rs/) ≥ 1.86
- [pnpm](https://pnpm.io/) / npm

### 安装依赖

```bash
# 安装前端依赖
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布

```bash
# 构建（不打包安装程序）
npm run tauri build -- --no-bundle

# 构建安装程序（Windows: .msi / .exe）
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`。

## ⚙️ 配置文件

应用首次启动时自动在 `{APP_DATA}/config.yaml` 生成默认配置：

```yaml
version: "1.0"
locale: zh-CN
theme: system
close_action: ask
```

| 字段 | 说明 | 可选值 |
|------|------|--------|
| `version` | 配置文件格式版本 | `"1.0"` |
| `locale` | 界面语言 | `zh-CN`, `en-US` |
| `theme` | 主题模式 | `light`, `dark`, `system` |
| `close_action` | 关闭窗口行为 | `ask`, `minimize`, `quit` |

## 🔌 Tauri 命令

| 命令 | 功能 |
|------|------|
| `get_config` | 读取配置（返回 JSON） |
| `save_config` | 保存配置（接收 JSON，写入 YAML） |
| `get_system_info` | 获取系统信息（OS / 架构 / 平台族） |
| `get_app_version` | 获取配置版本号 |

## 📚 教程文档

从零开始学习本项目的每个模块，详见 [教程目录](./docs/README.md)：

| # | 教程 | 说明 |
|---|------|------|
| 01 | [项目搭建](./docs/01-project-setup.md) | 创建 Tauri + Vue 3 项目，配置 TailwindCSS、Router、Pinia、i18n |
| 02 | [自定义标题栏](./docs/02-custom-titlebar.md) | 无边框窗口的自定义标题栏，拖拽与窗口控制 |
| 03 | [系统托盘](./docs/03-system-tray.md) | 创建托盘图标、右键菜单和双击事件 |
| 04 | [原生菜单栏](./docs/04-native-menu.md) | 文件/视图/帮助菜单，菜单事件驱动前端路由 |
| 05 | [配置管理](./docs/05-config-management.md) | YAML 配置读写、默认值、前后端通信 |
| 06 | [主题切换](./docs/06-theme-switching.md) | 亮色/暗色/系统三档主题，CSS 变量方案 |
| 07 | [国际化](./docs/07-internationalization.md) | vue-i18n 配置，中英双语切换 |
| 08 | [关闭行为](./docs/08-close-behavior.md) | 窗口关闭拦截，最小化到托盘/退出/询问 |
| 09 | [构建与打包](./docs/09-build-and-bundle.md) | 开发调试、生产构建、安装程序生成 |

## 📄 开源协议

[MIT License](./LICENSE)
