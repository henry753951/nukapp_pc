<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n"; // 引入 i18n 模組

import type {
  Semester,
  Course,
  SchoolYear,
  StudentGrades,
  CourseProgress,
} from "~/types/types";

const { t } = useI18n();

const logger = useLogger("ScoreView");
definePageMeta({
  name: "ScoreView",
  title: "成績查詢",
  middleware: ["auth"],
  breadcrumb: [
    { text: "個人資訊", path: "" },
    { text: "成績查詢", path: "/user/score" },
  ],
  keepalive: true,
});

const userStore = useUserStore();

const { studentGrades, refresh, status } = useCourseScore(() => {
  logger.info("Refreshing student grades");
  tab.value = current.value;
});

const tab = ref(0);

const current = computed(() => {
  if (studentGrades.value) {
    return studentGrades.value.各學期.length - 1;
  }
  return 0;
});

const Steps = computed(() => {
  let total = studentGrades.value
    ? studentGrades.value.各學期.length > 8
      ? studentGrades.value.各學期.length
      : 8
    : 8;
  let steps = [];
  if (studentGrades.value) {
    if (studentGrades.value.各學期.length > 0) {
      let start = studentGrades.value?.各學期[0].學期;
      const semester = start.match(/(\d+)學年度第\s(\d+)\s學期/);
      let year = semester ? parseInt(semester[1]) : 0;

      for (let i = 0; i < total; i++) {
        steps.push({
          title: `${year} ${t("year")}`, // "學年度" -> 使用翻譯
          description: `${t("semester", { number: (i % 2) + 1 })}`, // "第...學期" -> 使用翻譯
          status:
            i > current.value
              ? "wait"
              : i === current.value
              ? "process"
              : "finish",
        });
        if (i % 2 === 1) {
          year++;
        }
      }
    }
  }
  return steps;
});

const courses = computed((): Semester[] => {
  let result = [];
  if (studentGrades.value) {
    for (let semester of studentGrades.value.各學期) {
      let index = studentGrades.value.各學期.indexOf(semester);
      if (tab.value == -1) {
      } else {
        if (index !== tab.value) continue;
      }

      let temp = {
        學期: semester.學期,
        修習學分數: semester.修習學分數,
        實得學分數: semester.實得學分數,
        平均成績: semester.平均成績,
        排名: semester.排名,
        選修課程: semester.課程.filter((c) => c.修別 === "選修"),
        必修課程: semester.課程.filter((c) => c.修別 === "必修"),
      };
      result.push(temp);
    }
  }
  return result;
});

const Progress = (semester: Semester): CourseProgress => {
  let result = {
    選修: {
      total: 0,
      finished: 0,
    },
    必修: {
      total: 0,
      finished: 0,
    },
  } as CourseProgress;
  if (studentGrades.value) {
    for (let course of semester.選修課程) {
      let credit = parseInt(course.學分數);
      result.選修.total += credit;
      if (course.學期成績 !== "未送") {
        // "未送"
        if (parseInt(course.學期成績) >= 60) result.選修.finished += credit;
      }
    }
    for (let course of semester.必修課程) {
      let credit = parseInt(course.學分數);
      result.必修.total += credit;
      if (course.學期成績 !== "未送") {
        // "未送"
        if (parseInt(course.學期成績) >= 60) result.必修.finished += credit;
      }
    }
  }
  return result;
};

onMounted(() => {});
</script>

<template>
<div v-if="userStore.user.isLoggedIn" h-full>
    <template v-if="status !== 'pending'">
      <a-flex align="start">
        <div
          class="p-4"
          style="
            min-width: 220px;
            position: sticky;
            top: 0;
            align-self: flex-start;
          "
        >
          <div
            class="w-full p-3 rounded-lg my-2 cursor-pointer all-semesters"
            @click="tab = -1"
            :class="{ active: tab == -1 }"
          >
            {{ $t("menu.all_semesters") }}
          </div>
          <a-steps
            progress-dot
            v-model:current="tab"
            direction="vertical"
            :items="Steps"
          ></a-steps>
          <Button @click="refresh()" text w-full mt-auto>
            {{ $t("actions.refresh") }}
          </Button>
        </div>
        <div class="w-full">
          <div class="score-view p-4">
            <a-card v-if="tab == -1">
              <span style="font-size: x-large; font-weight: bold">
                {{ $t("menu.history_scores") }}
              </span>
              <a-descriptions bordered>
                <a-descriptions-item :label="$t('score.rank')">
                  {{ studentGrades!.歷年全班排名 }}
                </a-descriptions-item>
                <a-descriptions-item :label="$t('score.total_credits_earned')">
                  {{ studentGrades!.歷年實得學分數 }}
                </a-descriptions-item>
                <a-descriptions-item :label="$t('score.total_credits_attempted')">
                  {{ studentGrades!.歷年總修習學分數 }}
                </a-descriptions-item>
                <a-descriptions-item :label="$t('score.average_score')">
                  {{ studentGrades!.歷年平均成績 }}
                </a-descriptions-item>
                <a-descriptions-item :label="$t('score.latest_semester')">
                  {{ studentGrades!.截至學期 }}
                </a-descriptions-item>
              </a-descriptions>
            </a-card>
            <div
              class="semester mb-5"
              v-for="(semester, index) in courses"
              :key="index"
            >
              <a-divider v-if="index != 0" />
              <h2 v-if="tab == -1" mb-3>
                {{ Steps[index].title }} {{ Steps[index].description }}
              </h2>
              <ScoreStats
                :rank="semester.排名"
                :average="semester.平均成績"
                :Progress="tab == -1 ? undefined : Progress(semester)"
              />
              <h2 v-if="tab != -1" mb-3>
                {{ Steps[tab].title }} {{ Steps[tab].description }}
              </h2>
              <div flex gap="5" class="mt-5" xl:flex-nowrap flex-wrap>
                <div class="w-full xl:w-50% flex flex-col gap-2">
                  <div class="flex justify-between px-2">
                    {{ $t("score.required_courses") }}
                    <div class="flex items-center">
                      <a-progress
                        type="circle"
                        trail-color="#e6f4ff"
                        :percent="
                          (Progress(semester).必修.finished /
                            Progress(semester).必修.total) *
                          100
                        "
                        :stroke-width="20"
                        :size="12"
                      /><span class="ps-2">
                        {{ Progress(semester).必修.finished }}/
                        {{ Progress(semester).必修.total }}
                      </span>
                    </div>
                  </div>
                  <ScoreCard
                    v-for="course in semester.必修課程"
                    :key="course.課號"
                    :course="course"
                  />
                </div>
                <div class="w-full xl:w-50% flex flex-col gap-2">
                  <div class="flex justify-between px-2">
                    {{ $t("score.elective_courses") }}
                    <div class="flex items-center">
                      <a-progress
                        type="circle"
                        trail-color="#e6f4ff"
                        :percent="
                          (Progress(semester).選修.finished /
                            Progress(semester).選修.total) *
                          100
                        "
                        :stroke-width="20"
                        :size="12"
                      /><span class="ps-2">
                        {{ Progress(semester).選修.finished }}/
                        {{ Progress(semester).選修.total }}
                      </span>
                    </div>
                  </div>
                  <ScoreCard
                    v-for="course in semester.選修課程"
                    :key="course.課號"
                    :course="course"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </a-flex>
    </template>
    <template v-else>
      <div class="flex flex-col items-center justify-center h-full w-full">
        <h1 class="title">{{ $t("loading") }}</h1>
        <a-spin />
      </div>
    </template>
</div>
</template>

<style lang="scss">
.ant-steps-item {
  &.ant-steps-item-active {
    .ant-steps-item-container > .ant-steps-item-content {
      background-color: var(--highlight-color);
      border-radius: 8px;
      padding-left: 10px;
    }
  }

  .ant-steps-item-container > .ant-steps-item-content {
    transition: all 0.3s;
  }
}
</style>

<style scoped lang="scss">
.title {
  font-size: 2rem;
  margin-bottom: 20px;
}
h2 {
  font-size: 1.5rem;
  margin-top: 30px;
}
.score-view {
  width: 100%;
}

.semester {
  h3 {
    font-size: 1.2rem;
    margin-bottom: 10px;
  }
}
.all-semesters {
  background-color: var(--highlight-color-0);
  box-shadow: 0 0 3px 0 rgba(128, 128, 128, 0.1);
  opacity: 0.8;
  transition: all 0.3s;
  &:hover {
    opacity: 1;
    box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.3);
  }
  &:active {
    background-color: var(--highlight-color);
    opacity: 1;
  }
  &.active {
    background-color: var(--highlight-color);
    opacity: 1;
  }
}
</style>
