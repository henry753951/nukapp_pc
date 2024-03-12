<template>
  <a-flex vertical gap="18">
    <a-statistic title="課程名稱" :value="course.course_name" />
    <a-statistic
      title="課程編號"
      :value="course.department + '-' + course.course_id" />
    <a-statistic
      title="學分"
      :value="`${course.credits} 學分 / ${course.requirement_type}修`" />
    <a-statistic
      title="上課時間"
      :value="CourseTimeToString(course.course_time)" />
    <a-statistic title="上課地點" :value="course.classroom" />
    <a-flex gap="10">
      <a-card>
        <!-- 線上人數 -->
        <a-statistic title="線上人數" :value="course.online_number" />
      </a-card>
      <a-card>
        <!-- 名額 -->
        <a-statistic title="名額" :value="course.registration_confirmed">
          <template #suffix>
            <span>/ {{ course.limit }}</span>
          </template>
        </a-statistic>
      </a-card>
    </a-flex>

    <a-collapse :bordered="false" style="background: transparent">
      <a-collapse-panel key="2" header="備註" :style="collapseStyle">
        <p>{{ course.notes }}</p>
      </a-collapse-panel>
      <a-collapse-panel key="1" header="限修條件" :style="collapseStyle">
        <p>{{ course.prerequisites }}</p>
      </a-collapse-panel>
      <a-collapse-panel key="3" header="原始資料" :style="collapseStyle">
        <p>{{ course }}</p>
      </a-collapse-panel>
    </a-collapse>
  </a-flex>
</template>
<script setup lang="ts">
  import { ref, watch, computed, onMounted, onUnmounted, reactive } from "vue";
  import { BaseCourse, Course } from "../interface";

  const collapseStyle =
    "background: var(--highlight-color-0);border-radius: 4px;margin-bottom: 24px;border: 0;overflow: hidden";

  const CourseTimeToString = (course_time: BaseCourse["course_time"]) => {
    let str = "";
    for (let time of course_time) {
      str += `${time[0]} ${time[1].join(",")} \n`;
    }
    return str;
  };
  const props = defineProps<{
    course: Course | BaseCourse;
  }>();
</script>
<style lang="scss" scoped></style>
