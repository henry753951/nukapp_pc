import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

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
  actions: {
    invokeLogin() {
      if (this.user.loginData.username === "" || this.user.loginData.password === "") {
        return;
      }
      invoke("login", this.user.loginData).then((response) => {
        if (response) {
          let res = response as any;
          this.user.UserData = {
            學號: res["student_id"],
            姓名: res["name"],
            系所: res["department"],
            入學年度: res["admission_year"],
          
          };
          this.user.isLoggedIn = true;
        }
      });
    },
  },
});
