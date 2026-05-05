# 06 - 主题切换

本教程讲解如何实现亮色/暗色/跟随系统的三档主题切换。

## 整体方案

```
用户选择主题模式
    ↓
Pinia store 更新 theme 值
    ↓
applyTheme() 在 <html> 上添加/移除 "dark" 类名
    ↓
CSS 变量在 :root / .dark 中分别定义
    ↓
所有组件通过 var(--color-xxx) 自动适配
```

## 第一步：定义 CSS 变量

在 `src/styles/main.css` 中：

```css
/* 亮色主题（默认） */
:root {
  --color-bg-primary: #ffffff;
  --color-bg-secondary: #f3f4f6;
  --color-text-primary: #111827;
  --color-accent: #3b82f6;
  /* ... 更多变量 */
}

/* 暗色主题 */
.dark {
  --color-bg-primary: #0f172a;
  --color-bg-secondary: #1e293b;
  --color-text-primary: #f1f5f9;
  --color-accent: #60a5fa;
  /* ... 更多变量 */
}
```

**设计原则：**
- 所有颜色都通过 CSS 变量引用，不直接写色值
- 亮色和暗色使用相同的变量名，只是值不同
- 组件只需写 `style="color: var(--color-text-primary)"`，自动适配两种主题

## 第二步：配置 TailwindCSS dark 模式

```css
@custom-variant dark (&:where(.dark, .dark *));
```

这让 Tailwind 的 `dark:` 前缀生效。不过在本项目中，我们主要使用 CSS 变量而非 Tailwind 的 `dark:` 前缀。

## 第三步：创建主题 Store

在 `src/stores/app.ts` 中：

```typescript
import { defineStore } from "pinia";
import { ref } from "vue";

export type ThemeMode = "light" | "dark" | "system";

export const useAppStore = defineStore("app", () => {
  const theme = ref<ThemeMode>("system");

  function setTheme(mode: ThemeMode) {
    theme.value = mode;
    applyTheme(mode);
  }

  function applyTheme(mode: ThemeMode) {
    const html = document.documentElement;
    if (mode === "system") {
      // 检测操作系统的暗色模式设置
      const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      html.classList.toggle("dark", isDark);
    } else {
      html.classList.toggle("dark", mode === "dark");
    }
  }

  // 监听系统主题变化
  window.matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", () => {
      if (theme.value === "system") {
        applyTheme("system");
      }
    });

  // 初始化
  applyTheme(theme.value);

  return { theme, setTheme, applyTheme };
});
```

**关键点：**
- `window.matchMedia("(prefers-color-scheme: dark)")` 检测系统暗色模式
- `html.classList.toggle("dark", boolean)` 添加或移除 `dark` 类
- 监听 `change` 事件让"跟随系统"模式实时响应系统设置变化

## 第四步：创建主题切换 UI

```html
<template>
  <div class="flex gap-2">
    <button
      v-for="theme in themes"
      :key="theme.mode"
      :class="appStore.theme === theme.mode ? 'ring-2' : ''"
      :style="appStore.theme === theme.mode
        ? 'background-color: var(--color-accent); color: white'
        : 'background-color: var(--color-bg-tertiary)'"
      @click="appStore.setTheme(theme.mode)"
    >
      <component :is="theme.icon" class="w-4 h-4" />
      {{ t(theme.labelKey) }}
    </button>
  </div>
</template>
```

## 常见问题

### Q: 为什么用 CSS 变量而不是 Tailwind 的 `dark:` 前缀？

A: CSS 变量更灵活，可以在 JS 中动态读取，且不依赖 Tailwind 的类名生成。但两者可以混用。

### Q: "跟随系统"模式切换系统主题后没有立即生效？

A: 确保监听了 `matchMedia` 的 `change` 事件。有些浏览器可能延迟触发。

### Q: 刷新页面后主题重置了？

A: 需要将主题选择保存到配置文件（YAML），页面加载时从配置恢复。详见 [配置管理教程](./05-config-management.md)。

## 下一步

→ [07 - 国际化](./07-internationalization.md)
