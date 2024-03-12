<template>
  <a-card hoverable>
    <a-flex justify="space-between" style="z-index: 1">
      <a-flex gap="5" vertical>
        <div class="course_title">
          {{ course.課程名稱 }}
        </div>
        <div>{{ course.學分數 }}學分</div>
      </a-flex>

      <a-flex gap="15" align="center">
        <div>
          期中
          <div class="score">
            {{ course.期中成績
            }}<span v-if="course.期中成績 !== '未送'">分</span>
          </div>
        </div>
        <div>
          學期
          <div class="score">
            {{ course.學期成績
            }}<span v-if="course.期中成績 !== '未送'">分</span>
          </div>
        </div>
      </a-flex>
    </a-flex>
    <span class="code">{{ course.課號 }}</span>
  </a-card>
</template>

<script setup lang="ts">
  import { defineProps } from "vue";
  import { Semester, Course, SchoolYear, StudentGrades } from "../types";

  const props = defineProps({
    course: {
      type: Object as () => Course,
      required: true,
    },
  });
</script>

<style lang="scss" scoped>
  .course_title {
    font-size: 1.1rem;
    font-weight: bold;
  }
  .score {
    font-size: 1.2rem;
    width: 75px;
  }
  * {
    font-family: "JetBrains Mono", "Noto Sans TC", sans-serif;
    font-size: 0.9rem;
  }
  span {
    font-size: 0.8rem;
  }
  .code{
    position: absolute;
    right: 0;
    // center the code
    top: 50%;
    transform: translateY(-50%) rotate(-90deg);
    font-size: 0.8rem;
    
  }
</style>
