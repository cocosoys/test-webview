// =============================================================================
// main.rs — 应用程序入口
// =============================================================================
//
// 这是整个 Tauri 应用的 Rust 入口点。
// `fn main()` 是程序启动时第一个被执行的函数。
//
// 关键概念：
// - `#![cfg_attr(...)]` 是 Rust 的条件编译属性
// - `windows_subsystem = "windows"` 告诉 Windows 不要显示控制台窗口
// - 在 debug 模式下保留控制台（方便看日志），release 模式下隐藏
// - `test_webview_lib::run()` 调用 lib.rs 中定义的应用主逻辑

// 当不是 debug 模式（即 release 构建）时，设置 Windows 子系统为 "windows"
// 这样程序启动时就不会弹出黑色的命令行窗口
// debug 模式下保留控制台窗口，方便查看 println! 等日志输出
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 调用 lib.rs 中定义的 run() 函数，启动 Tauri 应用
    // 所有应用逻辑都在 lib.rs 中，main.rs 只是一个入口壳
    test_webview_lib::run()
}
