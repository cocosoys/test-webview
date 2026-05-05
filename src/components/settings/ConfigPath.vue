<!--
  ConfigPath.vue — 配置文件路径设置
  显示当前配置文件存储路径，提供文件夹选择器修改路径。

  技术要点：
  - 使用 @tauri-apps/plugin-dialog 的 open() 打开系统文件夹选择对话框
  - 使用 invoke("get_config") / invoke("save_config") 读写配置
  - 组件挂载时从后端加载当前配置，初始化前端状态
-->
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
// open: Tauri 对话框插件，打开文件/文件夹选择器
import { open } from "@tauri-apps/plugin-dialog";
// invoke: 调用 Rust 后端命令
import { invoke } from "@tauri-apps/api/core";
import { FolderOpen } from "lucide-vue-next";
import { onMounted } from "vue";

const { t } = useI18n();
const settingsStore = useSettingsStore();

// 组件挂载时从后端加载配置，初始化前端状态
onMounted(async () => {
  try {
    // get_config 返回的 JSON 使用 snake_case 键名（Rust serde 默认）
    const config = await invoke<Record<string, unknown>>("get_config");
    settingsStore.setConfigPath(String(config.config_path || ""));
    settingsStore.setLocale(String(config.locale || "zh-CN"));
    settingsStore.setCloseAction(String(config.close_action || "ask") as "ask" | "minimize" | "quit");
  } catch {
    // 配置文件尚未创建，使用默认值
  }
});

/**
 * 打开文件夹选择器，让用户选择新的配置文件存储路径
 */
async function changePath() {
  // 调用 Tauri 对话框插件的 open 方法
  // directory: true 表示选择文件夹（而非文件）
  // multiple: false 表示只选择一个
  const selected = await open({
    directory: true,
    multiple: false,
    title: t("settings.changePath"),
  });
  if (selected) {
    // 更新前端状态
    settingsStore.setConfigPath(selected);
    try {
      // 重新读取完整配置，只修改 config_path 字段，然后保存
      const config = await invoke<Record<string, unknown>>("get_config");
      config.config_path = selected;
      await invoke("save_config", { config });
    } catch {
      // 静默处理保存错误
    }
  }
}
</script>

<template>
  <div>
    <label class="text-sm font-medium mb-1 block" style="color: var(--color-text-primary)">
      {{ t('settings.configPath') }}
    </label>
    <p class="text-xs mb-3" style="color: var(--color-text-secondary)">
      {{ t('settings.configPathDescription') }}
    </p>
    <div class="flex items-center gap-3">
      <!-- 当前路径显示（等宽字体，超出截断） -->
      <div
        class="flex-1 px-3 py-2 rounded-lg text-sm font-mono truncate"
        style="background-color: var(--color-bg-tertiary); color: var(--color-text-primary)"
      >
        {{ settingsStore.configPath || t('settings.currentPath') + ': ...' }}
      </div>
      <!-- 更改路径按钮 -->
      <button
        class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium transition-colors"
        style="background-color: var(--color-accent); color: white"
        @click="changePath"
      >
        <FolderOpen class="w-4 h-4" />
        {{ t('settings.changePath') }}
      </button>
    </div>
  </div>
</template>
