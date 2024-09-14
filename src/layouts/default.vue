<template>
  <div class="h-screen max-h-screen flex flex-col">
    <a-config-provider :theme="themeData">
      <a-layout>
        <a-layout-sider
          width="250"
          :style="{
            background: themeData.token.colorSideBg,
          }"
        >
          <NavSide />
        </a-layout-sider>
        <a-layout>
          <a-layout>
            <div
              class="flex justify-between px-2 shadow-sm"
              data-tauri-drag-region
            >
              <a-breadcrumb style="margin: 10px 0">
                <a-breadcrumb-item @click="navigateTo('/')" cursor-pointer>
                  Home
                </a-breadcrumb-item>
                <a-breadcrumb-item
                  v-for="(item, index) in breadcrumb"
                  :key="index"
                  @click="item.path ? navigateTo(item.path) : null"
                  :class="item.path ? 'cursor-pointer' : ''"
                >
                  {{ item.text }}
                </a-breadcrumb-item>
              </a-breadcrumb>
              <windowControl />
            </div>

            <a-layout-content :style="{ margin: 0, minHeight: '280px' }">
              <div
                ref="layoutContent"
                class="h-full overflow-x-hidden overflow-y-auto"
              >
                <NuxtPage />
              </div>
            </a-layout-content>
          </a-layout>
        </a-layout>
      </a-layout>
    </a-config-provider>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from "~/stores/user";
const currentRoute = useRoute();
const { themeData } = useTheme();

const breadcrumb = computed(() => {
  const data = currentRoute.meta.breadcrumb as {
    text: string;
    path: string;
  }[];
  return data;
});

onMounted(() => {
  nextTick(() => {
    useUserStore().invokeLogin();
  });
});

onUnmounted(() => {});
</script>
