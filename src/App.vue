<script lang="ts">
  import { useRoute } from "vue-router";
  import { theme } from "ant-design-vue";
  import { useThemeStore } from "./stores/theme";

  // tauri
  import { invoke } from "@tauri-apps/api/tauri";

  export default {
    data() {
      return {
        themeData: {
          algorithm: theme.defaultAlgorithm,
          token: {},
        },
        mediaQueryList: null as MediaQueryList | null,
        systemTheme: "",
      };
    },
    mounted() {
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
      switchTheme(mode: string) {
        let variables = {
          "--color-surface": mode === "dark" ? "#00000059" : "#ffffff59",
          "--base-color": mode === "dark" ? "#ffffff" : "#000000",
        };
        for (let [key, value] of Object.entries(variables)) {
          document.documentElement.style.setProperty(key, value);
        }
        this.themeData = {
          algorithm:
            mode === "dark" ? theme.darkAlgorithm : theme.defaultAlgorithm,
          token: { ...this.themeData.token },
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
  <a-config-provider :theme="themeData">
    <!-- window control -->

    <div class="container">
      <h1>Welcome to Tauri! {{ currentRoute.fullPath }}</h1>
      <a-flex :vertical="false">
        <a-button @click="$router.push('/')">Home</a-button>
        <a-button type="text" @click="$router.push('/about')">About</a-button>
        <a-button type="text" @click="$router.push('/settings')"
          >Settings</a-button
        >
      </a-flex>
      <router-view />
    </div>
  </a-config-provider>
</template>

<style scoped></style>
