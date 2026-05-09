import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const environmentPath = resolve(rootDir, "environment.config.json");
const tauriConfigPath = resolve(rootDir, "src-tauri", "tauri.conf.json");

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function frontendUrl(frontend) {
  return `${frontend.protocol}://${frontend.host}:${frontend.port}`;
}

const config = readJson(environmentPath);
const requestedEnvironment = process.argv[2] || process.env.APP_ENV || config.defaultEnvironment;
const environmentName = config.modeAliases[requestedEnvironment] || requestedEnvironment;
const environment = config.environments[environmentName];

if (!environment) {
  const known = Object.keys(config.environments).join(", ");
  throw new Error(`Unknown environment "${requestedEnvironment}". Known environments: ${known}`);
}

const tauriConfig = readJson(tauriConfigPath);
tauriConfig.build = {
  ...tauriConfig.build,
  devUrl: frontendUrl(environment.frontend),
  beforeDevCommand: environment.commands.beforeDevCommand,
  beforeBuildCommand: environment.commands.beforeBuildCommand,
};

writeFileSync(tauriConfigPath, `${JSON.stringify(tauriConfig, null, 2)}\n`);

console.log(
  `[env] synced ${environment.name}: devUrl=${tauriConfig.build.devUrl}, ` +
    `beforeDevCommand="${tauriConfig.build.beforeDevCommand}", ` +
    `beforeBuildCommand="${tauriConfig.build.beforeBuildCommand}"`,
);
