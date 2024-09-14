import { NUKAppPreset } from "./primevue.theme";

export default defineNuxtConfig({
  modules: [
    "@vueuse/nuxt",
    "@unocss/nuxt",
    "@nuxt/icon",
    "nuxt-svgo",
    "@nuxt/eslint",
    "@nuxtjs/i18n",
    "@pinia/nuxt",
    "pinia-plugin-persistedstate/nuxt",
    "@ant-design-vue/nuxt",
    "@primevue/nuxt-module",
  ],
  app: {
    head: {
      title: "NUKApp",
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      meta: [{ name: "format-detection", content: "no" }],
      bodyAttrs: {
        class: "font-text antialiased",
      },
    },
    pageTransition: {
      name: "page",
      mode: "out-in",
    },
    layoutTransition: {
      name: "layout",
      mode: "out-in",
    },
  },
  experimental: {
    restoreState: true,
    typedPages: true,
    watcher: "chokidar",
  },
  primevue: {
    options: {
      theme: {
        preset: NUKAppPreset,
        options: {
          darkModeSelector: ".dark",
        },
      },
    },
  },
  css: [
    "@unocss/reset/tailwind.css",
    "assets/css/colors.scss",
    "assets/css/scroll.scss",
    "assets/css/typography.scss",
    "assets/css/app.scss",
  ],
  svgo: {
    autoImportPath: "@/assets/",
  },
  i18n: {
    strategy: "no_prefix",
    langDir: "locales",
    locales: [
      {
        code: "en",
        language: "en-US",
        file: "en.json",
      },
      {
        code: "zh-tw",
        language: "zh-TW",
        file: "zh-TW.json",
      },
      {
        code: "zh-cn",
        language: "zh-CN",
        file: "zh-CN.json",
      },
    ],
    defaultLocale: "zh-tw",
  },
  alias: {},
  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      hmr: {
        protocol: "ws",
        host: "0.0.0.0",
        port: 3001,
      },
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  },
  srcDir: "src/",
  ssr: false,
  devServer: {
    host: "0.0.0.0",
  },
  eslint: {
    config: {
      standalone: false,
    },
  },
  compatibilityDate: "2024-08-01",
});
