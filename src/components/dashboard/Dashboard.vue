<!--
  Dashboard.vue — 仪表盘页面
  应用启动后的默认首页，展示系统信息、运行时信息和快速操作入口。

  数据说明：
  - 系统信息通过浏览器 navigator API 获取（在 WebView 环境中可用）
  - 运行时信息为硬编码示例值
  - 快速操作提供常用的快捷入口（打开设置、查看关于、切换主题）
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { useAppStore } from "@/stores/app";
import { appEnvironment } from "@/config/environment";
import { Monitor, Cpu, Globe, Settings, Info, Sun } from "lucide-vue-next";

const { t } = useI18n();
const router = useRouter();
const appStore = useAppStore();

/** 系统信息卡片数据：通过浏览器 API 获取 */
const systemCards = [
  { icon: Monitor, labelKey: "dashboard.stats.platform", value: navigator.platform || "Unknown" },
  { icon: Cpu, labelKey: "dashboard.stats.architecture", value: navigator.userAgent.includes("Win64") ? "x64" : "Unknown" },
  { icon: Globe, labelKey: "dashboard.stats.webview", value: "WebView2" },
];

/** 运行时信息卡片数据：展示技术栈版本 */
const runtimeCards = [
  { icon: Cpu, labelKey: "dashboard.stats.rustVersion", value: "1.85.0" },
  { icon: Globe, labelKey: "dashboard.stats.frontendFramework", value: "Vue 3 + TypeScript" },
  { icon: Settings, labelKey: "dashboard.stats.environment", value: appEnvironment.displayName },
  { icon: Globe, labelKey: "dashboard.stats.apiBaseUrl", value: appEnvironment.services.apiBaseUrl },
];

/** 快速操作按钮 */
const quickActions = [
  { icon: Settings, labelKey: "dashboard.stats.openSettings", action: () => router.push("/settings") },
  { icon: Info, labelKey: "dashboard.stats.viewAbout", action: () => router.push("/about") },
  // 主题循环切换：light → dark → system → light ...
  { icon: Sun, labelKey: "dashboard.stats.toggleTheme", action: () => {
    const modes = ["light", "dark", "system"] as const;
    const currentIdx = modes.indexOf(appStore.theme);
    appStore.setTheme(modes[(currentIdx + 1) % modes.length]);
  }},
];
</script>

<template>
  <div class="space-y-6">
    <!-- 欢迎标题 -->
    <div>
      <h1 class="text-2xl font-bold" style="color: var(--color-text-primary)">
        {{ t('dashboard.welcome') }}
      </h1>
      <p class="mt-1 text-sm" style="color: var(--color-text-secondary)">
        {{ t('dashboard.subtitle') }}
      </p>
    </div>

    <!-- 系统信息卡片 -->
    <div>
      <h2 class="text-sm font-semibold mb-3 uppercase tracking-wider" style="color: var(--color-text-secondary)">
        {{ t('dashboard.stats.systemInfo') }}
      </h2>
      <!-- 响应式网格：小屏1列，中屏3列 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div
          v-for="card in systemCards"
          :key="card.labelKey"
          class="rounded-xl p-4 transition-shadow hover:shadow-md"
          style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)"
        >
          <div class="flex items-center gap-3 mb-2">
            <div class="p-2 rounded-lg" style="background-color: var(--color-accent); opacity: 0.15">
              <component :is="card.icon" class="w-5 h-5" style="color: var(--color-accent)" />
            </div>
            <span class="text-xs font-medium" style="color: var(--color-text-secondary)">
              {{ t(card.labelKey) }}
            </span>
          </div>
          <p class="text-lg font-semibold" style="color: var(--color-text-primary)">{{ card.value }}</p>
        </div>
      </div>
    </div>

    <!-- 运行时信息卡片 -->
    <div>
      <h2 class="text-sm font-semibold mb-3 uppercase tracking-wider" style="color: var(--color-text-secondary)">
        {{ t('dashboard.stats.runtimeInfo') }}
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          v-for="card in runtimeCards"
          :key="card.labelKey"
          class="rounded-xl p-4 transition-shadow hover:shadow-md"
          style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)"
        >
          <div class="flex items-center gap-3 mb-2">
            <div class="p-2 rounded-lg" style="background-color: var(--color-accent); opacity: 0.15">
              <component :is="card.icon" class="w-5 h-5" style="color: var(--color-accent)" />
            </div>
            <span class="text-xs font-medium" style="color: var(--color-text-secondary)">
              {{ t(card.labelKey) }}
            </span>
          </div>
          <p class="text-lg font-semibold" style="color: var(--color-text-primary)">{{ card.value }}</p>
        </div>
      </div>
    </div>

    <!-- 快速操作按钮 -->
    <div>
      <h2 class="text-sm font-semibold mb-3 uppercase tracking-wider" style="color: var(--color-text-secondary)">
        {{ t('dashboard.stats.quickActions') }}
      </h2>
      <div class="flex flex-wrap gap-3">
        <button
          v-for="action in quickActions"
          :key="action.labelKey"
          class="flex items-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-colors"
          style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border); color: var(--color-text-primary)"
          @click="action.action()"
        >
          <component :is="action.icon" class="w-4 h-4" />
          {{ t(action.labelKey) }}
        </button>
      </div>
    </div>
  </div>
</template>
