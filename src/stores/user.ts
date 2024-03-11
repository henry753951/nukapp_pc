import { defineStore } from "pinia";

export const useUserStore = defineStore({
  id: "user",
  state: () => ({
    user: {
      isLoggedIn: false,
      UserData: {
        學號: "",
        姓名: "",
        系所: "",
        入學年度: "",
      },
      loginData: {
        username: "",
        password: "",
      },
    },
  }),
  persist: true,
});
