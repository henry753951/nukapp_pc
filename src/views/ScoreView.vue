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
  <h1 class="title">目前為測試介面，尚未完成</h1>
  <div v-if="studentGrades" class="score-view">
    <div
      class="semester-card"
      v-for="(semester, index) in studentGrades.各學期"
      :key="index">
      <h2>{{ semester.學期 }}</h2>
      <div
        class="course-card"
        v-for="course in semester.課程"
        :key="course.課號">
        <p><strong>課程名稱:</strong> {{ course.課程名稱 }}</p>
        <p><strong>學分數:</strong> {{ course.學分數 }}</p>
        <p><strong>學期成績:</strong> {{ course.學期成績 }}</p>
        <p><strong>備註:</strong> {{ course.備註 }}</p>
      </div>

      <div class="semester-summary">
        <h3>修習學分數: {{ semester.修習學分數 }}</h3>
        <h3>實得學分數: {{ semester.實得學分數 }}</h3>
        <h3>平均成績: {{ semester.平均成績 }}</h3>
        <h3>排名: {{ semester.排名 }}</h3>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
  .title {
    padding: 20px;
    font-size: 2rem;
    margin-bottom: 20px;
  }

  .score-view {
    padding: 20px;
    display: flex;
    gap: 20px;
  }

  .semester-card {
    background-color: #f9f9f9;
    border-radius: 10px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    padding: 20px;
    width: calc(50% - 10px);
    box-sizing: border-box;

    h2 {
      font-size: 1.5rem;
      margin-bottom: 20px;
      color: #333;
    }

    .course-card {
      background-color: #fff;
      border: 1px solid #eee;
      border-radius: 8px;
      padding: 15px;
      margin-bottom: 10px;

      p {
        font-size: 1rem;
        margin-bottom: 5px;
        strong {
          font-weight: bold;
        }
      }
    }

    .semester-summary {
      background-color: #eef;
      border-radius: 8px;
      padding: 10px;
      margin-top: 10px;

      h3 {
        font-size: 1.2rem;
        margin-bottom: 10px;
      }
    }
  }
</style>
