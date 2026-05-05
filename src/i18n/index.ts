/**
 * i18n/index.ts — 国际化（i18n）配置
 *
 * vue-i18n 是 Vue 的国际化插件，让应用支持多语言。
 *
 * 工作原理：
 * 1. 加载语言包文件（zh-CN.json, en-US.json）
 * 2. 创建 i18n 实例，设置默认语言和回退语言
 * 3. 在组件中通过 t() 函数翻译文本
 *
 * 使用示例（在组件中）：
 * ```html
 * <template>
 *   <h1>{{ t('dashboard.welcome') }}</h1>
 *   <!-- 中文环境下显示："欢迎使用 Test WebView" -->
 *   <!-- 英文环境下显示："Welcome to Test WebView" -->
 * </template>
 *
 * <script setup>
 * import { useI18n } from "vue-i18n";
 * const { t } = useI18n();
 * </script>
 * ```
 */

import { createI18n } from "vue-i18n";
// 导入中文语言包
import zhCN from "./zh-CN.json";
// 导入英文语言包
import enUS from "./en-US.json";

const i18n = createI18n({
  // legacy: false 表示使用 Composition API 模式
  // 推荐使用此模式，配合 useI18n() 组合式函数
  legacy: false,

  // 默认语言：简体中文
  locale: "zh-CN",

  // 回退语言：当当前语言包中找不到某个翻译键时，使用英文
  fallbackLocale: "en-US",

  // 语言包消息：键是语言代码，值是对应的翻译对象
  messages: {
    "zh-CN": zhCN,  // 中文翻译
    "en-US": enUS,  // 英文翻译
  },
});

export default i18n;
