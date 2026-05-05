/**
 * vite.config.ts — Vite 构建工具配置
 *
 * Vite 是前端构建工具（类似 Webpack，但更快），负责：
 * - 开发时提供热更新（HMR）服务器
 * - 生产时打包优化代码
 *
 * Tauri 要求 Vite 开发服务器运行在固定端口（1420），
 * 这样 Tauri 的 WebView 才能正确加载前端页面。
 */

import { defineConfig } from "vite";
// Vue 官方 Vite 插件，支持 .vue 单文件组件
import vue from "@vitejs/plugin-vue";
// TailwindCSS 4 的 Vite 插件，自动扫描类名并生成 CSS
import tailwindcss from "@tailwindcss/vite";
// Node.js URL 工具，用于构建文件路径别名
import { fileURLToPath, URL } from "node:url";

// TAURI_DEV_HOST 环境变量：在移动端开发时，Tauri 会设置此变量为主机 IP
// 用于让手机/模拟器访问开发服务器
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  // 注册 Vite 插件
  plugins: [
    vue(),         // 支持 .vue 文件
    tailwindcss(), // 支持 TailwindCSS 4
  ],

  // 路径别名配置
  resolve: {
    alias: {
      // 将 @ 映射到 src/ 目录
      // 这样在导入时可以用 @/components/... 代替 ../../components/...
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // 不清屏：Tauri CLI 需要读取 Vite 的输出，清屏会干扰
  clearScreen: false,

  // 开发服务器配置
  server: {
    // 固定端口 1420（Tauri 默认配置，需与 tauri.conf.json 中 devUrl 一致）
    port: 1420,
    // 严格端口：如果 1420 被占用则报错，而不是自动换端口
    // 因为 Tauri WebView 会访问固定地址，端口变了就找不到页面
    strictPort: true,
    // 监听地址：移动端开发时用 TAURI_DEV_HOST，否则只监听本地
    host: host || false,
    // 热更新（HMR）配置：移动端开发时通过 WebSocket 连接
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    // 文件监听：忽略 Rust 后端目录，避免 Rust 文件变更触发前端热更新
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
