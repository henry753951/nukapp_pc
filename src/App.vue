<template>
  <Html min-h-screen>
    <Body overflow-x-hidden text-white>
      <div class="fixed h-1 w-full bg-transparent" />
      <MessageToast />
      <ConfirmDialog />
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </Body>
  </Html>
</template>

<script setup lang="ts">
import { usePreset } from "@primevue/themes";
import { NUKAppPreset } from "../primevue.theme";

onMounted(() => {
  usePreset(NUKAppPreset);
});


</script>

<style lang="scss">
// Transitions
.page-enter-active,
.page-leave-active {
  @apply transition-opacity ease-in-out duration-300;
}
.layout-enter-active,
.layout-leave-active {
  @apply transition-opacity ease-in-out duration-500;
}
.page-enter-from,
.page-leave-to,
.layout-enter-from,
.layout-leave-to {
  @apply opacity-0;
}
</style>
