/**
 * router/index.ts — Vue Router 路由配置
 *
 * Vue Router 是 Vue 官方的路由管理器，负责：
 * - 将 URL 路径映射到对应的页面组件
 * - 处理页面之间的导航
 * - 支持路由懒加载（按需加载，减小首屏体积）
 *
 * 路由结构：
 * /           → 重定向到 /dashboard
 * /dashboard  → 仪表盘页面
 * /settings   → 设置页面
 * /about      → 关于页面
 */

import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  // 使用 HTML5 History 模式（URL 无 # 号）
  // 在 Tauri WebView 中，所有路径都指向同一个 index.html
  createWebHistory(),

  // 路由规则数组
  routes: [
    {
      // 根路径重定向到仪表盘
      path: "/",
      redirect: "/dashboard",
    },
    {
      path: "/dashboard",
      name: "dashboard",
      // 懒加载：只在访问该路由时才下载对应的 JS 文件
      // 语法：() => import("路径")
      component: () => import("@/components/dashboard/Dashboard.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/components/settings/SettingsPage.vue"),
    },
    {
      path: "/about",
      name: "about",
      component: () => import("@/components/settings/AboutPanel.vue"),
    },
  ],
});

export default router;
