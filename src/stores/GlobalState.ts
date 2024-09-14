import { defineStore } from "pinia";

export const useGlobalStateStore = defineStore({
  id: "GlobalState",
  state: () => ({
    loginModal: false,
    window: {
      isFocused: false,
    },
  }),
  persist: false,

  actions: {
    setLoginModal(value: boolean) {
      this.loginModal = value;
    },
  },
});
