// =============================================================================
// lib.rs — 应用核心逻辑：Tauri 命令、插件注册、生命周期管理
// =============================================================================
//
// 这个文件是整个 Tauri 应用的"大脑"，负责：
// 1. 声明子模块（config / menu / tray）
// 2. 定义前端可以调用的"命令"（Tauri Command）
// 3. 注册 Tauri 插件（文件系统、对话框等）
// 4. 处理窗口事件（如关闭窗口时的行为）
//
// 关键概念：
// - `#[tauri::command]` 标记的函数可以被前端通过 invoke() 调用
// - `tauri::Builder` 用于配置应用的插件、命令、事件处理器等
// - `setup` 闭包在应用启动时执行一次，用于初始化
// - `on_window_event` 闭包在每个窗口事件发生时被调用

// 声明三个子模块，对应同名的 .rs 文件
// - config.rs: 配置文件管理（读写 YAML）
// - menu.rs:   原生菜单栏
// - tray.rs:   系统托盘
mod config;
mod menu;
mod tray;

// 从 config 模块导入 AppConfig 结构体，用于读写配置
use config::AppConfig;
// 从 tauri 导入常用的 trait 和类型：
// - Emitter:   提供 emit() 方法，用于向前端发送事件
// - Manager:   提供 get_webview_window() 等方法，用于管理窗口
// - WindowEvent: 窗口事件枚举（如 CloseRequested, Resized 等）
use tauri::{Emitter, Manager, WindowEvent};

// =============================================================================
// Tauri 命令：前端可以通过 @tauri-apps/api 的 invoke() 调用这些函数
// =============================================================================

/// 读取应用配置，返回 JSON 格式
///
/// # 前端调用方式
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
/// const config = await invoke("get_config");
/// // config = { version: "1.0", locale: "zh-CN", theme: "system", close_action: "ask" }
/// ```
///
/// # 参数
/// - `app`: Tauri 自动注入的 AppHandle，代表应用实例
///
/// # 返回
/// - `Ok`: 配置的 JSON 值（serde_json::Value）
/// - `Err`: 错误信息字符串
#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    // 从 YAML 文件加载配置
    let config = AppConfig::load(&app)?;
    // 将 Rust 结构体转换为 JSON 值，方便前端使用
    // 前端只能理解 JSON，所以需要 serde_json::to_value 做转换
    let json_val = serde_json::to_value(&config).map_err(|e| e.to_string())?;
    Ok(json_val)
}

/// 保存应用配置（前端传入 JSON，后端写入 YAML 文件）
///
/// # 前端调用方式
/// ```typescript
/// await invoke("save_config", {
///   config: { version: "1.0", locale: "en-US", theme: "dark", close_action: "ask" }
/// });
/// ```
///
/// # 流程
/// 1. 前端传入 JSON 对象
/// 2. Rust 将 JSON 反序列化为 AppConfig 结构体
/// 3. 将 AppConfig 序列化为 YAML 并写入文件
#[tauri::command]
fn save_config(app: tauri::AppHandle, config: serde_json::Value) -> Result<(), String> {
    // 将前端传来的 JSON 值转换为 Rust 的 AppConfig 结构体
    let app_config: AppConfig =
        serde_json::from_value(config).map_err(|e| e.to_string())?;
    // 保存到 YAML 文件
    app_config.save(&app)?;
    Ok(())
}

/// 获取系统信息（操作系统、架构、平台族）
///
/// # 前端调用方式
/// ```typescript
/// const info = await invoke("get_system_info");
/// // info = { os: "windows", arch: "x86_64", family: "unix" }
/// ```
///
/// # 说明
/// 使用 Rust 标准库的 `std::env::consts` 获取编译时确定的系统信息
#[tauri::command]
fn get_system_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "os": std::env::consts::OS,       // 操作系统名称，如 "windows", "macos", "linux"
        "arch": std::env::consts::ARCH,   // CPU 架构，如 "x86_64", "aarch64"
        "family": std::env::consts::FAMILY, // 平台族，如 "windows", "unix"
    }))
}

/// 获取配置文件版本号
///
/// # 前端调用方式
/// ```typescript
/// const version = await invoke("get_app_version");
/// // version = "1.0"
/// ```
#[tauri::command]
fn get_app_version() -> String {
    AppConfig::version().to_string()
}

// =============================================================================
// 应用主函数：配置并启动 Tauri 应用
// =============================================================================

/// 应用的主入口函数，由 main.rs 调用
///
/// 这个函数通过 `tauri::Builder` 构建 Tauri 应用，配置顺序：
/// 1. 注册插件（shell, dialog, fs, process）
/// 2. setup 闭包：初始化托盘、菜单、配置文件
/// 3. 窗口事件处理器：拦截窗口关闭事件
/// 4. 注册命令处理器：让前端可以调用上面定义的函数
/// 5. 启动应用
pub fn run() {
    tauri::Builder::default()
        // -------------------------------------------------------------------------
        // 注册 Tauri 官方插件
        // 这些插件提供了与操作系统交互的能力
        // -------------------------------------------------------------------------
        .plugin(tauri_plugin_shell::init())   // Shell 操作：打开外部链接、执行命令
        .plugin(tauri_plugin_dialog::init())  // 对话框：文件选择、消息提示等
        .plugin(tauri_plugin_fs::init())      // 文件系统：读写文件、创建目录等
        .plugin(tauri_plugin_process::init()) // 进程管理：退出应用、重启等

        // -------------------------------------------------------------------------
        // setup 闭包：应用启动后执行一次的初始化逻辑
        // -------------------------------------------------------------------------
        .setup(|app| {
            // 创建系统托盘图标和右键菜单
            let _tray = tray::create_tray(app)?;

            // 创建原生菜单栏（文件/视图/帮助）
            let menu = menu::create_menu(app)?;
            let _ = app.set_menu(menu);

            // 首次启动时，确保默认配置文件存在
            // 如果 config.yaml 不存在，会自动创建一个包含默认值的配置文件
            let _ = AppConfig::load(&app.handle().clone());

            Ok(())
        })

        // -------------------------------------------------------------------------
        // 窗口事件处理器
        // 当窗口发生事件（关闭请求、调整大小等）时被调用
        // -------------------------------------------------------------------------
        .on_window_event(|window, event| {
            // 只处理"关闭请求"事件（用户点击了窗口的 × 按钮，或按了 Alt+F4）
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 获取 AppHandle 的克隆，用于读取配置
                let app = window.app_handle().clone();
                // 从配置文件读取关闭行为设置
                let close_action = AppConfig::load(&app)
                    .map(|c| c.close_action)
                    .unwrap_or_else(|_| "ask".to_string());

                match close_action.as_str() {
                    // "minimize" 模式：阻止关闭，隐藏窗口到托盘
                    "minimize" => {
                        api.prevent_close();  // 阻止窗口关闭
                        let _ = window.hide(); // 隐藏窗口（不销毁）
                    }
                    // "ask" 模式：阻止关闭，向前端发送事件，让前端弹出确认对话框
                    "ask" => {
                        api.prevent_close();  // 阻止窗口关闭
                        let _ = window.emit("close-requested", ()); // 向前端发送事件
                    }
                    // "quit" 模式或其他：允许默认行为，直接关闭窗口退出程序
                    _ => {}
                }
            }
        })

        // -------------------------------------------------------------------------
        // 注册命令处理器
        // 只有在这里注册的命令，前端才能通过 invoke() 调用
        // -------------------------------------------------------------------------
        .invoke_handler(tauri::generate_handler![
            get_config,       // 读取配置
            save_config,      // 保存配置
            get_system_info,  // 获取系统信息
            get_app_version,  // 获取配置版本号
        ])

        // -------------------------------------------------------------------------
        // 启动应用
        // tauri::generate_context!() 宏会读取 tauri.conf.json 配置
        // -------------------------------------------------------------------------
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
