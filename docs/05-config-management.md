# 05 - 配置管理

本教程讲解如何实现 YAML 格式的配置文件管理，包括读写、默认值和前后端通信。

## 为什么选择 YAML？

- 比 JSON 更易读（不需要大括号和逗号）
- 支持注释（JSON 不支持）
- 结构清晰，适合手动编辑

## 配置结构设计

```yaml
# config.yaml
version: "1.0"     # 配置文件格式版本
locale: zh-CN       # 界面语言
theme: system       # 主题模式
close_action: ask   # 关闭窗口行为
```

## 后端实现

### 定义配置结构体

创建 `src-tauri/src/config.rs`：

```rust
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fs;
use std::path::PathBuf;

const CONFIG_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_version")]  // YAML 缺少此字段时使用默认值
    pub version: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_close_action")]
    pub close_action: String,
}
```

**关键概念：`#[serde(default)]`**

当 YAML 文件中缺少某个字段时，serde 会调用 `default_xxx()` 函数提供默认值。这样即使用户手动删除了某个字段，应用也不会崩溃。

### 配置文件路径

```rust
impl AppConfig {
    fn app_data_dir(app: &tauri::AppHandle) -> PathBuf {
        let dir = app.path().app_data_dir()
            .expect("failed to resolve app data dir");
        fs::create_dir_all(&dir).ok();  // 确保目录存在
        dir
    }

    pub fn config_file_path(app: &tauri::AppHandle) -> PathBuf {
        Self::app_data_dir(app).join("config.yaml")
    }
}
```

各平台路径：
| 平台 | 路径 |
|------|------|
| Windows | `C:\Users\{用户}\AppData\Roaming\com.test-webview.app\` |
| macOS | `~/Library/Application Support/com.test-webview.app/` |
| Linux | `~/.config/com.test-webview.app/` |

### 读取配置

```rust
pub fn load(app: &tauri::AppHandle) -> Result<Self, String> {
    let path = Self::config_file_path(app);
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let config: AppConfig = serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
        Ok(config)
    } else {
        // 首次启动：创建默认配置
        let config = Self::default();
        config.save(app)?;
        Ok(config)
    }
}
```

### 保存配置

```rust
pub fn save(&self, app: &tauri::AppHandle) -> Result<(), String> {
    let path = Self::config_file_path(app);
    let content = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}
```

## 前端通信

### 定义 Tauri 命令

在 `lib.rs` 中：

```rust
#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let config = AppConfig::load(&app)?;
    serde_json::to_value(&config).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: serde_json::Value) -> Result<(), String> {
    let app_config: AppConfig = serde_json::from_value(config).map_err(|e| e.to_string())?;
    app_config.save(&app)
}
```

**注意：** 前端只能传 JSON，后端需要做 JSON → Rust 结构体 → YAML 的转换。

### 前端调用

```typescript
import { invoke } from "@tauri-apps/api/core";

// 读取配置
const config = await invoke("get_config");
// config = { version: "1.0", locale: "zh-CN", theme: "system", close_action: "ask" }

// 保存配置
await invoke("save_config", {
  config: { version: "1.0", locale: "en-US", theme: "dark", close_action: "quit" }
});
```

**重要：** Rust 的 serde 默认使用 snake_case 序列化字段名（如 `close_action`），前端也需要使用 snake_case。

## 常见问题

### Q: 为什么前端不用 YAML，而用 JSON？

A: Tauri 的 `invoke` 通信层只支持 JSON 序列化。YAML 只用于文件存储格式。

### Q: 配置修改后需要重启应用吗？

A: 不需要。前端直接修改 Pinia store 的值即可立即生效，同时调用 `save_config` 持久化。

### Q: 如何处理配置版本升级？

A: 读取配置时检查 `version` 字段，根据版本号执行迁移逻辑，然后写回新版本。

## 下一步

→ [06 - 主题切换](./06-theme-switching.md)
