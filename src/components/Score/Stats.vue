<template>
  <a-card hoverable>
    <div v-if="Progress">
      <a-result
        status="success"
        :title="$t('score.status.success_message')"
        v-if="
          Progress.選修.total === Progress.選修.finished &&
          Progress.必修.total === Progress.必修.finished
        " />
    </div>

    <a-flex justify="center" gap="50">
      <a-statistic
        :title="$t('score.status.rank')"
        :value="_rank"
        :precision="0"
        :suffix="$t('score.status.suffix_rank')"
        :value-style="{ color: '#3f8600' }"
        style="">
      </a-statistic>
      <a-statistic
        :title="$t('score.status.average_score')"
        :value="average"
        :precision="2"
        :suffix="$t('score.status.suffix_score')"
        :value-style="{ color: '#3f8600' }"
        style="">
      </a-statistic>
    </a-flex>
  </a-card>
</template>

<script setup lang="ts">
import type { CourseProgress } from "@/types/types";

const props = defineProps({
  rank: {
    type: String,
    required: true,
  },
  average: {
    type: String,
    required: true,
  },
  Progress: {
    type: Object as () => CourseProgress,
    required: false,
  },
});

const _rank = computed(() => {
  if (props.rank.startsWith("--")) {
    return "--";
  }
  return parseInt(props.rank);
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
</style>
