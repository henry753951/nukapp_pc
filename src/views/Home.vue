<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import version from "../components/Debug/version.vue";
  import { router } from "../router";
  import { ref } from "vue";
</script>
<template>
  <div class="flex flex-col w-full h-full justify-center items-center">
    <div class="title pr-3">👋 NUK APP 2</div>
    <div class="subtitle">I am back!</div>
    <div class="">
      自從被伺服器被學校鎖掉IP後，現在只能以非網站的方式呈現T_T !
      <RouterLink class="ml-2" to="/QQ">詳情請見</RouterLink>
    </div>
    <a-flex gap="50" class="mt-5">
      <button class="button" @click="router.push('/course-selection')">
        模擬選課
      </button>
    </a-flex>
    <version />
  </div>
</template>

<style lang="scss" scoped>
  .title {
    font-size: 5rem;
    line-height: 1;
  }
  .subtitle {
    font-size: 2rem;
    line-height: 2;
  }
  .button {
    padding: 1rem 2rem;
    background-color: #c9cbcf6b;
    border: none;
    border-radius: 4px;
    font-size: 20px;
    font-weight: 800;
    line-height: 1.5;
    cursor: pointer;

    text-align: center;
    transition-duration: 0.3s;
    transition-property: color, background-color, border-color, box-shadow;
    -webkit-user-select: none;
    user-select: none;
    white-space: nowrap;
  }
</style>
