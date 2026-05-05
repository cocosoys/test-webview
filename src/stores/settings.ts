/**
 * stores/settings.ts — 设置相关状态（语言、配置路径、关闭行为）
 *
 * 这个 store 管理用户可配置的设置项，对应"设置"页面中的各个选项。
 *
 * 使用方式（在组件中）：
 * ```typescript
 * import { useSettingsStore } from "@/stores/settings";
 * const settingsStore = useSettingsStore();
 * settingsStore.setLocale("en-US");      // 切换到英文
 * settingsStore.setCloseAction("quit");  // 设置关闭行为为"退出"
 * ```
 *
 * 这些设置值最终会通过 Tauri 命令持久化到 config.yaml 文件中。
 */

import { defineStore } from "pinia";
import { ref } from "vue";

/** 关闭窗口行为类型 */
export type CloseAction = "ask" | "minimize" | "quit";

/**
 * useSettingsStore — 设置状态
 *
 * 管理的设置项：
 * 1. locale:        界面语言（zh-CN / en-US）
 * 2. configPath:    配置文件存储路径
 * 3. configVersion: 配置文件版本号（从后端读取）
 * 4. closeAction:   关闭窗口行为（ask / minimize / quit）
 */
export const useSettingsStore = defineStore("settings", () => {
  /** 界面语言，默认中文 */
  const locale = ref("zh-CN");

  /** 配置文件存储路径，默认为空（使用 Tauri 标准路径） */
  const configPath = ref("");

  /** 配置文件版本号，从后端 get_app_version 命令获取 */
  const configVersion = ref("");

  /** 关闭窗口行为，默认"每次询问" */
  const closeAction = ref<CloseAction>("ask");

  /** 设置界面语言 */
  function setLocale(lang: string) {
    locale.value = lang;
  }

  /** 设置配置文件路径 */
  function setConfigPath(path: string) {
    configPath.value = path;
  }

  /** 设置关闭窗口行为 */
  function setCloseAction(action: CloseAction) {
    closeAction.value = action;
  }

  /** 设置配置文件版本号 */
  function setConfigVersion(ver: string) {
    configVersion.value = ver;
  }

  return { locale, configPath, configVersion, closeAction, setLocale, setConfigPath, setCloseAction, setConfigVersion };
});
