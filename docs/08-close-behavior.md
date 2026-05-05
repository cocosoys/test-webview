# 08 - 关闭行为

本教程讲解如何拦截窗口关闭事件，实现"最小化到托盘 / 退出 / 询问"三种关闭行为。

## 需求分析

桌面应用常见的关闭行为：

| 行为 | 说明 |
|------|------|
| 最小化到托盘 | 关闭时隐藏窗口，托盘图标仍在，可恢复 |
| 退出程序 | 直接退出应用，释放所有资源 |
| 每次询问 | 弹出对话框让用户选择 |

## 后端实现：拦截关闭事件

在 `lib.rs` 的 `on_window_event` 中：

```rust
.on_window_event(|window, event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
        let app = window.app_handle().clone();
        let close_action = AppConfig::load(&app)
            .map(|c| c.close_action)
            .unwrap_or_else(|_| "ask".to_string());

        match close_action.as_str() {
            "minimize" => {
                api.prevent_close();  // 阻止窗口关闭
                let _ = window.hide(); // 隐藏窗口
            }
            "ask" => {
                api.prevent_close();  // 阻止窗口关闭
                let _ = window.emit("close-requested", ()); // 通知前端
            }
            _ => {}  // "quit" 或其他：允许默认关闭
        }
    }
})
```

### 关键 API

| API | 说明 |
|-----|------|
| `WindowEvent::CloseRequested` | 窗口关闭请求事件（用户点击 × 或 Alt+F4） |
| `api.prevent_close()` | 阻止窗口关闭（必须调用，否则窗口会被销毁） |
| `window.hide()` | 隐藏窗口（不销毁，仍在托盘中） |
| `window.emit("event", payload)` | 向前端发送事件（需要 `use tauri::Emitter;`） |

### ⚠️ 重要：Emitter trait

`emit()` 是 `Emitter` trait 的方法，不是 `AppHandle` 或 `Window` 的固有方法。必须导入 trait：

```rust
use tauri::Emitter;  // 否则 .emit() 编译报错！
```

## 前端实现：关闭确认对话框

### 监听 Rust 事件

```typescript
// TitleBar.vue
import { listen } from "@tauri-apps/api/event";

// 当 Rust 触发 Alt+F4 且 close_action 为 "ask" 时
const unlisten = await listen("close-requested", () => {
  handleClose();  // 弹出确认对话框
});
```

### 关闭对话框组件

```html
<template>
  <Teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center">
      <!-- 遮罩层 -->
      <div @click.self="emit('close')"></div>

      <!-- 对话框 -->
      <div class="w-80 rounded-2xl p-6">
        <h3>关闭确认</h3>
        <p>您希望如何处理？</p>

        <!-- 选项1：最小化到托盘 -->
        <button @click="doAction('minimize')">最小化到托盘</button>

        <!-- 选项2：退出程序 -->
        <button @click="doAction('quit')">退出程序</button>

        <!-- 记住选择 -->
        <label>
          <input type="checkbox" v-model="rememberChoice" />
          记住我的选择
        </label>
      </div>
    </div>
  </Teleport>
</template>
```

### "记住选择" 逻辑

```typescript
function doAction(action: "minimize" | "quit") {
  if (rememberChoice.value) {
    // 保存到设置，下次不再询问
    settingsStore.setCloseAction(action);
  }
  emit("action", action);
}
```

## 完整流程图

```
用户点击 × 按钮 或 Alt+F4
    ↓
┌─ Rust on_window_event ──────────────────┐
│  读取 close_action 配置                   │
│  ├── "minimize" → prevent_close + hide  │
│  ├── "ask"      → prevent_close + emit  │
│  └── "quit"     → 允许关闭              │
└─────────────────────────────────────────┘
    ↓ (如果是 "ask")
┌─ 前端 TitleBar.vue ─────────────────────┐
│  listen("close-requested")               │
│  显示 CloseDialog                        │
│  用户选择:                               │
│  ├── 最小化到托盘 → window.hide()       │
│  └── 退出程序   → window.destroy()      │
│  如果勾选"记住": 保存到 settingsStore   │
└─────────────────────────────────────────┘
```

## 常见问题

### Q: Alt+F4 触发的是什么流程？

A: Alt+F4 和点击 × 按钮触发的是同一个 `CloseRequested` 事件，流程完全一样。

### Q: `window.hide()` 和 `window.destroy()` 的区别？

A: `hide()` 只是隐藏窗口，进程仍在运行，托盘图标可恢复；`destroy()` 销毁窗口并可能退出进程。

### Q: 前端直接调 `window.close()` 行不行？

A: 不行。`window.close()` 会触发 `CloseRequested`，被 Rust 拦截后进入同样的流程。应该用 `appWindow.hide()` 或 `appWindow.destroy()`。

## 下一步

→ [09 - 构建与打包](./09-build-and-bundle.md)
