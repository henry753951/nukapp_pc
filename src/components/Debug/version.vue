<script setup lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/tauri";
  import { logger } from "../../logger";
  import { ref } from "vue";
import { router } from "../../router";
  const version = ref<string | null>(null);

  getVersion().then((v) => {
    version.value = v;
  });

  let clickCount = 0;
  const clickVersion = () => {
    clickCount++;
    if (clickCount === 5) {
      logger.debug("Hi there! You found me!");
      invoke("open_devtools");
      router.push("/dev");

      clickCount = 0;
    }
    setTimeout(() => {
      clickCount = 0;
    }, 2000);
  };
</script>

<template>
  <div class="fixed bottom-2 w-full flex justify-center" @click="clickVersion">
    v{{ version }}
  </div>
</template>
