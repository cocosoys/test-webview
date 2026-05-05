<!--
  AboutPanel.vue — 关于页面
  展示应用名称、配置版本、描述、技术栈和开源协议。

  配置版本从 Rust 后端通过 invoke("get_app_version") 获取，
  对应 config.yaml 中的 version 字段。
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { Copy, Heart } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const { t } = useI18n();

/** 配置版本号，从后端读取 */
const configVersion = ref("...");

// 组件挂载时从 Rust 后端获取版本号
onMounted(async () => {
  try {
    configVersion.value = await invoke<string>("get_app_version");
  } catch {
    // 如果后端不可用，使用默认值
    configVersion.value = "1.0";
  }
});

/** 技术栈列表，展示应用使用的主要技术 */
const techStack = [
  { name: "Tauri 2.x", category: t("about.backend") },
  { name: "Rust", category: t("about.backend") },
  { name: "Vue 3", category: t("about.frontend") },
  { name: "TypeScript", category: t("about.frontend") },
  { name: "TailwindCSS 4", category: t("about.frontend") },
  { name: "Vite", category: t("about.frontend") },
];
</script>

<template>
  <div class="rounded-xl p-6" style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border)">
    <!-- 应用图标 + 名称 + 版本 -->
    <div class="flex items-center gap-3 mb-4">
      <div class="w-12 h-12 rounded-xl flex items-center justify-center" style="background-color: var(--color-accent)">
        <Copy class="w-6 h-6 text-white" />
      </div>
      <div>
        <h2 class="text-lg font-bold" style="color: var(--color-text-primary)">
          {{ t('about.appName') }}
        </h2>
        <p class="text-xs" style="color: var(--color-text-secondary)">
          {{ t('about.configVersion') }} {{ configVersion }}
        </p>
      </div>
    </div>

    <!-- 应用描述 -->
    <p class="text-sm mb-4" style="color: var(--color-text-secondary)">
      {{ t('about.description') }}
    </p>

    <!-- 技术栈标签 -->
    <h3 class="text-sm font-semibold mb-2" style="color: var(--color-text-primary)">
      {{ t('about.techStack') }}
    </h3>
    <div class="flex flex-wrap gap-2 mb-4">
      <span
        v-for="tech in techStack"
        :key="tech.name"
        class="px-2.5 py-1 rounded-md text-xs font-medium"
        style="background-color: var(--color-bg-tertiary); color: var(--color-text-primary)"
      >
        {{ tech.name }}
        <span class="opacity-50 ml-1">{{ tech.category }}</span>
      </span>
    </div>

    <!-- 开源协议 -->
    <div class="flex items-center gap-2 text-xs" style="color: var(--color-text-secondary)">
      <Heart class="w-3.5 h-3.5 text-red-400" />
      <span>{{ t('about.license') }}: {{ t('about.mit') }}</span>
    </div>
  </div>
</template>
