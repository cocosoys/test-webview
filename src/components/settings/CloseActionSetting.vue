<!--
  CloseActionSetting.vue — 关闭窗口行为设置
  提供三种关闭行为选择：
  - 每次询问：关闭时弹窗让用户选择
  - 最小化到托盘：关闭时隐藏到系统托盘
  - 退出程序：关闭时直接退出应用
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore, type CloseAction } from "@/stores/settings";
import { Minus, Power, MessageCircleQuestion } from "lucide-vue-next";

const { t } = useI18n();
const settingsStore = useSettingsStore();

/** 关闭行为选项列表 */
const options: { value: CloseAction; icon: typeof MessageCircleQuestion; labelKey: string; descKey: string }[] = [
  {
    value: "ask",
    icon: MessageCircleQuestion,
    labelKey: "settings.closeActionAsk",
    descKey: "settings.closeActionAskDesc",
  },
  {
    value: "minimize",
    icon: Minus,
    labelKey: "settings.closeActionMinimize",
    descKey: "settings.closeActionMinimizeDesc",
  },
  {
    value: "quit",
    icon: Power,
    labelKey: "settings.closeActionQuit",
    descKey: "settings.closeActionQuitDesc",
  },
];
</script>

<template>
  <div>
    <label class="text-sm font-medium mb-1 block" style="color: var(--color-text-primary)">
      {{ t('settings.closeAction') }}
    </label>
    <p class="text-xs mb-3" style="color: var(--color-text-secondary)">
      {{ t('settings.closeActionDesc') }}
    </p>
    <div class="space-y-2">
      <button
        v-for="opt in options"
        :key="opt.value"
        class="w-full flex items-center gap-3 px-4 py-2.5 rounded-xl text-sm transition-all duration-150"
        :class="settingsStore.closeAction === opt.value ? 'ring-2' : ''"
        :style="
          settingsStore.closeAction === opt.value
            ? 'background-color: var(--color-accent); color: white; ring-color: var(--color-accent)'
            : 'background-color: var(--color-bg-tertiary); color: var(--color-text-primary); border: 1px solid var(--color-border)'
        "
        @click="settingsStore.setCloseAction(opt.value)"
      >
        <component :is="opt.icon" class="w-4 h-4 shrink-0" />
        <div class="text-left">
          <span class="font-medium">{{ t(opt.labelKey) }}</span>
          <!-- 未选中的选项显示描述文字 -->
          <span
            v-if="settingsStore.closeAction !== opt.value"
            class="ml-2 text-xs"
            style="color: var(--color-text-secondary)"
          >
            {{ t(opt.descKey) }}
          </span>
        </div>
      </button>
    </div>
  </div>
</template>
