# 03 - 系统托盘

本教程讲解如何创建系统托盘图标、右键菜单和双击事件。

## 什么是系统托盘？

系统托盘是桌面任务栏右下角的图标区域（Windows）或菜单栏右侧（macOS）。应用最小化到托盘后，用户可以通过托盘图标恢复窗口或退出应用。

## 第一步：启用托盘功能

在 `Cargo.toml` 中启用 Tauri 的 `tray-icon` feature：

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
```

在 `tauri.conf.json` 中配置托盘图标：

```json
{
  "app": {
    "trayIcon": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  }
}
```

## 第二步：创建托盘模块

创建 `src-tauri/src/tray.rs`：

```rust
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

pub fn create_tray(app: &App) -> Result<tauri::tray::TrayIcon, Box<dyn std::error::Error>> {
    // 创建菜单项
    let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 组合成菜单
    let menu = Menu::with_items(app, &[&show, &quit])?;

    // 构建托盘图标
    let tray = TrayIconBuilder::new()
        .menu(&menu)                    // 设置右键菜单
        .tooltip("Test WebView")        // 悬停提示
        .on_menu_event(|app, event| {   // 菜单事件回调
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => { app.exit(0); }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {  // 图标事件回调
            if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(tray)
}
```

### 关键 API 说明

| API | 说明 |
|-----|------|
| `MenuItem::with_id(app, id, text, enabled, accelerator)` | 创建菜单项，id 用于事件识别 |
| `Menu::with_items(app, &[&item1, &item2])` | 将菜单项组合成菜单 |
| `TrayIconBuilder::new()` | 创建托盘图标建造者 |
| `.menu(&menu)` | 设置右键菜单 |
| `.on_menu_event(closure)` | 菜单点击事件回调 |
| `.on_tray_icon_event(closure)` | 托盘图标事件（单击/双击） |
| `.build(app)` | 构建并返回托盘图标实例 |

## 第三步：在 setup 中初始化

在 `lib.rs` 的 `setup` 闭包中调用：

```rust
mod tray;  // 声明模块

.setup(|app| {
    let _tray = tray::create_tray(app)?;
    Ok(())
})
```

## 第四步：最小化到托盘

当用户点击关闭按钮时，隐藏窗口而不是销毁：

```typescript
// 前端 TitleBar.vue
async function handleClose() {
  await appWindow.hide();  // 隐藏窗口，托盘图标仍在
}
```

```rust
// 后端 lib.rs - on_window_event
"minimize" => {
    api.prevent_close();    // 阻止窗口关闭
    let _ = window.hide();  // 隐藏窗口
}
```

## 常见问题

### Q: 托盘图标不显示？

A: 检查 `tauri.conf.json` 中的 `iconPath` 是否指向了实际存在的图标文件。图标文件放在 `src-tauri/icons/` 目录下。

### Q: macOS 上托盘图标是空白的？

A: macOS 需要使用模板图标（纯黑白），设置 `iconAsTemplate: true`。图标应为 16x16 或 32x32 的 PNG，只使用黑色和透明色。

### Q: 如何让托盘图标在应用退出后也消失？

A: 确保使用 `app.exit(0)` 而不是 `std::process::exit(0)`，前者会正确清理 Tauri 资源。

## 下一步

→ [04 - 原生菜单栏](./04-native-menu.md)
