import { defineStore } from "pinia";
import { CustomData } from "../interface";

export const useCustomCourseDataStore = defineStore({
  id: "CustomCourseData",
  state: () => ({
    CustomCourseData: {} as Record<string, CustomData>,
  }),
  getters: {
    getCustomCourseData: (state) => (key: string) => {
      if (!state.CustomCourseData[key]) {
        return {
          course_name: null,
          location: null,
          teacher: null,
          time: null,
          requirement_type: null,
          students_note: "",
          color: "",
        } as CustomData;
      }
      return state.CustomCourseData[key];
    },
  },
  persist: true,
});
