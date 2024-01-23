<script setup lang="ts">
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
  <div>
    <a-select v-model:value="themeSwitchRef" style="width: 120px" :options="options" />
  </div>
</template>
