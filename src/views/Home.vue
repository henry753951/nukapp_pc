<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const test = async () => {
    let str = await invoke("get_all_course");
    console.log(str);
  };
</script>
<template>
  <div>
    <a-button @click="test">Test</a-button>
  </div>
</template>
