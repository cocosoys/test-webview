# 04 - 原生菜单栏

本教程讲解如何创建跨平台的原生菜单栏，并通过事件机制驱动前端路由。

## 什么是原生菜单？

原生菜单是桌面应用顶部的菜单条，如：

```
文件(F)    视图(V)    帮助(H)
├── 设置    ├── 切换全屏  ├── 关于
└── 退出    └──────────┘  └─────┘
```

在不同操作系统上，Tauri 自动适配原生风格（Windows 使用 Win32 菜单，macOS 使用 Cocoa 菜单栏）。

## 创建菜单模块

创建 `src-tauri/src/menu.rs`：

```rust
use tauri::{
    menu::{Menu, MenuItem, Submenu},
    App, Emitter, Manager,
};

pub fn create_menu(app: &App) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    // 创建菜单项
    let settings = MenuItem::with_id(app, "settings", "设置...", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 创建子菜单
    let file_menu = Submenu::with_items(app, "文件", true, &[&settings, &quit])?;

    // 组合为顶级菜单栏
    let menu = Menu::with_items(app, &[&file_menu, &view_menu, &help_menu])?;

    // 注册事件处理器
    let handle = app.handle().clone();  // 需要克隆，因为闭包需要 'static 生命周期
    app.on_menu_event(move |app, event| {
        match event.id.as_ref() {
            "settings" => {
                // 向前端发送导航事件
                let _ = handle.emit("navigate", "/settings");
            }
            "quit" => { app.exit(0); }
            _ => {}
        }
    });

    Ok(menu)
}
```

## 菜单 → 前端路由的桥接机制

原生菜单是 Rust 创建的，无法直接调用 Vue Router。我们使用 Tauri 事件机制作为桥梁：

```
用户点击菜单
    ↓
Rust: on_menu_event 回调
    ↓
handle.emit("navigate", "/settings")  ← 发送事件
    ↓
前端: listen("navigate", callback)    ← 监听事件
    ↓
router.push(event.payload)            ← 执行路由跳转
```

### 前端监听（main.ts）

```typescript
import { listen } from "@tauri-apps/api/event";
import router from "./router";

listen<string>("navigate", (event) => {
  if (event.payload) {
    router.push(event.payload);
  }
});
```

## 关键 API

| API | 说明 |
|-----|------|
| `MenuItem::with_id()` | 普通菜单项 |
| `Submenu::with_items()` | 子菜单（可展开） |
| `Menu::with_items()` | 顶级菜单栏 |
| `app.set_menu(menu)` | 将菜单设置到应用 |
| `app.on_menu_event(closure)` | 全局菜单事件处理器 |
| `handle.emit("event", payload)` | 向前端发送事件 |

## 常见问题

### Q: macOS 上菜单栏在哪里？

A: macOS 的菜单栏显示在屏幕顶部（不是窗口顶部），这是 macOS 的原生行为。

### Q: 为什么 `on_menu_event` 闭包需要 `handle.clone()`？

A: Rust 闭包捕获变量时，如果闭包的生命周期比当前函数长（`'static`），就需要拥有所有权。`app.handle().clone()` 创建一个独立的 AppHandle 副本。

### Q: 如何添加快捷键？

A: `MenuItem::with_id` 的第 5 个参数接受快捷键字符串：

```rust
let quit = MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrControl+Q"))?;
```

## 下一步

→ [05 - 配置管理](./05-config-management.md)
