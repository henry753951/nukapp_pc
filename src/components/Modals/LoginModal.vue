<script lang="ts" setup>
import { ref, reactive, watch } from "vue";
import VueFeather from "vue-feather";
import { storeToRefs } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "../../stores/GlobalState";
import { useUserStore } from "../../stores/user";
const logger = useLogger("LoginModal");
const GlobalStateStore = useGlobalStateStore();
const UserStore = useUserStore();
const toast = useToast();
const { loginModal } = storeToRefs(GlobalStateStore);
const { user } = storeToRefs(UserStore);
const notification = useNotification();

interface FormState {
  sid: string;
  password: string;
}

const formState = reactive<FormState>({
  sid: "",
  password: "",
});

const login = async () => {
  UserStore.invokeLogin(formState.sid, formState.password)
    .then((res) => {
      logger.info("Login result", res);
      if (res) {
        notification.add({
          severity: "success",
          summary: "登入成功",
          life: 3000,
        });
      } else {
        throw new Error("登入失敗");
      }
    })
    .catch((err) => {
      logger.error("Login error", err);
      notification.add({
        severity: "error",
        summary: "登入失敗",
        detail: err.message,
        life: 3000,
      });
    });
};

const logout = () => {
  UserStore.invokeLogout();
};
</script>

<template>
  <div class="absolute bottom-0">
    <a-modal
      v-model:open="loginModal"
      width="300px"
      :wrap-style="{ overflow: 'hidden' }"
      :footer="null"
    >
      <a-flex gap="middle" vertical>
        <template v-if="user.isLoggedIn">
          <a-space direction="vertical">
            <a-space direction="vertical">
              <span>{{ user.UserData.姓名 }}</span>
            </a-space>
            <a-space direction="vertical">
              <span>{{ user.UserData.學號 }}@mail.nuk.edu.tw</span>
              <span>{{ user.UserData.系所 }}</span>
            </a-space>
            <a-space direction="horizontal">
              <span strong>入學年度:</span>
              <span>{{ user.UserData.入學年度 }}</span>
            </a-space>
            <Button w-full @click="logout"> 登出 </Button>
          </a-space>
        </template>
        <template v-else>
          <a-input
            size="large"
            v-model:value="formState.sid"
            placeholder="學號"
          >
            <template #prefix>
              <VueFeather type="user" size="18" />
            </template>
          </a-input>
          <a-input-password
            size="large"
            v-model:value="formState.password"
            @keyup.enter="login"
            placeholder="密碼"
          >
            <template #prefix>
              <VueFeather type="lock" size="18" />
            </template>
          </a-input-password>
          <Button @click="login">登入</Button>
        </template>
      </a-flex>

      <template #title>
        <div v-if="user.isLoggedIn" style="width: 100%">帳號</div>
        <div v-else style="width: 100%">登入</div>
      </template>
    </a-modal>
  </div>
</template>

<style lang="scss" scoped>
.login-button {
  background-color: #000000;
  border: none;
  color: white;
  padding: 10px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  cursor: pointer;
  border-radius: 12px;
  width: 100%;
  transition-duration: 0.3s;
  &:hover {
    background-color: #242424;
    color: white;
  }
  &:active {
    background-color: #727272;
    color: white;
  }
}
</style>
