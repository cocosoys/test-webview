# 02 - 自定义标题栏

本教程讲解如何实现无边框窗口的自定义标题栏，支持拖拽移动和窗口控制按钮。

## 为什么需要自定义标题栏？

操作系统原生标题栏样式固定，无法自定义颜色和按钮。通过设置 `decorations: false` 移除原生标题栏，然后自己实现一个。

## 第一步：禁用原生标题栏

在 `src-tauri/tauri.conf.json` 中：

```json
{
  "app": {
    "windows": [
      {
        "title": "Test WebView",
        "decorations": false,
        "width": 1024,
        "height": 680,
        "minWidth": 720,
        "minHeight": 480,
        "center": true
      }
    ]
  }
}
```

**关键配置：**
- `decorations: false`：隐藏原生标题栏和边框
- `center: true`：窗口启动时居中显示
- `minWidth/minHeight`：最小窗口尺寸，防止内容挤压

## 第二步：创建标题栏组件

创建 `src/components/layout/TitleBar.vue`：

### 核心概念：data-tauri-drag-region

Tauri 提供了一个特殊 HTML 属性 `data-tauri-drag-region`，标记在元素上后，用户拖拽该元素可以移动窗口。

**重要规则：**
- ✅ 在标题栏空白区域添加此属性
- ❌ 不要在按钮上添加此属性（否则按钮无法点击）
- 💡 使用 `pointer-events-none` 在图标上，防止 SVG 拦截点击事件

### 三段式布局

```
┌──────────┬──────────────────┬────────────────┐
│ 左:图标+名 │ 中:空白(可拖拽)    │ 右:按钮(不可拖拽) │
│ (可拖拽)   │                  │                │
└──────────┴──────────────────┴────────────────┘
```

```html
<div class="flex items-center justify-between h-10 px-3">
  <!-- 左侧：图标+名称，可拖拽 -->
  <div class="flex items-center gap-2.5 flex-1 h-full" data-tauri-drag-region>
    <span>App Name</span>
  </div>

  <!-- 中间：空白，可拖拽 -->
  <div class="flex-1 h-full" data-tauri-drag-region></div>

  <!-- 右侧：按钮，不可拖拽 -->
  <div class="flex items-center flex-shrink-0 h-full">
    <button @click="minimize">─</button>
    <button @click="toggleMaximize">□</button>
    <button @click="handleClose">✕</button>
  </div>
</div>
```

## 第三步：窗口控制 API

使用 Tauri 的窗口 API 控制窗口行为：

```typescript
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

// 最小化
async function minimize() {
  await appWindow.minimize();
}

// 切换最大化/还原
async function toggleMaximize() {
  await appWindow.toggleMaximize();
}

// 隐藏窗口（最小化到托盘）
async function hide() {
  await appWindow.hide();
}

// 销毁窗口（退出程序）
async function destroy() {
  await appWindow.destroy();
}
```

## 第四步：监听窗口状态

需要跟踪窗口是否最大化，以便切换图标：

```typescript
import { ref, onMounted, onUnmounted } from "vue";

const isMaximized = ref(false);

onMounted(async () => {
  // 监听窗口大小变化
  const unlisten = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized();
  });

  // 初始化状态
  isMaximized.value = await appWindow.isMaximized();
});
```

## 第五步：按钮样式

使用 scoped CSS 实现原生风格的按钮效果：

```css
/* 最小化/最大化按钮 */
.win-btn:hover {
  background-color: var(--color-bg-tertiary);
}
.win-btn:active {
  transform: scale(0.92); /* 按下缩小效果 */
}

/* 关闭按钮：悬停变红 */
.win-btn-close:hover {
  background-color: #ef4444;
  color: #ffffff;
}
```

## 常见问题

### Q: 按钮点击无响应？

A: 确保按钮的 SVG 图标添加了 `pointer-events-none`，否则点击被图标拦截：

```html
<button @click="minimize">
  <Minus class="w-4 h-4 pointer-events-none" />  ← 关键！
</button>
```

### Q: 拖拽区域挡住了按钮点击？

A: Tauri 的 `isDragRegion()` 会检查 `composedPath()`，遇到 `<button>` 等可点击元素时自动返回 `false`。所以只需确保按钮不在 `data-tauri-drag-region` 的元素上即可。

### Q: 如何让窗口可以通过边缘调整大小？

A: Tauri 在 `decorations: false` 时仍支持边缘拖拽调整大小，这是默认行为。

## 下一步

→ [03 - 系统托盘](./03-system-tray.md)
