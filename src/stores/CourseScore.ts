import { defineStore } from "pinia";
import type { StudentGrades } from "~/types/types";

export const useCourseScoreStore = defineStore({
  id: "CourseScoreStore",
  state: () => ({
    studentGrades: null as StudentGrades | null,
  }),
  actions: {},
});
