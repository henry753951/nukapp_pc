<script setup lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/tauri";
  import { logger } from "../logger";
  import { defineProps, ref, watch, onMounted } from "vue";

  type Course = {
    課號: string;
    課程名稱: string;
    學分數: string;
    修別: string;
    期中成績: string;
    學期成績: string;
    備註: string;
  };

  type SchoolYear = {
    學期: string;
    修習學分數: string;
    實得學分數: string;
    平均成績: string;
    排名: string;
    課程: Course[];
  };

  type StudentGrades = {
    歷年總修習學分數: string;
    歷年實得學分數: string;
    歷年平均成績: string;
    歷年全班排名: string;
    節至學期: string;
    各學期: SchoolYear[];
  };

  const studentGrades = ref<StudentGrades>();

  const getScore = () => {
    invoke("get_user_score").then((result) => {
      studentGrades.value = result as StudentGrades;
      console.log("studentGrades", studentGrades.value);
    });
  };

  onMounted(() => {
    getScore();
  });

  watch(
    studentGrades,
    (newValue) => {
      console.log("Updated grades", newValue);
    },
    { deep: true }
  );
</script>

<template>
  <div v-if="studentGrades">
    <div v-for="(semester, index) in studentGrades.各學期" :key="index">
      <h2>{{ semester.學期 }}</h2>
      <div v-for="course in semester.課程" :key="course.課號">
        <p>課程名稱: {{ course.課程名稱 }}</p>
        <p>學分數: {{ course.學分數 }}</p>
        <p>學期成績: {{ course.學期成績 }}</p>
      </div>
    </div>
  </div>
</template>
