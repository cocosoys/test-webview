<!--
  LanguageSelect.vue — 语言选择组件
  提供中文/英文切换按钮。
  切换语言时会同步更新：
  - vue-i18n 的 locale（即时翻译所有 t() 调用）
  - settingsStore 的 locale（持久化设置）
  - HTML 的 lang 属性（辅助浏览器识别语言）
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";

// locale: 当前语言代码，可读写；t: 翻译函数
const { t, locale } = useI18n();
const settingsStore = useSettingsStore();

/** 支持的语言列表 */
const languages = [
  { code: "zh-CN", label: "简体中文" },
  { code: "en-US", label: "English" },
];

/**
 * 切换界面语言
 * @param code - 语言代码（如 "zh-CN" 或 "en-US"）
 */
function changeLocale(code: string) {
  locale.value = code;                    // 更新 vue-i18n 当前语言
  settingsStore.setLocale(code);          // 更新设置状态（用于持久化）
  document.documentElement.lang = code;   // 更新 <html> 标签的 lang 属性
}
</script>

<template>
  <div>
    <label class="text-sm font-medium mb-2 block" style="color: var(--color-text-primary)">
      {{ t('settings.language') }}
    </label>
    <div class="flex gap-2">
      <button
        v-for="lang in languages"
        :key="lang.code"
        class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all"
        :class="locale === lang.code ? 'ring-2' : ''"
        :style="
          locale === lang.code
            ? 'background-color: var(--color-accent); color: white; ring-color: var(--color-accent)'
            : 'background-color: var(--color-bg-tertiary); color: var(--color-text-primary)'
        "
        @click="changeLocale(lang.code)"
      >
        {{ lang.label }}
      </button>
    </div>
  </div>
</template>
