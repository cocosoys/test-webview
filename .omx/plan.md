# Tauri + Vue WebView 桌面应用 — 实施计划

> 项目定位：基于 Tauri 2.x + Vue 3 的 WebView 桌面应用技术验证原型  
> 创建日期：2026-05-05  
> 目标平台：Windows / macOS / Linux

---

## 一、项目初始化

### 1.1 Tauri 项目脚手架
- 使用 `create-tauri-app` 初始化 Tauri 2.x 项目
- 选择 Vue + TypeScript 模板
- 确认 Rust 工具链（rustup、cargo）就绪

### 1.2 前端依赖安装
- **Vue 3** — 已由脚手架包含
- **TypeScript** — 已由脚手架包含
- **TailwindCSS 4** — 原子化 CSS，支持亮/暗主题切换
- **vue-i18n** — 国际化框架，可扩展多语言
- **Vue Router** — 页面路由（仪表盘 / 设置 / 关于）
- **Pinia** — 状态管理（主题、语言、配置等全局状态）
- **@heroicons/vue** 或 **lucide-vue-next** — 图标库
- **Vite** — 已由脚手架包含

### 1.3 项目目录结构规划

```
test-webview/
├── src-tauri/               # Rust 后端
│   ├── src/
│   │   ├── main.rs          # 入口：窗口创建、托盘、菜单注册
│   │   ├── lib.rs           # Tauri 命令（文件操作、配置读写等）
│   │   ├── tray.rs          # 系统托盘逻辑
│   │   ├── menu.rs          # 原生菜单逻辑
│   │   └── config.rs        # 应用配置管理（读写 JSON/YAML）
│   ├── icons/               # 应用图标（各尺寸）
│   ├── capabilities/        # Tauri 2 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json      # Tauri 核心配置
├── src/                     # Vue 前端
│   ├── assets/              # 静态资源
│   ├── components/          # 通用组件
│   │   ├── layout/
│   │   │   ├── TitleBar.vue      # 自定义标题栏
│   │   │   ├── Sidebar.vue       # 侧边导航
│   │   │   └── AppLayout.vue     # 整体布局
│   │   ├── dashboard/
│   │   │   ├── StatsCard.vue     # 统计卡片
│   │   │   └── Dashboard.vue     # 仪表盘主页
│   │   └── settings/
│   │       ├── LanguageSelect.vue # 语言选择
│   │       ├── ThemeToggle.vue    # 主题切换
│   │       ├── ConfigPath.vue     # 配置路径设置
│   │       └── AboutPanel.vue     # 关于面板
│   ├── composables/         # 组合式函数
│   │   ├── useTheme.ts           # 主题逻辑
│   │   ├── useLocale.ts          # 国际化逻辑
│   │   └── useConfig.ts          # 配置读写逻辑
│   ├── i18n/                # 国际化
│   │   ├── index.ts              # vue-i18n 实例
│   │   ├── zh-CN.json            # 中文语言包
│   │   └── en-US.json            # 英文语言包
│   ├── router/              # 路由
│   │   └── index.ts
│   ├── stores/              # Pinia 状态
│   │   ├── app.ts                # 应用全局状态
│   │   └── settings.ts           # 设置状态
│   ├── styles/              # 全局样式
│   │   └── main.css              # Tailwind + 自定义样式
│   ├── App.vue
│   └── main.ts
├── package.json
├── vite.config.ts
├── tsconfig.json
└── tailwind.config.ts
```

---

## 二、功能模块实施（按优先级排序）

### 阶段 1：基础框架搭建

| 步骤 | 任务 | 产出 |
|------|------|------|
| 1.1 | Tauri + Vue 脚手架初始化 | 可运行的空壳应用 |
| 1.2 | 安装并配置 TailwindCSS | 亮/暗 CSS 变量体系 |
| 1.3 | 配置 Vue Router | 路由骨架（/dashboard, /settings, /about） |
| 1.4 | 配置 Pinia 状态管理 | 全局状态仓库 |
| 1.5 | 配置 vue-i18n | 中/英语言包 + 切换机制 |
| 1.6 | 自定义无边框窗口 + 标题栏 | 拖拽区、窗口控制按钮 |

### 阶段 2：核心桌面集成

| 步骤 | 任务 | 产出 |
|------|------|------|
| 2.1 | 系统托盘 | 托盘图标 + 右键菜单（显示/隐藏/退出） |
| 2.2 | 原生菜单栏 | 文件/视图/帮助菜单 + 快捷键 |
| 2.3 | 文件系统访问 | Tauri 命令：读/写配置文件，文件/文件夹对话框 |
| 2.4 | 应用配置管理 | Rust 端 JSON 配置读写，前端持久化同步 |

### 阶段 3：UI 界面开发

| 步骤 | 任务 | 产出 |
|------|------|------|
| 3.1 | 应用整体布局 | 侧边栏 + 主内容区 + 自定义标题栏 |
| 3.2 | 仪表盘页面 | 数据卡片、统计信息展示（示例数据） |
| 3.3 | 设置页面 — 语言切换 | 下拉选择，实时切换界面语言 |
| 3.4 | 设置页面 — 主题切换 | 亮/暗/跟随系统三档切换 |
| 3.5 | 设置页面 — 配置路径 | 显示当前路径 + 文件夹选择器修改 |
| 3.6 | 关于页面 | 应用版本、技术栈信息、开源协议 |

### 阶段 4：打磨与构建

| 步骤 | 任务 | 产出 |
|------|------|------|
| 4.1 | 窗口状态持久化 | 记住窗口大小/位置 |
| 4.2 | 开机自启（可选） | 系统托盘 + 自启动选项 |
| 4.3 | 跨平台构建测试 | Windows .msi/.exe, macOS .dmg, Linux .AppImage |
| 4.4 | 应用图标制作 | 各平台所需尺寸和格式 |

---

## 三、关键技术决策

### 3.1 主题系统
- 使用 TailwindCSS 的 `dark` 模式 + `class` 策略
- Pinia store 维护 `theme: 'light' | 'dark' | 'system'`
- 通过 `window.matchMedia('(prefers-color-scheme: dark)')` 监听系统主题
- 在 `<html>` 标签上切换 `dark` class

### 3.2 国际化
- vue-i18n 的 `legacy: false` 模式（Composition API）
- 语言包按页面拆分，通过 `import()` 懒加载
- Tauri 命令获取系统语言作为默认值
- 语言偏好持久化到本地配置

### 3.3 前后端通信
- Tauri 2.x 的 `invoke` 机制：前端 → Rust
- Tauri `Event` 机制：Rust → 前端推送
- 核心命令清单：
  - `get_config` / `save_config` — 读写应用配置
  - `select_directory` — 打开文件夹选择对话框
  - `get_system_info` — 获取系统信息（用于关于页面和仪表盘）

### 3.4 配置文件
- 格式：JSON
- 默认路径：`{APP_DATA}/config.json`（Tauri 的 `app_data_dir`）
- 可通过设置面板自定义存储路径
- 配置内容包括：语言、主题、窗口状态、自定义路径等

### 3.5 窗口定制
- `decorations: false` 关闭原生标题栏
- 自定义 TitleBar 组件：拖拽区（`data-tauri-drag-region`）+ 最小化/最大化/关闭按钮
- 通过 Tauri API 调用 `window.minimize()` / `window.toggle_maximize()` / `window.close()`

---

## 四、依赖清单

### Rust (Cargo.toml)
- `tauri` 2.x — 核心框架
- `tauri-plugin-shell` — Shell 操作
- `tauri-plugin-dialog` — 文件对话框
- `tauri-plugin-fs` — 文件系统
- `tauri-plugin-process` — 进程管理
- `serde` + `serde_json` — 序列化
- `dirs` 或 Tauri 内置的 path API — 路径管理

### 前端 (package.json)
- `vue` 3.x
- `vue-router` 4.x
- `pinia`
- `vue-i18n` 10.x
- `@tauri-apps/api` 2.x — Tauri 前端 API
- `@tauri-apps/plugin-shell` — Shell 插件前端
- `@tauri-apps/plugin-dialog` — 对话框插件前端
- `@tauri-apps/plugin-fs` — 文件系统插件前端
- `tailwindcss` 4.x
- `lucide-vue-next` — 图标
- `typescript`

---

## 五、验收标准

- [ ] 应用能正常启动，显示无边框自定义窗口
- [ ] 仪表盘页面展示示例数据卡片
- [ ] 系统托盘功能正常（图标、右键菜单、最小化到托盘）
- [ ] 原生菜单栏功能正常
- [ ] 亮/暗/跟随系统三种主题模式切换正常
- [ ] 中/英界面语言切换正常，可扩展新语言
- [ ] 配置文件路径可查看和修改（文件夹选择器）
- [ ] 所有设置持久化，重启后恢复
- [ ] 关于页面展示完整信息
- [ ] Windows 平台可正常打包和运行
