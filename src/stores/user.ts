import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";
import { router } from "../router";

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
    invokeLogin(
      _username: string | undefined = undefined,
      _password: string | undefined = undefined
    ) {
      let username = _username || this.user.loginData.username;
      let password = _password || this.user.loginData.password;

      if (username === "" || password === "") {
        return;
      }
      invoke("login", { username: username, password: password })
        .then((response) => {
          if (response) {
            let res = response as any;
            this.user.UserData = {
              學號: res["student_id"],
              姓名: res["name"],
              系所: res["department"],
              入學年度: res["admission_year"],
            };
            this.user.loginData = {
              username: username,
              password: password,
            };
            this.user.isLoggedIn = true;
          }
        })
        .catch((err) => {
          console.error(err);
        });
    },
    invokeLogout() {
      invoke("logout").then((response) => {
        console.log(response);
        if (response) {
          this.user.isLoggedIn = false;
          this.user.UserData = {
            學號: "",
            姓名: "",
            系所: "",
            入學年度: "",
          };
          this.user.loginData = {
            username: "",
            password: "",
          };
          if (router.currentRoute.value.meta.needAuth) {
            router.push({
              path: "/unauthorized",
              query: {
                nextUrl: router.currentRoute.value.fullPath,
              },
            });
          }
        }
      });
    },
  },
});
