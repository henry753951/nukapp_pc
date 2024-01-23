import { defineStore } from "pinia";

export const useThemeStore = defineStore({
  id: "theme",
  state: () => ({
    theme: "default",
  }),
  persist: true,
});
