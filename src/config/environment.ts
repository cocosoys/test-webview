export type AppEnvironmentName = "dev" | "local" | "prod";

export interface AppServiceConfig {
  apiBaseUrl: string;
  webSocketUrl: string;
}

export interface AppFeatureConfig {
  enableDebugTools: boolean;
  enableVerboseLogging: boolean;
}

export interface AppEnvironmentConfig {
  name: AppEnvironmentName;
  displayName: string;
  description: string;
  frontendUrl: string;
  services: AppServiceConfig;
  features: AppFeatureConfig;
}

const fallbackEnvironment: AppEnvironmentConfig = {
  name: "dev",
  displayName: "Development",
  description: "Fallback development environment.",
  frontendUrl: "http://localhost:1420",
  services: {
    apiBaseUrl: "http://localhost:3000",
    webSocketUrl: "ws://localhost:3000",
  },
  features: {
    enableDebugTools: true,
    enableVerboseLogging: true,
  },
};

export const appEnvironment: AppEnvironmentConfig =
  typeof __APP_ENV__ === "undefined" ? fallbackEnvironment : __APP_ENV__;

export const apiBaseUrl = appEnvironment.services.apiBaseUrl;
export const webSocketUrl = appEnvironment.services.webSocketUrl;
export const isProductionEnvironment = appEnvironment.name === "prod";
