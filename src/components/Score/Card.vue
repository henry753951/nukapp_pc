<template>
  <a-card hoverable>
    <div flex justify="space-between" style="z-index: 1" xl:flex-row flex-col gap-5>
      <div
        flex
        gap="5"
        items-center
        xl:items-start
        xl:flex-col
        flex-row
        style="flex: 1; min-width: 0"
      >
        <div class="course_title">
          {{ course.課程名稱 }}
        </div>
        <div mr-5>{{ course.學分數 }} {{ $t('score.card.credit_unit') }}</div>
      </div>

      <div flex gap="-4px" align="center" style="flex-shrink: 0">
        <div xl:block flex items-center>
          {{ $t('score.card.midterm') }}
          <div class="score">
            <span>{{ course.期中成績 == '未送' ? $t('score.not_submitted_tiny') : course.期中成績 }}</span>
            <span v-if="course.期中成績 !== '未送'">
              {{ $t('score.card.unit') }}
            </span>
          </div>
        </div>
        <div xl:block flex items-center>
          {{ $t('score.card.semester') }}
          <div class="score">
            <span>{{ course.學期成績 == '未送' ? $t('score.not_submitted_tiny') : course.學期成績 }}</span>
            <span v-if="course.學期成績 !== '未送'">
              {{ $t('score.card.unit') }}
            </span>
          </div>
        </div>
      </div>
    </div>
    <span class="code">{{ course.課號 }}</span>
  </a-card>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
import type { Course } from "@/types/types";

const props = defineProps({
  course: {
    type: Object as () => Course,
    required: true,
  },
});
</script>

<style lang="scss" scoped>
.course_title {
  width: 100%;
  font-size: 1.1rem;
  font-weight: bold;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden; /* 隱藏超出部分 */
  flex: 1; /* 允許自適應剩餘空間 */
}

.score {
  width: 65px;
  span {
    font-size: 1.2rem;
    & + span {
      font-size: 0.8rem;
    }
  }
}

* {
  font-family: "JetBrains Mono", "Noto Sans TC", sans-serif;
  font-size: 0.9rem;
}

.code {
  position: absolute;
  right: -5px;
  top: 50%;
  transform: translateY(-50%) rotate(-90deg);
  font-size: 0.7rem;
}

a-card {
  position: relative;
  display: flex;
}

a-flex {
  display: flex;
  flex-direction: row;
  width: 100%;
}

a-flex[vertical] {
  flex-direction: column;
}
</style>
