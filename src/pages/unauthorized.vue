<template>
  <div class="flex justify-center items-center h-full">
    <h1 class="text-4xl">請先登入</h1>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useUserStore } from "../stores/user";
definePageMeta({
  name: "Unauthorized",
  title: "未授權",
});

const params = useUrlSearchParams();

const userStore = useUserStore();

watch(
  () => userStore.user,
  (newVal) => {
    if (newVal.isLoggedIn) {
      navigateTo((params.nextUrl as string) || "/");
    }
  },
  { deep: true }
);
</script>
