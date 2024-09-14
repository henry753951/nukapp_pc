import { invoke } from "@tauri-apps/api/core";
import type { StudentGrades } from "~/types/types";

export const useCourseScore = (onLoad: () => void = () => {}) => {
  const logger = useLogger("User");
  const courseScoreStore = useCourseScoreStore();
  const userStore = useUserStore();
  const { refresh, status } = useLazyAsyncData("CourseScore", async () => {
    onLoad();
    const data = (await invoke("get_user_score")) as StudentGrades;
    courseScoreStore.studentGrades = data;
    return data;
  });
  const studentGrades = computed(() => courseScoreStore.studentGrades);

  watch(
    () => userStore.user.UserData,
    (oldVal,newVal ) => {
      if (newVal.學號 != oldVal.學號) {
        logger.info("User changed", newVal.學號, "to", oldVal.學號);
        refresh();
      }
    },
    { deep: true }
  );

  return { studentGrades, refresh, status };
};
