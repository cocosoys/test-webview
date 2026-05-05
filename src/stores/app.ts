/**
 * stores/app.ts — 应用全局状态（主题、侧边栏）
 *
 * Pinia 是 Vue 的状态管理库，用于在组件间共享数据。
 * 这个 store 管理应用级别的状态：主题模式和侧边栏折叠状态。
 *
 * 使用方式（在组件中）：
 * ```typescript
 * import { useAppStore } from "@/stores/app";
 * const appStore = useAppStore();
 * appStore.setTheme("dark");       // 切换到暗色主题
 * appStore.toggleSidebar();        // 折叠/展开侧边栏
 * console.log(appStore.theme);     // 读取当前主题
 * ```
 *
 * 关键概念：
 * - defineStore: 定义一个 store（状态仓库）
 * - ref: 响应式数据，值变化时自动更新 UI
 * - Composition API 风格：用函数组织逻辑，比 Options API 更灵活
 */

import { defineStore } from "pinia";
import { ref } from "vue";

/** 主题模式类型 */
export type ThemeMode = "light" | "dark" | "system";

/**
 * useAppStore — 应用全局状态
 *
 * 管理两个状态：
 * 1. theme: 当前主题模式（亮色/暗色/跟随系统）
 * 2. sidebarCollapsed: 侧边栏是否折叠
 */
export const useAppStore = defineStore("app", () => {
  // =========================================================================
  // 响应式状态
  // =========================================================================

  /** 当前主题模式，默认"跟随系统" */
  const theme = ref<ThemeMode>("system");

  /** 侧边栏是否折叠，默认展开 */
  const sidebarCollapsed = ref(false);

  // =========================================================================
  // 方法
  // =========================================================================

  /**
   * 设置主题模式
   * 同时更新状态值和应用 CSS 类名
   *
   * @param mode - 主题模式："light" | "dark" | "system"
   */
  function setTheme(mode: ThemeMode) {
    theme.value = mode;
    applyTheme(mode);
  }

  /**
   * 应用主题到 DOM
   *
   * 工作原理：
   * - TailwindCSS 的 dark 模式使用 "class" 策略
   * - 在 <html> 标签上添加/移除 "dark" 类名来切换主题
   * - "system" 模式通过 matchMedia 检测操作系统的暗色模式设置
   *
   * @param mode - 主题模式
   */
  function applyTheme(mode: ThemeMode) {
    const html = document.documentElement; // <html> 标签
    if (mode === "system") {
      // 跟随系统：检测操作系统是否启用了暗色模式
      const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      html.classList.toggle("dark", isDark);
    } else {
      // 手动模式：直接根据选择设置
      html.classList.toggle("dark", mode === "dark");
    }
  }

  /** 切换侧边栏折叠/展开状态 */
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  // =========================================================================
  // 初始化逻辑
  // =========================================================================

  // 应用启动时立即应用主题（使用默认值 "system"）
  applyTheme(theme.value);

  // 监听操作系统主题变化
  // 当用户在系统设置中切换亮/暗模式时，如果应用是"跟随系统"，则自动更新
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (theme.value === "system") {
      applyTheme("system");
    }
  });

  // Pinia 要求返回所有需要在组件中使用的状态和方法
  return { theme, sidebarCollapsed, setTheme, applyTheme, toggleSidebar };
});
