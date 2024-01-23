<script lang="ts">
  import { logger } from "./logger";

  import { useRoute } from "vue-router";
  import { theme } from "ant-design-vue";
  import { useThemeStore } from "./stores/theme";
  // Components
  import window_control from "./components/window_control.vue";
  // tauri
  import { Window } from "@tauri-apps/api/window";
  import { platform } from "@tauri-apps/plugin-os";
  import { router } from "./router";
  const platformName = await platform();
  const AppWindow = Window.getCurrent();

  export default {
    components: {
      window_control,
    },
    data() {
      return {
        themeData: {
          algorithm: theme.defaultAlgorithm,
          light_token: {
            colorBgLayout: "#ffffff50",
            colorPrimaryBg: "#00000015",
            colorPrimary: "#141414",
          },
          dark_token: {
            colorBgLayout: "#00000030",
            colorPrimaryBg: "#ffffff15",
            colorPrimary: "#ffffff",
          },
          token: {},
        },
        mediaQueryList: null as MediaQueryList | null,
        systemTheme: "",
        isFocused: true,
      };
    },
    mounted() {
      logger.info(`Platform: ${platformName}`);
      AppWindow.listen("tauri://focus", ({}) => {
        this.focused(true);
      });
      AppWindow.listen("tauri://blur", ({}) => {
        this.focused(false);
      });
      // System theme
      this.mediaQueryList = window.matchMedia("(prefers-color-scheme: dark)");
      this.systemTheme = this.mediaQueryList.matches ? "dark" : "light";
      this.$nextTick(() => {
        this.mediaQueryList!.addEventListener("change", (e) => {
          this.systemTheme = e.matches ? "dark" : "light";
        });
        if (this.themeStore.theme === "default") {
          this.switchTheme(this.systemTheme);
        }
      });
      if (this.themeStore.theme) {
        this.switchTheme(this.themeStore.theme);
      }
    },
    unmounted() {
      this.mediaQueryList!.removeEventListener("change", () => {});
    },
    computed: {
      themeStore() {
        return useThemeStore();
      },
      currentRoute() {
        return useRoute();
      },
    },
    methods: {
      focused(focus: boolean) {
        this.isFocused = focus;

        if (!focus) {
          if (this.themeData.algorithm === theme.darkAlgorithm) {
            document.documentElement.style.setProperty(
              "background-color",
              "#000000"
            );
          } else {
            document.documentElement.style.setProperty(
              "background-color",
              "#ffffff"
            );
          }
        } else {
          document.documentElement.style.removeProperty("background-color");
        }
      },
      switchTheme(mode: string) {
        let variables = {
          "--color-surface":
            mode === "dark"
              ? "var(  --dark-color-surface)"
              : "var(  --light-color-surface)",
          "--base-color":
            mode === "dark"
              ? "var(--dark-base-color)"
              : "var(--light-base-color)",
        };
        for (let [key, value] of Object.entries(variables)) {
          document.documentElement.style.setProperty(key, value);
        }
        this.themeData = {
          algorithm:
            mode === "dark" ? theme.darkAlgorithm : theme.defaultAlgorithm,
          token:
            mode === "dark"
              ? { ...this.themeData.dark_token }
              : { ...this.themeData.light_token },
          dark_token: {
            ...this.themeData.dark_token,
          },
          light_token: {
            ...this.themeData.light_token,
          },
        };
      },
    },

    watch: {
      "themeStore.theme"(val) {
        if (val === "default") {
          this.switchTheme(this.systemTheme);
        } else {
          this.switchTheme(val);
        }
      },
      systemTheme(val) {
        if (this.themeStore.theme === "default") {
          this.switchTheme(val);
        }
      },
    },
  };
</script>

<template>
  <div class="h-screen flex flex-col">
    <a-config-provider :theme="themeData">
      <a-layout>
        <a-layout-sider
          :style="{
            background: 'transparent',
          }"
          ><a-menu
            mode="inline"
            :style="{
              overflow: 'auto',
              height: '100vh',
              background: 'transparent',
            }">
            <a-sub-menu key="sub1">
              <template #title>
                <span>
                  <user-outlined />
                  subnav 1
                </span>
              </template>
              <a-menu-item key="1">option1</a-menu-item>
              <a-menu-item key="2">option2</a-menu-item>
              <a-menu-item key="3">option3</a-menu-item>
              <a-menu-item key="4">option4</a-menu-item>
            </a-sub-menu>
          </a-menu>
        </a-layout-sider>
        <a-layout>
          <a-layout style="padding: 0 10px 5px">
            <div class="flex justify-between" data-tauri-drag-region>
              <a-breadcrumb style="margin: 10px 0">
                <a-breadcrumb-item>Home</a-breadcrumb-item>
                <a-breadcrumb-item>List</a-breadcrumb-item>
                <a-breadcrumb-item>App</a-breadcrumb-item>
              </a-breadcrumb>
              <window_control />
            </div>

            <a-layout-content
              :style="{
                margin: 0,
                minHeight: '280px',
              }">
              <router-view />
            </a-layout-content>
          </a-layout>
        </a-layout>
      </a-layout>
    </a-config-provider>
    <!-- Debug -->
    <div class="debug-view">
      <!-- <p>Welcome to Tauri!</p>
      <p>Current theme: {{ themeStore.theme }}</p>
      <p>Current system theme: {{ systemTheme }}</p>
      <p>Current window focus: {{ isFocused }}</p> -->
      <p>{{ currentRoute.fullPath }}</p>
      <a-flex :vertical="false">
        <a-button @click="$router.push('/')">Home</a-button>
        <a-button type="text" @click="$router.push('/about')">About</a-button>
        <a-button type="text" @click="$router.push('/settings')"
          >Settings</a-button
        >
      </a-flex>
    </div>
  </div>
</template>

<style scoped lang="scss">
  .debug-view {
    /* right bottom fixed */
    position: fixed;
    bottom: 0;
    right: 0;
    /* padding */
    padding: 15px;
    margin: 15px;
    /* background */
    background: rgba(255, 255, 255, 0.479);
    /* border */
    border-radius: 10px;
    // drop shadow
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
  }
</style>
