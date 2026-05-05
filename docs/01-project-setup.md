# 01 - 项目搭建

本教程介绍如何从零创建一个 Tauri 2.x + Vue 3 + TypeScript 的桌面应用项目。

## 前置条件

| 工具 | 版本要求 | 安装方式 |
|------|---------|---------|
| Node.js | ≥ 18 | [nodejs.org](https://nodejs.org/) |
| Rust | ≥ 1.86 | [rustup.rs](https://rustup.rs/) |
| npm | ≥ 8 | 随 Node.js 安装 |

验证安装：

```bash
node --version   # 应输出 v18+ 或更高
npm --version    # 应输出 8+ 或更高
rustc --version  # 应输出 1.86+ 或更高
cargo --version  # 应输出对应版本
```

## 第一步：创建 Tauri 项目

Tauri 官方提供了 `create-tauri-app` 脚手架工具：

```bash
npm create tauri-app@latest
```

按提示选择：
- 项目名称：`test-webview`
- 前端语言：`TypeScript`
- 包管理器：`npm`
- UI 模板：`Vue`
- UI 变体：`TypeScript`

这会生成一个基础的项目结构。

## 第二步：安装前端依赖

进入项目目录，安装核心依赖：

```bash
# Vue Router - 页面路由
npm install vue-router@4

# Pinia - 状态管理
npm install pinia

# vue-i18n - 国际化
npm install vue-i18n@11

# Tauri API - 前端调用 Rust 的桥接层
npm install @tauri-apps/api@2

# Tauri 插件前端包
npm install @tauri-apps/plugin-shell @tauri-apps/plugin-dialog @tauri-apps/plugin-fs @tauri-apps/plugin-process

# Lucide Icons - 图标库
npm install lucide-vue-next
```

安装开发依赖：

```bash
# TailwindCSS 4 + Vite 插件
npm install -D tailwindcss @tailwindcss/vite

# Tauri CLI
npm install -D @tauri-apps/cli@2
```

## 第三步：配置 Vite

编辑 `vite.config.ts`：

```typescript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath, URL } from "node:url";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,       // Tauri 默认端口
    strictPort: true,  // 端口被占用时报错而非自动换端口
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
```

**关键点：**
- `port: 1420` 必须与 `tauri.conf.json` 中的 `devUrl` 一致
- `strictPort: true` 确保端口固定，否则 Tauri WebView 找不到页面
- `watch.ignored` 避免监听 Rust 文件变更

## 第四步：配置 TailwindCSS

在 `src/styles/main.css` 中：

```css
@import "tailwindcss";
@custom-variant dark (&:where(.dark, .dark *));
```

然后在 `main.ts` 中引入：

```typescript
import "./styles/main.css";
```

**说明：**
- TailwindCSS 4 无需 `tailwind.config.js`，使用 CSS-first 配置
- `@custom-variant dark` 让 `.dark` 类名控制暗色模式

## 第五步：配置 Vue Router

创建 `src/router/index.ts`：

```typescript
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/dashboard" },
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("@/components/dashboard/Dashboard.vue"),
    },
    // ... 更多路由
  ],
});

export default router;
```

**说明：**
- 懒加载 `() => import(...)` 减小首屏体积
- `createWebHistory` 使用 HTML5 History 模式（URL 无 # 号）

## 第六步：配置 Pinia

在 `main.ts` 中：

```typescript
import { createPinia } from "pinia";

const app = createApp(App);
app.use(createPinia());
```

创建 store 文件，如 `src/stores/app.ts`，详见 [主题切换教程](./06-theme-switching.md)。

## 第七步：配置 vue-i18n

创建 `src/i18n/index.ts`：

```typescript
import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.json";
import enUS from "./en-US.json";

const i18n = createI18n({
  legacy: false,           // 使用 Composition API 模式
  locale: "zh-CN",         // 默认语言
  fallbackLocale: "en-US", // 回退语言
  messages: {
    "zh-CN": zhCN,
    "en-US": enUS,
  },
});

export default i18n;
```

在 `main.ts` 中注册：

```typescript
app.use(i18n);
```

## 第八步：配置 Rust 后端

编辑 `src-tauri/Cargo.toml`，添加依赖：

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-process = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
```

在 `src-tauri/src/lib.rs` 中注册插件：

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        // ...
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 第九步：配置 Tauri 权限

编辑 `src-tauri/capabilities/default.json`：

```json
{
  "identifier": "default",
  "description": "默认权限",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:default",
    "fs:default",
    "shell:allow-open",
    "process:default"
  ]
}
```

**说明：** Tauri 2 使用基于能力的权限系统，每个操作都需要显式授权。

## 第十步：运行开发模式

```bash
npm run tauri dev
```

如果一切配置正确，你会看到一个桌面窗口。

> ⚠️ **常见问题：** 如果 Rust 不在 PATH 中，需要先设置环境变量：
> ```bash
> # Windows CMD
> set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
> # PowerShell
> $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
> ```

## 下一步

→ [02 - 自定义标题栏](./02-custom-titlebar.md)
