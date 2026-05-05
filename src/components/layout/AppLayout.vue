<!--
  AppLayout.vue — 应用整体布局
  布局结构：
  ┌──────────────────────────────────┐
  │          标题栏 (TitleBar)         │  ← 固定在顶部，包含窗口控制按钮
  ├────────┬─────────────────────────┤
  │        │                         │
  │ 侧边栏  │      主内容区            │
  │(Sidebar)│   (<router-view>)      │
  │        │                         │
  └────────┴─────────────────────────┘
-->
<script setup lang="ts">
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar.vue";
</script>

<template>
  <!-- 最外层容器：垂直 flex 布局，占满整个屏幕 -->
  <div class="flex flex-col h-screen w-screen overflow-hidden" style="background-color: var(--color-bg-primary)">
    <!-- 顶部标题栏：自定义窗口控制按钮（最小化/最大化/关闭） -->
    <TitleBar />

    <!-- 标题栏下方：水平 flex 布局（侧边栏 + 主内容） -->
    <div class="flex flex-1 overflow-hidden">
      <!-- 侧边栏导航 -->
      <Sidebar />

      <!-- 主内容区：通过 <router-view> 渲染当前路由对应的页面 -->
      <!-- overflow-y-auto: 内容超出时可垂直滚动 -->
      <main class="flex-1 overflow-y-auto p-6" style="background-color: var(--color-bg-primary)">
        <router-view />
      </main>
    </div>
  </div>
</template>
