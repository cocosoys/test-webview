<!--
  CloseDialog.vue — 关闭窗口确认对话框
  当用户点击关闭按钮且设置为"每次询问"时弹出此对话框。

  功能：
  - 提供两个选项："最小化到托盘" / "退出程序"
  - "记住我的选择"复选框：勾选后将选择保存到设置，下次不再询问
  - 点击遮罩层关闭对话框（取消关闭操作）

  技术要点：
  - <Teleport to="body">: 将对话框渲染到 body 根节点，避免被父元素 overflow:hidden 裁剪
  - defineEmits: 定义组件事件，向父组件（TitleBar）传递用户选择
-->
<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import { Minus, Power, X } from "lucide-vue-next";

// 定义组件事件：
// - close: 用户取消关闭（点击遮罩或 × 按钮）
// - action: 用户选择了关闭行为（minimize 或 quit）
const emit = defineEmits<{
  close: [];
  action: [action: "minimize" | "quit"];
}>();

const { t } = useI18n();
const settingsStore = useSettingsStore();

/** 是否勾选"记住我的选择" */
const rememberChoice = ref(false);

/**
 * 执行关闭行为
 * 如果勾选了"记住选择"，则保存到 settingsStore
 * 然后通过 emit 通知父组件执行具体操作
 */
function doAction(action: "minimize" | "quit") {
  if (rememberChoice.value) {
    // 保存用户偏好，下次点击关闭按钮时直接执行，不再弹窗
    settingsStore.setCloseAction(action);
  }
  emit("action", action);
}
</script>

<template>
  <!-- 使用 Teleport 将对话框渲染到 body，避免被裁剪 -->
  <Teleport to="body">
    <!-- 遮罩层：半透明黑色背景 + 毛玻璃效果 -->
    <div
      class="fixed inset-0 z-50 flex items-center justify-center"
      style="background-color: rgba(0,0,0,0.45); backdrop-filter: blur(2px)"
      @click.self="emit('close')"
    >
      <!-- 对话框主体 -->
      <div
        class="w-80 rounded-2xl p-6 shadow-2xl"
        style="background-color: var(--color-bg-primary); border: 1px solid var(--color-border)"
      >
        <!-- 标题行：标题 + 关闭按钮 -->
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-base font-bold" style="color: var(--color-text-primary)">
            {{ t('closeDialog.title') }}
          </h3>
          <button
            class="p-1 rounded-md transition-colors hover:bg-[var(--color-bg-tertiary)]"
            style="color: var(--color-text-secondary)"
            @click="emit('close')"
          >
            <X class="w-4 h-4" />
          </button>
        </div>

        <!-- 提示问题 -->
        <p class="text-sm mb-5" style="color: var(--color-text-secondary)">
          {{ t('closeDialog.question') }}
        </p>

        <!-- 选项按钮区域 -->
        <div class="space-y-2.5 mb-4">
          <!-- 选项1：最小化到托盘 -->
          <button
            class="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all duration-150"
            style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border); color: var(--color-text-primary)"
            @click="doAction('minimize')"
            @mouseenter="($event.target as HTMLElement).style.borderColor = 'var(--color-accent)'"
            @mouseleave="($event.target as HTMLElement).style.borderColor = 'var(--color-border)'"
          >
            <div class="p-1.5 rounded-lg" style="background-color: var(--color-accent); opacity: 0.15">
              <Minus class="w-4 h-4" style="color: var(--color-accent)" />
            </div>
            <div class="text-left">
              <div>{{ t('closeDialog.minimizeToTray') }}</div>
              <div class="text-xs mt-0.5" style="color: var(--color-text-secondary)">
                {{ t('closeDialog.minimizeToTrayDesc') }}
              </div>
            </div>
          </button>

          <!-- 选项2：退出程序 -->
          <button
            class="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all duration-150"
            style="background-color: var(--color-bg-secondary); border: 1px solid var(--color-border); color: var(--color-text-primary)"
            @click="doAction('quit')"
            @mouseenter="($event.target as HTMLElement).style.borderColor = '#ef4444'"
            @mouseleave="($event.target as HTMLElement).style.borderColor = 'var(--color-border)'"
          >
            <div class="p-1.5 rounded-lg" style="background-color: rgba(239,68,68,0.15)">
              <Power class="w-4 h-4 text-red-500" />
            </div>
            <div class="text-left">
              <div>{{ t('closeDialog.quitApp') }}</div>
              <div class="text-xs mt-0.5" style="color: var(--color-text-secondary)">
                {{ t('closeDialog.quitAppDesc') }}
              </div>
            </div>
          </button>
        </div>

        <!-- "记住我的选择"复选框 -->
        <label class="flex items-center gap-2 cursor-pointer select-none">
          <input
            type="checkbox"
            v-model="rememberChoice"
            class="w-4 h-4 rounded accent-[var(--color-accent)]"
          />
          <span class="text-xs" style="color: var(--color-text-secondary)">
            {{ t('closeDialog.rememberChoice') }}
          </span>
        </label>
      </div>
    </div>
  </Teleport>
</template>
