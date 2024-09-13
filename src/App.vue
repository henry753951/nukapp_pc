<script lang="ts">
import { logger } from "./logger";

import { useRoute } from "vue-router";
import { h, provide, ref } from "vue";
import { theme } from "ant-design-vue";
import { useThemeStore } from "./stores/theme";
import { useUserStore } from "./stores/user";
import variables from "./variables";
// Components
import window_control from "./components/window_control.vue";
import LoginModal from "./components/modals/LoginModal.vue";
import Sidebar from "./components/Sidebar.vue";
// tauri
import { appWindow } from "@tauri-apps/api/window";
// import { platform } from "@tauri-apps/plugin-os";
import { router } from "./router";
// const platformName = await platform();
export default {
  components: {
    window_control,
    Sidebar,
    LoginModal,
  },
  data() {
    return {
      layoutContent: null as HTMLDivElement | null,
      debug: false,
      themeData: {
        algorithm: theme.defaultAlgorithm,
        light_token: {
          colorBgLayout: "#ffffff80",
          colorPrimaryBg: "#00000015",
          colorPrimary: "#808080",
          colorSiderBg: "#ffffff05",
        },
        dark_token: {
          colorBgLayout: "#00000080",
          colorPrimaryBg: "#ffffff15",
          colorPrimary: "#ffffff",
          colorSiderBg: "#00000005",
        },
        token: {
          colorBgLayout: "#ffffff80",
          colorPrimaryBg: "#00000015",
          colorPrimary: "#808080",
          colorSiderBg: "#ffffff05",
        },
      },
      mediaQueryList: null as MediaQueryList | null,
      systemTheme: "",
      currentTheme: "",
      isFocused: true,
      currentRoute: useRoute(),
    };
  },
  provide() {
    return {
      scrollToTop: this.scrollToTop,
    };
  },
  mounted() {
    // logger.info(`Platform: ${platformName}`);
    this.layoutContent = this.$refs.layoutContent as HTMLDivElement;
    appWindow.listen("tauri://focus", ({}) => {
      this.focused(true);
    });
    appWindow.listen("tauri://blur", ({}) => {
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
    useUserStore().invokeLogin();
  },
  unmounted() {
    this.mediaQueryList!.removeEventListener("change", () => {});
  },
  computed: {
    themeStore() {
      return useThemeStore();
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
    switchTheme(mode: string) {
      this.currentTheme = mode;
      for (let key of variables) {
        let value = `var(--${mode}-${key})`;
        document.documentElement.style.setProperty(`--${key}`, value);
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
  <div class="w-full h-1 fixed bg-transparent"></div>
  <div class="h-screen max-h-screen flex flex-col">
    <a-config-provider :theme="themeData">
      <a-layout>
        <a-layout-sider
          :style="{
            background: themeData.token.colorSiderBg,
          }"
          ><Sidebar />
        </a-layout-sider>
        <a-layout>
          <a-layout>
            <div
              class="flex justify-between px-2 shadow-sm"
              style="background-color: rgba(255, 255, 255, 0.06)"
              data-tauri-drag-region
            >
              <a-breadcrumb style="margin: 10px 0">
                <a-breadcrumb-item>Home</a-breadcrumb-item>
                <a-breadcrumb-item
                  v-for="text in currentRoute.meta.breadcrumb"
                  >{{ text }}</a-breadcrumb-item
                >
              </a-breadcrumb>
              <window_control />
            </div>

            <a-layout-content
              :style="{
                margin: 0,
                minHeight: '280px',
              }"
            >
              <div
                ref="layoutContent"
                style="height: 100%; overflow-y: overlay; overflow-x: hidden"
              >
                <router-view />
              </div>
            </a-layout-content>
          </a-layout>
        </a-layout>
      </a-layout>
      <LoginModal />
    </a-config-provider>
    <!-- Debug -->
    <div class="debug-view" v-if="debug">
      <!-- <p>Welcome to Tauri!</p>
      <p>Current theme: {{ themeStore.theme }}</p>
      <p>Current system theme: {{ systemTheme }}</p>
      <p>Current window focus: {{ isFocused }}</p> -->
      <p>{{ currentRoute.meta }}</p>
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
