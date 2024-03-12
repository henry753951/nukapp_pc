<script lang="ts" setup>
  import { ref, reactive, watch } from "vue";
  import VueFeather from "vue-feather";
  import { storeToRefs } from "pinia";
  import { invoke } from "@tauri-apps/api";
  import { useGlobalStateStore } from "../../stores/GlobalState";
  import { useUserStore } from "../../stores/user";

  const GlobalStateStore = useGlobalStateStore();
  const UserStore = useUserStore();

  const { loginModal } = storeToRefs(GlobalStateStore);

  interface FormState {
    sid: string;
    password: string;
  }

  const formState = reactive<FormState>({
    sid: "",
    password: "",
  });

  const login = () => {
    console.log("Login");
    invoke("login", {
      username: formState.sid,
      password: formState.password,
    })
      .then((response) => {
        console.log(response);
        if (response) {
          let res = response as any;
          loginModal.value = false;
          UserStore.user.isLoggedIn = true;
          UserStore.user.loginData = {
            username: formState.sid,
            password: formState.password,
          };

          UserStore.user.UserData = {
            學號: res.student_id,
            姓名: res.name,
            系所: res.department,
            入學年度: res.admission_year,
          };
        }
      })
      .catch((err) => {
        console.error(err);
      });
  };
</script>

<template>
  <div class="absolute bottom-0">
    <a-modal
      v-model:open="loginModal"
      width="300px"
      :wrap-style="{ overflow: 'hidden' }"
      :footer="null">
      <a-flex gap="middle" vertical>
        <a-input size="large" v-model:value="formState.sid" placeholder="學號">
          <template #prefix>
            <VueFeather type="user" size="18" />
          </template>
        </a-input>
        <a-input-password
          size="large"
          v-model:value="formState.password"
          placeholder="密碼">
          <template #prefix>
            <VueFeather type="lock" size="18" />
          </template>
        </a-input-password>
        <button class="login-button" @click="login">登入</button>
      </a-flex>

      <template #title>
        <div style="width: 100%">登入</div>
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
