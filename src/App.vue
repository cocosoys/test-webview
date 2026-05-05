<!--
  App.vue — Vue 根组件
  这是整个前端组件树的根节点，只负责渲染应用布局组件 AppLayout。
  所有页面内容（仪表盘、设置、关于）都通过 AppLayout → <router-view> 展示。
-->
<script setup lang="ts">
// 导入应用布局组件，包含标题栏 + 侧边栏 + 主内容区
import AppLayout from "@/components/layout/AppLayout.vue";
</script>

<template>
  <!-- 渲染应用布局 -->
  <AppLayout />
</template>
