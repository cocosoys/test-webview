<!--
  Sidebar.vue — 侧边栏导航
  包含页面导航按钮（仪表盘/设置/关于）和折叠/展开切换。

  导航项通过 vue-i18n 翻译，因此切换语言时按钮文本会自动更新。
  当前活动路由的按钮会高亮显示（蓝色背景）。
-->
<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { useAppStore } from "@/stores/app";
import { LayoutDashboard, Settings, Info, ChevronLeft, ChevronRight } from "lucide-vue-next";
import { computed } from "vue";

const route = useRoute();   // 当前路由信息（用于判断哪个按钮高亮）
const router = useRouter(); // 路由实例（用于编程式导航）
const { t } = useI18n();    // 翻译函数
const appStore = useAppStore(); // 应用状态（侧边栏折叠状态）

/**
 * 导航项列表
 * 使用 computed 确保切换语言时 label 自动更新
 * 每项包含：路径、图标组件、翻译后的标签
 */
const navItems = computed(() => [
  { path: "/dashboard", icon: LayoutDashboard, label: t("app.dashboard") },
  { path: "/settings", icon: Settings, label: t("app.settings") },
  { path: "/about", icon: Info, label: t("app.about") },
]);

/** 点击导航项，跳转到对应路由 */
function navigate(path: string) {
  router.push(path);
}

/** 判断某个路径是否是当前活动路由（用于高亮显示） */
function isActive(path: string): boolean {
  return route.path === path;
}
</script>

<template>
  <!-- 侧边栏容器 -->
  <aside
    class="flex flex-col h-full shrink-0 transition-all duration-200"
    :class="appStore.sidebarCollapsed ? 'w-14' : 'w-52'"
    style="background-color: var(--color-sidebar-bg); border-right: 1px solid var(--color-border)"
  >
    <!-- 导航按钮列表 -->
    <nav class="flex-1 py-3 px-2 space-y-1">
      <button
        v-for="item in navItems"
        :key="item.path"
        class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-colors"
        :class="
          isActive(item.path)
            ? 'bg-[var(--color-accent)] text-white'  /* 活动项：蓝色背景白字 */
            : 'hover:bg-[var(--color-bg-tertiary)]'  /* 非活动项：悬停变灰 */
        "
        :style="!isActive(item.path) ? 'color: var(--color-text-primary)' : ''"
        @click="navigate(item.path)"
      >
        <!-- 图标（始终显示，即使侧边栏折叠） -->
        <component :is="item.icon" class="w-5 h-5 shrink-0" />
        <!-- 文字标签（折叠时隐藏） -->
        <span v-if="!appStore.sidebarCollapsed" class="truncate">{{ item.label }}</span>
      </button>
    </nav>

    <!-- 底部：折叠/展开切换按钮 -->
    <div class="p-2 border-t" style="border-color: var(--color-border)">
      <button
        class="w-full flex items-center justify-center p-2 rounded-lg text-sm transition-colors hover:bg-[var(--color-bg-tertiary)]"
        style="color: var(--color-text-secondary)"
        @click="appStore.toggleSidebar()"
      >
        <!-- 展开状态显示左箭头（点击折叠），折叠状态显示右箭头（点击展开） -->
        <ChevronLeft v-if="!appStore.sidebarCollapsed" class="w-4 h-4" />
        <ChevronRight v-else class="w-4 h-4" />
      </button>
    </div>
  </aside>
</template>
