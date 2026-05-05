# 09 - 构建与打包

本教程讲解如何开发调试、构建生产版本和生成安装程序。

## 开发模式

```bash
npm run tauri dev
```

这个命令会同时：
1. 启动 Vite 开发服务器（端口 1420），支持热更新（HMR）
2. 编译并运行 Rust 后端
3. 在 WebView 中加载前端页面

### 开发模式特性

| 特性 | 说明 |
|------|------|
| 热更新（HMR） | 修改前端代码后自动刷新，无需重启 |
| Rust 热重载 | 修改 Rust 代码后自动重新编译并重启 |
| 开发者工具 | 右键 → 检查元素，打开 Chrome DevTools |
| 控制台日志 | Debug 模式下保留控制台窗口 |

### 常用开发命令

```bash
# 类型检查（不运行应用）
npx vue-tsc --noEmit

# 只启动前端开发服务器（不启动 Tauri）
npm run dev
```

## 生产构建

```bash
# 完整构建（包含安装程序）
npm run tauri build
```

这个命令会：
1. 运行 `vite build` 打包前端代码到 `dist/`
2. 编译 Rust 为 release 版本
3. 生成安装程序（Windows: .msi / .exe, macOS: .dmg, Linux: .AppImage）

### 不含安装程序的构建

如果安装程序工具链有问题（如 WiX 下载超时），可以跳过：

```bash
npm run tauri build -- --no-bundle
```

这只会生成可执行文件，不生成安装包。

### 构建产物位置

```
src-tauri/target/release/
├── test-webview.exe          # 可执行文件
└── bundle/
    ├── msi/                  # Windows MSI 安装包
    │   └── test-webview_0.1.0_x64_en-US.msi
    └── nsis/                 # Windows NSIS 安装包
        └── test-webview_0.1.0_x64-setup.exe
```

## Windows 安装程序

Tauri 在 Windows 上支持两种安装程序格式：

### WiX (MSI)

- 传统 Windows 安装包格式
- 首次构建需要下载 WiX 工具链（可能较慢或超时）
- 配置在 `tauri.conf.json` 的 `bundle.windows.wix` 中

### NSIS

- 更现代的安装包格式
- 构建速度通常比 WiX 快
- 配置在 `tauri.conf.json` 的 `bundle.windows.nsis` 中

## 常见问题

### Q: WiX 下载超时怎么办？

A: 使用 `--no-bundle` 跳过安装程序，或手动下载 WiX 并设置环境变量。

### Q: 构建后体积太大？

A: Tauri 的 release 构建已经做了大量优化。进一步减小体积的方法：
- 在 `Cargo.toml` 中设置 `opt-level = "z"` 和 `lto = true`
- 前端使用 `vite build` 的代码分割和 tree-shaking

### Q: 如何在构建时设置应用图标？

A: 使用 `tauri icon` 命令从一张 PNG 生成所有平台所需的图标：

```bash
npm run tauri icon path/to/icon.png
```

图标会生成到 `src-tauri/icons/` 目录。

### Q: 如何进行跨平台构建？

A: Tauri 不支持交叉编译，需要在目标平台上构建：
- Windows: 在 Windows 上构建
- macOS: 在 macOS 上构建
- Linux: 在 Linux 上构建

可以使用 GitHub Actions 等 CI/CD 平台实现多平台自动构建。

---

🎉 恭喜！你已经完成了所有教程。回到 [教程目录](./README.md)
