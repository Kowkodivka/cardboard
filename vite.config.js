import tailwindcss from "@tailwindcss/vite";
import { fileRoutes } from "filesystem-routing/vite";
import { defineConfig, loadEnv } from "vite";
import solid from "@solidjs/vite-plugin";
import Icons from "unplugin-icons/vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const backendTarget = env.CARDBOARD_DEV_PROXY_TARGET ?? "http://localhost:8080";

  return {
    resolve: {
      alias: {
        "@": new URL("./src", import.meta.url).pathname,
      },
    },
    plugins: [
      solid({ start: true, extensions: [".jsx"], diagnostics: true }),
      fileRoutes({ types: ".solid/file-routes.d.ts" }),
      tailwindcss(),
      Icons({
        compiler: "solid",
        autoInstall: true,
      }),
    ],
    server: {
      port: 3000,
      proxy: {
        "/api": {
          target: backendTarget,
          changeOrigin: true,
        },
      },
    },
    build: {
      target: "esnext",
      assetsInlineLimit: 0,
    },
  };
});
