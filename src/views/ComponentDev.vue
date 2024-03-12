<script lang="ts" setup>
  import ScoreCard from "../components/ScoreCard.vue";
  import ScoreStats from "../components/ScoreStats.vue";
  import { Course } from "../types";

  import { ref } from "vue";
  const course = ref({
    課號: "CS101",
    課程名稱: "程式設計",
    學分數: "3",
    修別: "必修",
    期中成績: "80",
    學期成績: "90",
    備註: "",
  } as Course);
</script>
<template>
  <div class="container p-5 flex gap-5 flex-col">
    <ScoreCard :course="course" />
    <ScoreStats rank="10" average="85" />
  </div>
</template>
