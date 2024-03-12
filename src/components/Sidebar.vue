<template>
  <div class="sidebar">
    <a-flex align="center" justify="center" gap="10">
      <h1 class="title" @click="GoHome">NUK 2</h1>
    </a-flex>
    <a-menu
      v-model:selectedKeys="currentPage"
      v-model:openKeys="Sider.openKeys"
      :inline-collapsed="Sider.collapsed"
      @click="changePage"
      mode="inline"
      :style="{
        overflow: 'overlay',
        background: 'transparent',
      }">
      <a-sub-menu key="sub1">
        <template #title>
          <a-flex align="center" gap="5">
            <vue-feather type="book-open" size="18"></vue-feather>
            課程系統
          </a-flex>
        </template>
        <a-menu-item key="/course-selection">課程查詢</a-menu-item>
        <a-menu-item key="/選課系統">選課系統</a-menu-item>
      </a-sub-menu>
      <a-sub-menu key="sub2">
        <template #title>
          <a-flex align="center" gap="5">
            <vue-feather type="user" size="18"></vue-feather>
            個人資訊
          </a-flex>
        </template>
        <a-menu-item key="/score">成績查詢</a-menu-item>
        <a-menu-item key="/學分分析">學分分析</a-menu-item>
      </a-sub-menu>
    </a-menu>
    <div class="user-card" @click="showLoginModal">
      <div class="user-details">
        <h3>{{user.UserData.姓名}}</h3>
        <p>{{user.UserData.系所}}</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { router } from "../router";
  import { useRoute } from "vue-router";
  import { useGlobalStateStore } from "../stores/GlobalState";
  import { useUserStore } from "../stores/user";
  import { storeToRefs } from "pinia";

  export default {
    components: {},
    setup() {
      const userStore = useUserStore();
      const GlobalStateStore = useGlobalStateStore();
      const { user } = storeToRefs(userStore);
      return { userStore, GlobalStateStore ,user};
    },
    data() {
      return {
        Sider: {
          openKeys: ["sub1", "sub2"],
          collapsed: false,
        },
        currentPage: ["/"],
        menuItems: [
          {
            text: "首頁",
            icon: "home-icon",
            isOpen: false,
          },
          {
            text: "課程系統",
            icon: "education-icon",
            isOpen: false,
            children: ["課程查詢", "選課系統"],
          },
          {
            text: "個人資訊",
            icon: "user-icon",
            isOpen: false,
            children: ["成績查詢", "學分分析"],
          },
        ],
      };
    },
    computed: {
      currentRoute() {
        return useRoute();
      },
    },
    methods: {
      changePage({ key }: { key: string }) {
        console.log(`Change page to ${key}`);
        router.push(key);
      },
      showLoginModal() {
        this.GlobalStateStore.setLoginModal(true);
      },
      GoHome() {
        router.push("/");
      },
    },
    watch: {
      "currentRoute.fullPath"(val: string) {
        this.currentPage = [val];
      },
    },
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

  .user-image {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    margin-right: 10px;
    background-color: #eaeaea;
  }

  .user-details {
    h3 {
      margin: 0;
      color: var(--base-color);
    }
    p {
      margin: 0;
      color: var(--base-color-0);
    }
  }

  .title {
    font-size: 1.5rem;
  }
</style>

<style lang="scss">
  .ant-menu-light {
    .ant-menu-submenu-selected > .ant-menu-submenu-title {
      color: var(--base-color);
    }
  }
</style>
