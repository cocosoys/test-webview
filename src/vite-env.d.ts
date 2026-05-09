/// <reference types="vite/client" />

declare const __APP_ENV__: import("./config/environment").AppEnvironmentConfig;

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<object, object, unknown>;
  export default component;
}
