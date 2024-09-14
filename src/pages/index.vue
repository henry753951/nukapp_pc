<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
</script>

<template>
  <div class="flex flex-col w-full h-full justify-center items-center">
    <div class="title pr-3">👋 NUK APP 2</div>
    <div class="subtitle">I am back!</div>
    <div class="">
      自從被伺服器被學校鎖掉IP後，現在只能以非網站的方式呈現T_T !
      <Hyperlink
        block
        class="ml-2"
        :to="{
          path: '/md',
          query: {
            sourceURL:
              'https://raw.githubusercontent.com/henry753951/nukapp_pc/main/md/QQ.md',
            breadcrumb: '為什麼一代不能使用了',
          },
        }"
        >詳情請見</Hyperlink
      >
    </div>
    <a-flex gap="50" class="mt-5">
      <Button @click="navigateTo({ name: 'CourseView' })"> 模擬選課 </Button>
    </a-flex>
    <DebugVersion />
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
</style>
