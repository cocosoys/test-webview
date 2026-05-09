import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";

type AppEnvironmentName = "dev" | "local" | "prod";

interface FrontendEnvironmentConfig {
  protocol: "http" | "https";
  host: string;
  listenHost: string | boolean;
  port: number;
  strictPort: boolean;
  hmrPort: number;
}

interface ServiceEnvironmentConfig {
  apiBaseUrl: string;
  webSocketUrl: string;
}

interface FeatureEnvironmentConfig {
  enableDebugTools: boolean;
  enableVerboseLogging: boolean;
}

interface CommandEnvironmentConfig {
  beforeDevCommand: string;
  beforeBuildCommand: string;
}

interface AppEnvironmentConfig {
  name: AppEnvironmentName;
  displayName: string;
  description: string;
  frontend: FrontendEnvironmentConfig;
  services: ServiceEnvironmentConfig;
  features: FeatureEnvironmentConfig;
  commands: CommandEnvironmentConfig;
}

interface EnvironmentConfigFile {
  defaultEnvironment: AppEnvironmentName;
  modeAliases: Record<string, AppEnvironmentName>;
  environments: Record<AppEnvironmentName, AppEnvironmentConfig>;
}

const rootDir = fileURLToPath(new URL(".", import.meta.url));

function loadEnvironment(mode: string): AppEnvironmentConfig {
  const configPath = resolve(rootDir, "environment.config.json");
  const config = JSON.parse(readFileSync(configPath, "utf-8")) as EnvironmentConfigFile;
  const environmentName = config.modeAliases[mode] ?? mode;

  if (environmentName in config.environments) {
    return config.environments[environmentName as AppEnvironmentName];
  }

  return config.environments[config.defaultEnvironment];
}

function toFrontendUrl(frontend: FrontendEnvironmentConfig) {
  return `${frontend.protocol}://${frontend.host}:${frontend.port}`;
}

function toRuntimeEnvironment(environment: AppEnvironmentConfig) {
  return {
    name: environment.name,
    displayName: environment.displayName,
    description: environment.description,
    frontendUrl: toFrontendUrl(environment.frontend),
    services: environment.services,
    features: environment.features,
  };
}

export default defineConfig(({ mode }) => {
  const environment = loadEnvironment(mode);
  const listenHost = process.env.TAURI_DEV_HOST || environment.frontend.listenHost;
  const hmrHost =
    process.env.TAURI_DEV_HOST ||
    (typeof environment.frontend.listenHost === "string" ? environment.frontend.listenHost : undefined);

  return {
    plugins: [vue(), tailwindcss()],

    define: {
      __APP_ENV__: JSON.stringify(toRuntimeEnvironment(environment)),
    },

    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },

    clearScreen: false,

    server: {
      port: environment.frontend.port,
      strictPort: environment.frontend.strictPort,
      host: listenHost || false,
      hmr: hmrHost
        ? {
            protocol: "ws",
            host: hmrHost,
            port: environment.frontend.hmrPort,
          }
        : undefined,
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  };
});
