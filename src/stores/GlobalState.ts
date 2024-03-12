import { defineStore } from "pinia";

export const useGlobalStateStore = defineStore({
  id: "GlobalState",
  state: () => ({
    loginModal : false,
  }),
  persist: false,

  actions: {
    setLoginModal(value: boolean) {
      this.loginModal = value;
    },
  },
});
