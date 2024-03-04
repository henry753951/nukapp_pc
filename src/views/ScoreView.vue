<script setup lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/tauri";
  import { logger } from "../logger";
  import { ref } from "vue";

</script>

<template>
    <div>成績</div>
</template>
