<!--
  TitleBar.vue — 自定义无边框窗口标题栏
  替代操作系统原生标题栏，提供：
  - 应用名称显示
  - 窗口拖拽移动（通过 data-tauri-drag-region 属性）
  - 最小化 / 最大化 / 关闭按钮
  - 关闭行为选择弹窗

  重要概念：
  - data-tauri-drag-region: Tauri 专用属性，标记该区域可拖拽移动窗口
  - 按钮区域不带此属性，确保按钮可正常点击
  - pointer-events-none 在图标上，防止点击事件被图标拦截
  - 使用 scoped CSS 实现按钮悬停/激活效果
-->
<script setup lang="ts">
// getCurrentWindow: 获取当前 Tauri 窗口实例，用于调用最小化/最大化/关闭等方法
import { getCurrentWindow } from "@tauri-apps/api/window";
// listen: 监听来自 Rust 后端的事件（如 Alt+F4 触发的 close-requested）
import { listen } from "@tauri-apps/api/event";
// Lucide 图标库
import { Minus, Square, X, Copy, Columns2 } from "lucide-vue-next";
import { ref, computed, onMounted, onUnmounted } from "vue";
// vue-i18n: 国际化翻译函数
import { useI18n } from "vue-i18n";
// 设置状态：读取关闭行为配置
import { useSettingsStore, type CloseAction } from "@/stores/settings";
// 关闭确认对话框组件
import CloseDialog from "./CloseDialog.vue";

const { t } = useI18n();
const settingsStore = useSettingsStore();
const appWindow = getCurrentWindow(); // 当前窗口实例

// =========================================================================
// 响应式状态
// =========================================================================

/** 窗口是否最大化 */
const isMaximized = ref(false);

/** 是否显示关闭确认对话框 */
const showCloseDialog = ref(false);

// 事件监听器的取消函数（用于组件卸载时清理）
let unlistenResized: (() => void) | null = null;
let unlistenCloseRequested: (() => void) | null = null;

// =========================================================================
// 生命周期：挂载时注册事件监听
// =========================================================================

onMounted(async () => {
  // 监听窗口大小变化，更新 isMaximized 状态
  // 这样最大化/还原时图标能自动切换
  try {
    unlistenResized = await appWindow.onResized(async () => {
      try {
        isMaximized.value = await appWindow.isMaximized();
      } catch { /* 忽略错误 */ }
    });
  } catch { /* 忽略错误 */ }

  // 监听 Rust 后端发出的 "close-requested" 事件
  // 当用户按 Alt+F4 且 close_action 为 "ask" 时，Rust 会阻止关闭并发出此事件
  try {
    unlistenCloseRequested = await listen("close-requested", () => {
      handleClose();
    });
  } catch { /* 忽略错误 */ }

  // 初始化最大化状态
  try {
    isMaximized.value = await appWindow.isMaximized();
  } catch { /* 忽略错误 */ }
});

// 组件卸载时取消事件监听，避免内存泄漏
onUnmounted(() => {
  unlistenResized?.();
  unlistenCloseRequested?.();
});

// =========================================================================
// 窗口控制方法
// =========================================================================

/** 最小化窗口 */
async function minimize() {
  try {
    await appWindow.minimize();
  } catch (e) {
    console.error("Failed to minimize:", e);
  }
}

/** 切换最大化/还原 */
async function toggleMaximize() {
  try {
    await appWindow.toggleMaximize();
  } catch (e) {
    console.error("Failed to toggle maximize:", e);
  }
}

/**
 * 处理关闭按钮点击
 * 根据 settingsStore.closeAction 的值决定行为：
 * - "minimize": 隐藏窗口到系统托盘
 * - "quit": 销毁窗口，退出程序
 * - "ask": 弹出确认对话框让用户选择
 */
async function handleClose() {
  if (settingsStore.closeAction === "minimize") {
    try { await appWindow.hide(); } catch (e) { console.error("Failed to hide:", e); }
  } else if (settingsStore.closeAction === "quit") {
    try { await appWindow.destroy(); } catch (e) { console.error("Failed to destroy:", e); }
  } else {
    // "ask" 模式：显示关闭确认对话框
    showCloseDialog.value = true;
  }
}

/**
 * 执行用户在关闭对话框中选择的行为
 * @param action - "minimize"（最小化到托盘）或 "quit"（退出程序）
 */
async function executeCloseAction(action: CloseAction) {
  showCloseDialog.value = false;
  if (action === "minimize") {
    try { await appWindow.hide(); } catch (e) { console.error("Failed to hide:", e); }
  } else {
    try { await appWindow.destroy(); } catch (e) { console.error("Failed to destroy:", e); }
  }
}

/** 最大化按钮的提示文本（根据当前状态切换） */
const maximizeTooltip = computed(() =>
  isMaximized.value ? t("titlebar.restore") : t("titlebar.maximize")
);
</script>

<template>
  <!--
    标题栏容器
    注意：此 div 不带 data-tauri-drag-region，因为按钮也在其中
    拖拽区域在左右两侧的子 div 上单独设置
  -->
  <div
    class="flex items-center justify-between h-10 px-3 shrink-0 select-none"
    style="background-color: var(--color-titlebar-bg); border-bottom: 1px solid var(--color-border)"
  >
    <!-- 左侧：应用图标 + 名称（可拖拽区域） -->
    <div class="flex items-center gap-2.5 flex-1 h-full" data-tauri-drag-region>
      <div class="w-5 h-5 rounded-md flex items-center justify-center" style="background-color: var(--color-accent)">
        <Copy class="w-3 h-3 text-white" />
      </div>
      <span class="text-xs font-semibold tracking-wide" style="color: var(--color-text-primary)">
        {{ t('app.name') }}
      </span>
    </div>

    <!-- 中间：空白区域（可拖拽，用于移动窗口） -->
    <div class="flex-1 h-full" data-tauri-drag-region></div>

    <!-- 右侧：窗口控制按钮（不可拖拽） -->
    <div class="flex items-center flex-shrink-0 h-full">
      <!-- 最小化按钮 -->
      <button
        class="win-btn w-12 h-full flex items-center justify-center transition-all duration-150 cursor-pointer"
        :title="t('titlebar.minimize')"
        @click="minimize"
      >
        <!-- pointer-events-none: 防止点击事件被 SVG 图标拦截 -->
        <Minus class="w-4 h-4 pointer-events-none" />
      </button>

      <!-- 最大化/还原按钮 -->
      <button
        class="win-btn w-12 h-full flex items-center justify-center transition-all duration-150 cursor-pointer"
        :title="maximizeTooltip"
        @click="toggleMaximize"
      >
        <!-- 最大化状态显示"还原"图标，否则显示"最大化"图标 -->
        <Columns2 v-if="isMaximized" class="w-4 h-4 pointer-events-none" />
        <Square v-else class="w-3.5 h-3.5 pointer-events-none" />
      </button>

      <!-- 关闭按钮 -->
      <button
        class="win-btn-close w-12 h-full flex items-center justify-center transition-all duration-150 cursor-pointer"
        :title="t('titlebar.close')"
        @click="handleClose"
      >
        <X class="w-4 h-4 pointer-events-none" />
      </button>
    </div>

    <!-- 关闭确认对话框（条件渲染） -->
    <CloseDialog
      v-if="showCloseDialog"
      @close="showCloseDialog = false"
      @action="executeCloseAction"
    />
  </div>
</template>

<style scoped>
/* 最小化/最大化按钮样式 */
.win-btn {
  color: var(--color-text-secondary);
  pointer-events: auto; /* 确保按钮可点击 */
}
.win-btn:hover {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}
.win-btn:active {
  transform: scale(0.92); /* 按下时缩小，提供触觉反馈 */
  background-color: var(--color-bg-tertiary);
}

/* 关闭按钮样式：悬停时变红色 */
.win-btn-close {
  color: var(--color-text-secondary);
  pointer-events: auto;
}
.win-btn-close:hover {
  background-color: #ef4444; /* 红色背景 */
  color: #ffffff;             /* 白色图标 */
}
.win-btn-close:active {
  transform: scale(0.92);
  background-color: #dc2626; /* 更深的红色 */
}
</style>
