// =============================================================================
// config.rs — 应用配置管理：YAML 格式的读写与默认值
// =============================================================================
//
// 这个模块负责应用配置的完整生命周期：
// 1. 定义配置结构体（AppConfig）
// 2. 提供默认值
// 3. 从 YAML 文件读取配置
// 4. 将配置写入 YAML 文件
//
// 关键概念：
// - serde: Rust 的序列化/反序列化框架，支持 JSON、YAML 等多种格式
// - #[derive(Serialize, Deserialize)]: 自动生成序列化/反序列化代码
// - #[serde(default)]: 当 YAML 中缺少该字段时，使用默认值
// - app_data_dir: Tauri 提供的应用数据目录，各平台路径不同：
//   - Windows: C:\Users\{用户}\AppData\Roaming\{identifier}
//   - macOS:   ~/Library/Application Support/{identifier}
//   - Linux:   ~/.config/{identifier}

// serde: 提供 Serialize 和 Deserialize trait，让结构体可以被序列化/反序列化
use serde::{Deserialize, Serialize};
// tauri::Manager: 提供 path() 方法，用于获取应用数据目录等路径
use tauri::Manager;
// std::fs: 文件系统操作（创建目录、读写文件）
use std::fs;
// std::path::PathBuf: 可变的文件路径类型
use std::path::PathBuf;

/// 配置文件格式版本号
///
/// 当配置文件结构发生不兼容变更时，应递增此版本号。
/// 前端可通过 `get_app_version` 命令读取此值。
const CONFIG_VERSION: &str = "1.0";

// =============================================================================
// AppConfig — 应用配置结构体
// =============================================================================
//
// 每个字段对应 YAML 配置文件中的一个键。
// #[serde(default = "...")] 指定当 YAML 中缺少该字段时使用的默认值函数。
// 这样即使配置文件不完整，也能正常加载。

/// 应用配置结构体，对应 config.yaml 文件的内容
///
/// # YAML 示例
/// ```yaml
/// version: "1.0"
/// locale: zh-CN
/// theme: system
/// close_action: ask
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 配置文件格式版本，如 "1.0"
    /// 用于未来配置格式升级时的兼容性判断
    #[serde(default = "default_version")]
    pub version: String,

    /// 界面语言，可选值："zh-CN"（简体中文）、"en-US"（英文）
    #[serde(default = "default_locale")]
    pub locale: String,

    /// 外观主题模式，可选值："light"（亮色）、"dark"（暗色）、"system"（跟随系统）
    #[serde(default = "default_theme")]
    pub theme: String,

    /// 关闭窗口行为，可选值：
    /// - "ask": 每次关闭时弹窗询问
    /// - "minimize": 最小化到系统托盘
    /// - "quit": 直接退出程序
    #[serde(default = "default_close_action")]
    pub close_action: String,
}

// 为 AppConfig 实现 Default trait，提供全默认值实例
// 当 YAML 文件不存在时，会使用这些默认值创建新配置
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            locale: default_locale(),
            theme: default_theme(),
            close_action: default_close_action(),
        }
    }
}

// 各字段的默认值函数
// 被 #[serde(default = "函数名")] 引用

fn default_version() -> String {
    CONFIG_VERSION.to_string()
}

fn default_locale() -> String {
    "zh-CN".to_string()
}

fn default_theme() -> String {
    "system".to_string()
}

fn default_close_action() -> String {
    "ask".to_string()
}

// =============================================================================
// AppConfig 的方法实现
// =============================================================================

impl AppConfig {
    /// 获取应用数据目录，如果不存在则创建
    ///
    /// 各平台的默认路径：
    /// - Windows: `C:\Users\{用户}\AppData\Roaming\com.test-webview.app`
    /// - macOS:   `~/Library/Application Support/com.test-webview.app`
    /// - Linux:   `~/.config/com.test-webview.app`
    fn app_data_dir(app: &tauri::AppHandle) -> PathBuf {
        // app.path().app_data_dir() 返回 Tauri 配置的标准应用数据目录
        let dir = app.path().app_data_dir().expect("failed to resolve app data dir");
        // 确保目录存在，如果不存在则递归创建
        fs::create_dir_all(&dir).ok();
        dir
    }

    /// 获取配置文件的完整路径
    ///
    /// 返回值示例（Windows）：
    /// `C:\Users\Alice\AppData\Roaming\com.test-webview.app\config.yaml`
    pub fn config_file_path(app: &tauri::AppHandle) -> PathBuf {
        Self::app_data_dir(app).join("config.yaml")
    }

    /// 从 YAML 文件加载配置
    ///
    /// # 流程
    /// 1. 检查 config.yaml 是否存在
    /// 2. 如果存在：读取文件内容 → 用 serde_yaml 反序列化为 AppConfig
    /// 3. 如果不存在：创建默认配置 → 保存到文件 → 返回
    ///
    /// # 错误处理
    /// 所有 I/O 和反序列化错误都转换为 String 类型的 Err
    pub fn load(app: &tauri::AppHandle) -> Result<Self, String> {
        let path = Self::config_file_path(app);
        if path.exists() {
            // 读取文件全部内容为字符串
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            // 将 YAML 字符串解析为 AppConfig 结构体
            let config: AppConfig = serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
            Ok(config)
        } else {
            // 首次启动：使用默认值创建配置并保存
            let config = Self::default();
            config.save(app)?;
            Ok(config)
        }
    }

    /// 将当前配置保存到 YAML 文件
    ///
    /// 使用 `serde_yaml::to_string` 将结构体序列化为 YAML 格式字符串，
    /// 然后写入 config.yaml 文件。
    pub fn save(&self, app: &tauri::AppHandle) -> Result<(), String> {
        let path = Self::config_file_path(app);
        // 将 AppConfig 序列化为 YAML 字符串
        let content = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        // 写入文件（如果文件已存在则覆盖）
        fs::write(&path, content).map_err(|e| e.to_string())
    }

    /// 获取配置文件版本号（静态方法，不需要实例）
    ///
    /// 用于前端通过 `get_app_version` 命令获取版本号
    pub fn version() -> &'static str {
        CONFIG_VERSION
    }
}
