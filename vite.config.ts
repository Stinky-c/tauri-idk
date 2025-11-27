import { defineConfig } from "vite";
import path from "node:path";

// Plugins
import AutoImport from "unplugin-auto-import/vite";
import { VueRouterAutoImports } from "unplugin-vue-router";
import VueRouter from "unplugin-vue-router/vite";
import Icons from "unplugin-icons/vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST as string;

// https://vite.dev/config/
export default defineConfig(async () => ({
    plugins: [
        Icons({
            // experimental
            autoInstall: true,
        }),
        AutoImport({
            imports: ["vue", VueRouterAutoImports],
        }),
        VueRouter(),
        vue(),
        tailwindcss(),
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
            "~": path.resolve(__dirname, "./src/pages"),
        },
    },
}));
