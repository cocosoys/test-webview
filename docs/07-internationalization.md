# 07 - 国际化

本教程讲解如何使用 vue-i18n 实现中英双语切换。

## 第一步：安装 vue-i18n

```bash
npm install vue-i18n@11
```

## 第二步：创建语言包

### 中文 `src/i18n/zh-CN.json`

```json
{
  "app": {
    "name": "Test WebView",
    "dashboard": "仪表盘",
    "settings": "设置",
    "about": "关于"
  },
  "dashboard": {
    "welcome": "欢迎使用 Test WebView",
    "subtitle": "基于 Tauri + Vue 3 的桌面应用技术验证原型"
  },
  "settings": {
    "title": "设置",
    "appearance": "外观",
    "language": "语言",
    "theme": "主题"
  }
}
```

### 英文 `src/i18n/en-US.json`

```json
{
  "app": {
    "name": "Test WebView",
    "dashboard": "Dashboard",
    "settings": "Settings",
    "about": "About"
  },
  "dashboard": {
    "welcome": "Welcome to Test WebView",
    "subtitle": "A desktop app tech demo based on Tauri + Vue 3"
  },
  "settings": {
    "title": "Settings",
    "appearance": "Appearance",
    "language": "Language",
    "theme": "Theme"
  }
}
```

**设计原则：**
- 使用嵌套结构组织翻译键（`app.name`、`dashboard.welcome`）
- 中英文使用相同的键名，只是值不同
- 保持两个语言包的键名完全一致

## 第三步：配置 i18n 实例

创建 `src/i18n/index.ts`：

```typescript
import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.json";
import enUS from "./en-US.json";

const i18n = createI18n({
  legacy: false,           // ✅ 使用 Composition API 模式
  locale: "zh-CN",         // 默认语言
  fallbackLocale: "en-US", // 找不到翻译时回退到英文
  messages: {
    "zh-CN": zhCN,
    "en-US": enUS,
  },
});

export default i18n;
```

**`legacy: false` 的含义：**
- `true`（传统模式）：使用 `this.$t('key')`，仅支持 Options API
- `false`（Composition API 模式）：使用 `const { t } = useI18n()`，更灵活

在 `main.ts` 中注册：

```typescript
app.use(i18n);
```

## 第四步：在组件中使用

### 模板中使用

```html
<template>
  <h1>{{ t('dashboard.welcome') }}</h1>
  <!-- 中文: 欢迎使用 Test WebView -->
  <!-- 英文: Welcome to Test WebView -->
</template>

<script setup>
import { useI18n } from "vue-i18n";
const { t } = useI18n();
</script>
```

### 切换语言

```typescript
const { locale } = useI18n();

function changeLocale(code: string) {
  locale.value = code;                     // 更新 vue-i18n 当前语言
  document.documentElement.lang = code;    // 更新 HTML lang 属性
}
```

切换 `locale.value` 后，所有 `t()` 调用会自动返回对应语言的翻译，UI 实时更新。

## 第五步：持久化语言选择

将语言选择保存到配置文件，应用启动时恢复：

```typescript
// 从后端加载配置
const config = await invoke("get_config");
locale.value = config.locale;  // 恢复上次选择的语言
```

## 常见问题

### Q: 语言包中缺少某个翻译键会怎样？

A: 会显示键名本身（如 `dashboard.welcome`），然后尝试 `fallbackLocale` 中的翻译。

### Q: 如何添加更多语言？

A: 1. 创建新的语言包文件（如 `ja-JP.json`）
   2. 在 `i18n/index.ts` 中导入并添加到 `messages`
   3. 在语言切换 UI 中添加新选项

### Q: 语言包太大怎么办？

A: 可以使用动态导入实现按需加载：

```typescript
const i18n = createI18n({
  legacy: false,
  messages: {},  // 初始为空
});

// 异步加载语言包
async function loadLocale(lang: string) {
  const messages = await import(`./${lang}.json`);
  i18n.global.setLocaleMessage(lang, messages.default);
  i18n.global.locale.value = lang;
}
```

## 下一步

→ [08 - 关闭行为](./08-close-behavior.md)
