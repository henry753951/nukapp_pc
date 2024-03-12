<template>
  <div class="flex justify-center items-center h-full">
    <h1 class="text-4xl">請先登入</h1>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref, watch } from "vue";
  import { useUserStore } from "../stores/user";
  import { invoke } from "@tauri-apps/api/tauri";
  import { router } from "../router";

  const nextUrl = ref<string>("/");
  onMounted(() => {
    nextUrl.value = router.currentRoute.value.query.nextUrl as string;
  });

  const userStore = useUserStore();

  watch(
    () => userStore.user,
    (newVal) => {
      if (newVal.isLoggedIn) {
        router.push(nextUrl.value);
      }
    },
    { deep: true }
  );
</script>
