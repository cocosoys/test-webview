// =============================================================================
// menu.rs — 原生菜单栏：文件、视图、帮助菜单及事件处理
// =============================================================================
//
// 原生菜单栏是桌面应用顶部的菜单条（如 文件 | 视图 | 帮助）。
// Tauri 提供了跨平台的菜单 API，在不同操作系统上自动适配原生风格。
//
// 关键概念：
// - MenuItem: 普通菜单项（可点击）
// - Submenu:  子菜单（可展开，包含子菜单项）
// - Menu:     顶级菜单栏（包含多个子菜单）
// - on_menu_event: 菜单事件回调，根据菜单项 ID 分发处理
// - Emitter trait: 提供 emit() 方法，用于向前端发送事件
//
// 本模块的菜单结构：
// ```
// 文件(F)    视图(V)    帮助(H)
// ├── 设置...  ├── 切换全屏  ├── 关于
// └── 退出     └──────────┘  └─────┘
// ```

use tauri::{
    menu::{Menu, MenuItem, Submenu},  // 菜单相关类型
    App, Emitter, Manager,             // App: 应用实例, Emitter: 发送事件, Manager: 管理窗口
};

/// 创建应用菜单栏
///
/// # 返回值
/// - `Ok(Menu)`: 成功创建的菜单
/// - `Err`: 创建失败时的错误信息
///
/// # 菜单项功能
/// | 菜单路径       | 功能                                    |
/// |----------------|----------------------------------------|
/// | 文件 → 设置    | 发送 `navigate` 事件，前端跳转到 /settings |
/// | 文件 → 退出    | 完全退出应用                              |
/// | 视图 → 切换全屏 | 切换窗口全屏/还原                         |
/// | 帮助 → 关于    | 发送 `navigate` 事件，前端跳转到 /about    |
pub fn create_menu(app: &App) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    // =========================================================================
    // 创建"文件"子菜单
    // =========================================================================
    let settings = MenuItem::with_id(app, "settings", "设置...", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    // Submenu::with_items 参数：app, 菜单标签文本, 是否启用, 子菜单项数组
    let file_menu = Submenu::with_items(app, "文件", true, &[&settings, &quit])?;

    // =========================================================================
    // 创建"视图"子菜单
    // =========================================================================
    let fullscreen =
        MenuItem::with_id(app, "fullscreen", "切换全屏", true, None::<&str>)?;
    let view_menu = Submenu::with_items(app, "视图", true, &[&fullscreen])?;

    // =========================================================================
    // 创建"帮助"子菜单
    // =========================================================================
    let about = MenuItem::with_id(app, "about", "关于", true, None::<&str>)?;
    let help_menu = Submenu::with_items(app, "帮助", true, &[&about])?;

    // =========================================================================
    // 组合为顶级菜单栏
    // =========================================================================
    let menu = Menu::with_items(app, &[&file_menu, &view_menu, &help_menu])?;

    // -------------------------------------------------------------------------
    // 注册菜单事件回调
    // -------------------------------------------------------------------------
    // 注意：需要克隆 app.handle()，因为闭包需要 'static 生命周期
    // handle 用于在回调中向前端发送事件
    let handle = app.handle().clone();

    // on_menu_event 注册一个全局菜单事件处理器
    // 当任何菜单项被点击时，这个闭包都会被调用
    app.on_menu_event(move |app, event| match event.id.as_ref() {
        // 点击"设置"：通过事件机制让前端导航到设置页面
        "settings" => {
            // emit() 向所有前端窗口广播 "navigate" 事件
            // 前端在 main.ts 中监听此事件，收到后执行 router.push("/settings")
            let _ = handle.emit("navigate", "/settings");
        }
        // 点击"关于"：导航到关于页面
        "about" => {
            let _ = handle.emit("navigate", "/about");
        }
        // 点击"退出"：完全退出应用
        "quit" => {
            app.exit(0);
        }
        // 点击"切换全屏"：切换主窗口的全屏状态
        "fullscreen" => {
            if let Some(window) = app.get_webview_window("main") {
                // 先获取当前是否全屏
                let is_fullscreen = window.is_fullscreen().unwrap_or(false);
                // 切换到相反状态
                let _ = window.set_fullscreen(!is_fullscreen);
            }
        }
        _ => {}
    });

    Ok(menu)
}
