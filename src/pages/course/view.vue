<template>
  <div class="flex flex-col h-full">
    <Splitter
      v-model:percent="limitedPercent"
      :hide-text="{
        'left-pane': '課程清單',
      }"
      :hide-when-drag="{
        'left-pane': true,
      }"
    >
      <template #left-pane>
        <CourseList
          :courses_data="course_data?.course_ls"
          :last-update-time="course_data?.updateTime || ''"
          :loading="status == 'pending'"
          @refresh="refreshCourseData"
        />
      </template>
      <template #right-pane>
        <div
          class="h-full overflow-y-overlay flex flex-col items-center"
          style="scrollbar-gutter: stable both-edges"
        >
          <CourseTable
            :course-list="selectedCourses"
            @on-course-click="showCourseModal"
            @on-course-clear="clearSelectdCourse"
          />
        </div>
      </template>
    </Splitter>
    <a-drawer
      :title="CheckCourseModalData.course.course_name"
      placement="right"
      :closable="true"
      :open="CheckCourseModalData.open"
      :get-container="true"
      @close="onCloseCourseModal"
    >
      <CourseDetail :course="CheckCourseModalData.course" />
    </a-drawer>
  </div>
</template>
<script lang="ts" setup>
import Splitter from "~/components/Splitter.vue";
import type { BaseCourse, Course } from "~/types/interface";
import { invoke } from "@tauri-apps/api/core";

definePageMeta({
  name: "CourseView",
  title: "課程查詢",
  description: "查詢所有課程",
  keepalive: true,
  breadcrumb: [
    { text: "課程系統", path: "" },
    { text: "課程查詢", path: "/course/view" },
  ],
});

const logger = useLogger("CourseView");
logger.info("is initialized");

let isRenew = false;
const refreshCourseData = () => {
  isRenew = true;
  refresh();
};
const {
  data: course_data,
  refresh,
  status,
} = useLazyAsyncData("courses_data", async () => {
  let jsondata = (await invoke("get_all_course", {
    refresh: isRenew,
  })) as { course_ls: BaseCourse[]; updateTime: string };
  isRenew = false;
  return jsondata;
});

const SelectedCourseStore = useSelectedCourseStore();
const selectedCourses = computed(() => {
  if (!course_data.value) return [];
  return course_data.value?.course_ls.filter((course) =>
    SelectedCourseStore.selectedCourseKeys.includes(course.key)
  );
});

const CheckCourseModalData = reactive({
  course: {} as BaseCourse,
  open: false,
});

// Methods
const showCourseModal = (record: BaseCourse) => {
  if (record.key) {
    logger.info("Show Course Modal");
    console.log(record);
    CheckCourseModalData.open = true;
    CheckCourseModalData.course = record;
  }
};
const onCloseCourseModal = () => {
  CheckCourseModalData.open = false;
  // CheckCourseModalData.course = {} as BaseCourse;
};
const clearSelectdCourse = () => {
  SelectedCourseStore.selectedCourseKeys = [];
  return;
};

// Splitter
const percent = ref(50);
const limitedPercent = computed({
  get() {
    return percent.value;
  },
  set(val) {
    percent.value = Math.max(20, Math.min(80, val));
  },
});

onMounted(() => {});
</script>
