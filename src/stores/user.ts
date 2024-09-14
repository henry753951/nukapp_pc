import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

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
    async invokeLogin(
      _username: string | undefined = undefined,
      _password: string | undefined = undefined
    ) {
      const { $i18n } = useNuxtApp();
      let username = _username || this.user.loginData.username;
      let password = _password || this.user.loginData.password;

      if (username === "" || password === "") {
        throw new Error($i18n.t("user.login.error.empty"));
      }

      let response = await invoke("login", {
        username: username,
        password: password,
      });

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
        return true;
      } else {
        throw new Error($i18n.t("user.login.error.failed"));
      }
    },

    async invokeLogout() {
      const { $i18n } = useNuxtApp();
      const logger = useLogger("userStore");
      const route = useRoute();
      navigateTo({ path: route.path });
      try {
        let response = await invoke("logout");
        const route = useRoute();

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

          if ((route.meta.middleware as string[])?.includes("auth")) {
            navigateTo({
              name:"Unauthorized",
              query: {
                nextUrl: route.fullPath,
              },
            });
          }

          return true;
        } else {
          throw new Error($i18n.t("user.logout.error.failed"));
        }
      } catch (err) {
        logger.error(err);
        throw err;
      }
    },
  },
});
