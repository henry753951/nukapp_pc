<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";

  // themeSwitch
  import { Ref, ref, onMounted, watch } from "vue";
  import { useThemeStore } from "../stores/theme";
  const themeStore = useThemeStore();
  const themeSwitchRef = ref(themeStore.theme);
  const options = [
    { value: "default", label: "跟隨系統" },
    { value: "dark", label: "深色" },
    { value: "light", label: "亮色"}
  ];

  watch(themeSwitchRef, (val) => {
    themeStore.theme = val;
  });
</script>

<template>
  <div class="flex items-center" data-tauri-drag-region>
    <!--  -->
    <div class="pr-2">
      <a-select
        v-model:value="themeSwitchRef"
        style="width: 120px"
        :options="options" />
    </div>
    <!--  -->
    <div class="flex gap-1 my-auto ml-auto justify-center window-control">
      <div @click="appWindow.minimize()" class="box">
        <vue-feather type="minus" size="15"></vue-feather>
      </div>
      <div @click="appWindow.toggleMaximize()" class="box">
        <vue-feather type="square" size="15"></vue-feather>
      </div>
      <div @click="appWindow.close()" class="box">
        <vue-feather type="x" size="15"></vue-feather>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  * {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
  .box {
    border-radius: 0.25rem;
    background-color: transparent;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transition: background-color 0.2s ease-in-out;
    padding: 5px;
    &:hover {
      background-color: rgba(255, 255, 255, 0.3);
    }
    &:last-child:hover {
      background-color: rgba(255, 0, 0, 0.4);
    }
  }
</style>
