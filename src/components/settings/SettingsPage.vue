<!--
  SettingsPage.vue — 设置页面
  整合所有设置项子组件，按分区组织：
  1. 外观：语言选择 + 主题切换
  2. 行为：关闭窗口行为
  3. 存储：配置文件路径
  4. 关于：应用信息
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import ThemeToggle from "./ThemeToggle.vue";
import LanguageSelect from "./LanguageSelect.vue";
import ConfigPath from "./ConfigPath.vue";
import CloseActionSetting from "./CloseActionSetting.vue";
import AboutPanel from "./AboutPanel.vue";
import { Palette, Folder, XCircle } from "lucide-vue-next";

const { t } = useI18n();
</script>

<template>
  <div class="space-y-8 max-w-2xl">
    <h1 class="text-2xl font-bold" style="color: var(--color-text-primary)">
      {{ t('settings.title') }}
    </h1>

    <!-- 外观设置区 -->
    <section class="space-y-4">
      <div class="flex items-center gap-2">
        <Palette class="w-5 h-5" style="color: var(--color-accent)" />
        <h2 class="text-sm font-semibold uppercase tracking-wider" style="color: var(--color-text-secondary)">
          {{ t('settings.appearance') }}
        </h2>
      </div>
      <div class="space-y-4 rounded-xl p-5" style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)">
        <LanguageSelect />
        <div class="h-px" style="background-color: var(--color-border)"></div>
        <ThemeToggle />
      </div>
    </section>

    <!-- 行为设置区 -->
    <section class="space-y-4">
      <div class="flex items-center gap-2">
        <XCircle class="w-5 h-5" style="color: var(--color-accent)" />
        <h2 class="text-sm font-semibold uppercase tracking-wider" style="color: var(--color-text-secondary)">
          {{ t('settings.behavior') }}
        </h2>
      </div>
      <div class="rounded-xl p-5" style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)">
        <CloseActionSetting />
      </div>
    </section>

    <!-- 存储设置区 -->
    <section class="space-y-4">
      <div class="flex items-center gap-2">
        <Folder class="w-5 h-5" style="color: var(--color-accent)" />
        <h2 class="text-sm font-semibold uppercase tracking-wider" style="color: var(--color-text-secondary)">
          {{ t('settings.storage') }}
        </h2>
      </div>
      <div class="rounded-xl p-5" style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)">
        <ConfigPath />
      </div>
    </section>

    <!-- 关于信息 -->
    <section class="space-y-4">
      <AboutPanel />
    </section>
  </div>
</template>
