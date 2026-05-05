<!--
  ThemeToggle.vue — 主题切换组件
  提供亮色/暗色/跟随系统三种主题模式选择按钮。
  选中项以蓝色高亮显示，未选中项为灰色。
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useAppStore, type ThemeMode } from "@/stores/app";
import { Sun, Moon, Monitor } from "lucide-vue-next";

const { t } = useI18n();
const appStore = useAppStore();

/** 三种主题模式选项 */
const themes: { mode: ThemeMode; icon: typeof Sun; labelKey: string }[] = [
  { mode: "light", icon: Sun, labelKey: "settings.themeLight" },
  { mode: "dark", icon: Moon, labelKey: "settings.themeDark" },
  { mode: "system", icon: Monitor, labelKey: "settings.themeSystem" },
];
</script>

<template>
  <div>
    <label class="text-sm font-medium mb-2 block" style="color: var(--color-text-primary)">
      {{ t('settings.theme') }}
    </label>
    <div class="flex gap-2">
      <button
        v-for="theme in themes"
        :key="theme.mode"
        class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all"
        :class="appStore.theme === theme.mode ? 'ring-2' : ''"
        :style="
          appStore.theme === theme.mode
            ? 'background-color: var(--color-accent); color: white; ring-color: var(--color-accent)'
            : 'background-color: var(--color-bg-tertiary); color: var(--color-text-primary)'
        "
        @click="appStore.setTheme(theme.mode)"
      >
        <component :is="theme.icon" class="w-4 h-4" />
        {{ t(theme.labelKey) }}
      </button>
    </div>
  </div>
</template>
