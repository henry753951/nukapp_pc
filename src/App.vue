<script lang="ts">
  import { logger } from "./logger";

  import { useRoute } from "vue-router";
  import { provide, ref } from "vue";
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
        layoutContent: null as HTMLDivElement | null,
        debug: false,
        Sider: {
          openKeys: ["sub1", "sub2"],
          collapsed: false,
        },
        currentPage: ["/"],
        themeData: {
          algorithm: theme.defaultAlgorithm,
          light_token: {
            colorBgLayout: "#ffffff50",
            colorPrimaryBg: "#00000015",
            colorPrimary: "#141414",
            colorSiderBg: "#ffffffc5",
          },
          dark_token: {
            colorBgLayout: "#00000030",
            colorPrimaryBg: "#ffffff15",
            colorPrimary: "#ffffff",
            colorSiderBg: "#000000c5",
          },
          token: {
            colorBgLayout: "#ffffff50",
            colorPrimaryBg: "#00000015",
            colorPrimary: "#141414",
            colorSiderBg: "#ffffffc5",
          },
        },
        mediaQueryList: null as MediaQueryList | null,
        systemTheme: "",
        currentTheme: "",
        isFocused: true,
      };
    },
    provide() {
      return {
        scrollToTop: this.scrollToTop,
      };
    },
    mounted() {
      logger.info(`Platform: ${platformName}`);
      this.layoutContent = this.$refs.layoutContent as HTMLDivElement;
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
      scrollToTop() {
        logger.info("Scroll to top");
        if (this.layoutContent !== null) {
          this.layoutContent!.scrollTop = 0;
        }
      },
      focused(focus: boolean) {
        this.isFocused = focus;
        if (!focus) {
          if (this.currentTheme === "dark") {
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
      changePage({ key }: { key: string }) {
        console.log(`Change page to ${key}`);
        router.push(key);
      },
      switchTheme(mode: string) {
        this.currentTheme = mode;
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
      "currentRoute.fullPath"(val) {
        this.currentPage = [val];
      },
    },
  };
</script>

<template>
  <div class="h-screen max-h-screen overflow-hidden flex flex-col">
    <a-config-provider :theme="themeData">
      <a-layout>
        <a-layout-sider
          :style="{
            background: themeData.token.colorSiderBg,
          }"
          ><a-menu
            v-model:selectedKeys="currentPage"
            v-model:openKeys="Sider.openKeys"
            :inline-collapsed="Sider.collapsed"
            @click="changePage"
            mode="inline"
            :style="{
              overflow: 'auto',
              background: 'transparent',
            }">
            <a-menu-item key="/">
              <a-flex align="center" gap="10">
                <vue-feather type="home" size="20"></vue-feather>
                <h1>NUK 2</h1>
              </a-flex>
            </a-menu-item>
            <a-sub-menu key="sub1">
              <template #title>
                <a-flex align="center" gap="5">
                  <vue-feather type="book-open" size="18"></vue-feather>
                  課程系統
                </a-flex>
              </template>
              <a-menu-item key="/course-selection">課程查詢</a-menu-item>
              <a-menu-item key="/選課系統">選課系統</a-menu-item>
            </a-sub-menu>
            <a-sub-menu key="sub2">
              <template #title>
                <a-flex align="center" gap="5">
                  <vue-feather type="user" size="18"></vue-feather>
                  個人資訊
                </a-flex>
              </template>
              <a-menu-item key="/成績查詢">成績查詢</a-menu-item>
              <a-menu-item key="/學分分析">學分分析</a-menu-item>
            </a-sub-menu>
          </a-menu>
        </a-layout-sider>
        <a-layout>
          <a-layout>
            <div class="flex justify-between px-2" data-tauri-drag-region>
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
              <div
                ref="layoutContent"
                class="flex flex-col"
                :style="{ overflow: 'auto', height: '100%' }">
                <router-view />
              </div>
            </a-layout-content>
          </a-layout>
        </a-layout>
      </a-layout>
    </a-config-provider>
    <!-- Debug -->
    <div class="debug-view" v-if="debug">
      <!-- <p>Welcome to Tauri!</p>
      <p>Current theme: {{ themeStore.theme }}</p>
      <p>Current system theme: {{ systemTheme }}</p>
      <p>Current window focus: {{ isFocused }}</p> -->
      <p>{{ currentRoute.meta }}</p>
      <p>{{ currentPage }}</p>
      <a-button @click="scrollToTop">TT</a-button>
      <a-flex :vertical="false">
        <a-button @click="$router.push('/')">Home</a-button>
        <a-button type="text" @click="$router.push('/about')"> About </a-button>
        <a-button type="text" @click="$router.push('/settings')">
          Settings
        </a-button>
        <a-button type="text" @click="$router.push('/course-selection')">
          course-selection
        </a-button>
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
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
  }
</style>
