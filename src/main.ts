/**
 * main.ts — Vue 应用入口文件
 *
 * 这是整个前端应用的启动点，负责：
 * 1. 创建 Vue 应用实例
 * 2. 注册全局插件（Pinia 状态管理、Vue Router 路由、vue-i18n 国际化）
 * 3. 挂载到 DOM
 * 4. 监听来自 Rust 后端的事件（如原生菜单触发的导航）
 */

// 从 Vue 导入 createApp 工厂函数，用于创建应用实例
import { createApp } from "vue";
// 从 Pinia 导入状态管理器（Vue 官方推荐的状态管理库）
import { createPinia } from "pinia";
// 从 Tauri API 导入事件监听器，用于接收 Rust 后端发来的事件
import { listen } from "@tauri-apps/api/event";
// 根组件
import App from "./App.vue";
// 路由配置
import router from "./router";
// 国际化配置
import i18n from "./i18n";
// 全局样式（包含 TailwindCSS）
import "./styles/main.css";

// 创建 Vue 应用实例，传入根组件
const app = createApp(App);

// 注册 Pinia 状态管理插件
// 之后在组件中可以通过 useAppStore()、useSettingsStore() 等访问全局状态
app.use(createPinia());

// 注册 Vue Router 路由插件
// 之后在组件中可以使用 <router-view>、<router-link>、router.push() 等
app.use(router);

// 注册 vue-i18n 国际化插件
// 之后在组件中可以使用 t() 函数翻译文本，如 t("app.name")
app.use(i18n);

// 将 Vue 应用挂载到 HTML 中 id="app" 的 DOM 元素上
// 对应 index.html 中的 <div id="app"></div>
app.mount("#app");

// =============================================================================
// 监听来自 Rust 后端的导航事件
// =============================================================================
//
// 当用户点击原生菜单（如"文件 → 设置"）时，Rust 后端会 emit "navigate" 事件
// 前端收到事件后，使用 router.push() 进行页面跳转
//
// 为什么需要这样做？
// - 原生菜单是 Rust 创建的，无法直接调用 Vue Router
// - 通过 Tauri 事件机制作为桥梁：Rust 发事件 → 前端监听 → 执行路由跳转
//
// 流程示例：
// 1. 用户点击"帮助 → 关于"
// 2. Rust menu.rs: handle.emit("navigate", "/about")
// 3. 前端 main.ts: listen("navigate", ...) 收到事件
// 4. router.push("/about") 跳转到关于页面
listen<string>("navigate", (event) => {
  if (event.payload) {
    router.push(event.payload);
  }
});
