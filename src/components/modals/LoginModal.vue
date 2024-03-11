<script lang="ts" setup>
  import { ref, reactive } from "vue";
  import VueFeather from "vue-feather";
  import { useGlobalStateStore }  from "../../stores/GlobalState";
  const { loginModal } = useGlobalStateStore();

  interface FormState {
    sid: string;
    password: string;
  }

  const formState = reactive<FormState>({
    sid: "",
    password: "",
  });
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
        <a-input-password size="large" v-model:value="formState.password" placeholder="密碼">
          <template #prefix>
            <VueFeather type="lock" size="18" />
          </template>
        </a-input-password>
        <button class="login-button">登入</button>
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
