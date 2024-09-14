import { defineStore } from "pinia";

export const useSelectedCourseStore = defineStore({
  id: "selectedCourse",
  state: () => ({
    selectedCourseKeys: [] as String[],
    lastUpdated: new Date().getTime(),
  }),
  persist: true,
});
