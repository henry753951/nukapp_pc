<template>
  <div class="sidebar">
    <div my-2>
      <span class="flex-center gap-1 px-2 py-2">
        <span class="text-xl font-semibold">
          NUK App
        </span>
      </span>
    </div>
    <!-- 使用 PanelMenu 替換 Menu -->
    <PanelMenu
      :model="menuItems"
      class="w-full h-full"
      :style="menuStyle"
      :multiple="true"
    >
      <template #item="{ item }">
        <a
          v-ripple
          class="flex items-center px-4 py-2 cursor-pointer group mt-1"
        >
          <Icon :name="item.icon ? item.icon : ''" />
          <span :class="['ml-2', { 'font-semibold': item.items }]">{{
            item.label
          }}</span>
          <Badge v-if="item.badge" class="ml-auto" :value="item.badge" />
          <span
            v-if="item.shortcut"
            class="ml-auto border border-surface rounded bg-emphasis text-muted-color text-xs p-1"
            >{{ item.shortcut }}</span
          >
        </a>
      </template>
    </PanelMenu>

    <!-- 用戶信息 -->
    <div px-2 py-1>
      <div
        class="w-full border-0 bg-transparent flex items-start my-2 p-2 pl-4 hover:bg-#f8fafc dark:hover:bg-warmgray dark:hover:bg-surface-800 rounded-xl cursor-pointer transition-colors duration-200"
        @click="showLoginModal"
      >
        <ModalsLoginModal />
        <span class="inline-flex flex-col items-start" v-if="user.isLoggedIn">
          <span class="font-bold">{{ user.UserData.姓名 }}</span>
          <span class="text-sm">{{ user.UserData.系所 }}</span>
        </span>
        <div v-else flex flex-center gap-4>
          <Icon name="hugeicons:login-01" size="1.7rem" />
          <span class="inline-flex flex-col items-start">
            <span class="font-bold">{{ $t("user.login.title") }}</span>
            <span class="text-sm">{{ $t("user.login.description") }}</span>
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";

const { t } = useI18n(); // 使用 i18n 函數

const menuStyle = ref({
  "--p-panelmenu-panel-border-width": "0",
  "--p-panelmenu-panel-first-border-width": "0",
  "--p-panelmenu-panel-last-border-width": "0",
  "--p-panelmenu-gap": "0px",
  "--p-panelmenu-item-gap": "22px",
  "--p-panelmenu-panel-background": "transparent",
  "--p-panelmenu-item-focus-background": "var(--highlight-color-1)",
});

// 使用 Pinia store
const userStore = useUserStore();
const globalStateStore = useGlobalStateStore();
const { user } = storeToRefs(userStore);

// 定義菜單項目
const menuItems = ref([
  {
    label: t("menu.home"), // 使用 t 函數
    icon: "hugeicons:home-05",
    command: () => navigateTo("/"),
  },
  {
    label: t("menu.course_system"),
    icon: "hugeicons:course",
    items: [
      {
        label: t("menu.course_search"),
        icon: "hugeicons:search-list-01",
        command: () =>
          navigateTo({
            name: "CourseView",
          }),
      },
      { label: t("menu.select_course"), icon: "hugeicons:select-01" },
    ],
  },
  {
    label: t("menu.profile"),
    icon: "hugeicons:profile-02",
    items: [
      {
        label: t("menu.score_view"),
        icon: "hugeicons:chart-column",
        command: () => navigateTo({ name: "ScoreView" }),
      },
      { label: t("menu.timetable_view"), icon: "hugeicons:calendar-01" },
      { label: t("menu.credit_analysis"), icon: "hugeicons:champion" },
    ],
  },
]);

const showLoginModal = () => {
  globalStateStore.setLoginModal(true);
};
</script>

<style scoped lang="scss">
.sidebar {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
.user-card {
  margin-top: auto;
  margin-bottom: 12px;
  margin-left: 12px;
  margin-right: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  background-color: var(--highlight-color);
  border-radius: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
}

.title {
  font-size: 1.5rem;
}

.collapsed-menu {
  display: none;
}
</style>
