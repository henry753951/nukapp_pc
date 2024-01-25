<template>
  <div class="p-4 z-20 w-full" style="max-width: 500px">
    <a-segmented v-model:value="tableType" block :options="data" />
  </div>
  <div ref="outerCourseTable" class="w-full h-full flex flex-col items-center">
    <div
      v-if="tableType === '全螢幕課表'"
      class="container"
      :style="containerStyle">
      <a-flex gap="5" class="w-full">
        <div
          v-for="(day, d_index) in Days"
          style="width: 100%"
          class="flex justify-center">
          <p>{{ day }}</p>
        </div>
      </a-flex>
      <a-flex gap="8" class="pb-10">
        <div v-for="(day, d_index) in Days" class="w-full">
          <a-flex vertical gap="5" class="w-full">
            <div
              class="course-card cursor-pointer"
              @click="emit('onCourseClick', CourseTable[d_index][t_index])"
              :bordered="false"
              :class="{
                'my-4': time === 'Y' || time === 'X',
                flex: true,
                'justify-center': true,
                'items-center': true,
              }"
              v-for="(time, t_index) in Times">
              <p class="text-xs text-center">
                {{ CourseTable[d_index][t_index].course_name }}
              </p>
            </div>
          </a-flex>
        </div>
      </a-flex>
    </div>
    <div v-if="tableType === '簡易式課表'">
      <!-- Coming soon -->
      <p>Coming soon</p>
    </div>
  </div>
  <a-float-button-group shape="circle" :style="{ right: '24px' }">
    <a-popconfirm
      title="你確定要清空課表嗎？"
      okType="danger"
      placement="left"
      ok-text="清空"
      cancel-text="取消"
      @confirm="emit('onCourseClear')">
      <a-float-button>
        <template #icon>
          <VueFeather type="trash" size="20" />
        </template>
      </a-float-button>
    </a-popconfirm>
  </a-float-button-group>
</template>
<script setup lang="ts">
  import { ref, watch, computed, onMounted, onUnmounted, reactive } from "vue";
  import { BaseCourse, Course } from "../interface";
  import { logger } from "../logger";
  import { useCustomCourseDataStore } from "../stores/CustomCourseData";
  import VueFeather from "vue-feather";
  const CustomDataStore = useCustomCourseDataStore();

  const outerCourseTable = ref<HTMLElement>();
  const maxWidth = ref(650);
  const scale = ref(1);
  const resizeObserver = ref<ResizeObserver>();

  const data = reactive(["全螢幕課表", "簡易式課表"]);
  const tableType = ref(data[0]);

  const containerStyle = computed(() => {
    return {
      transform: `scale(${scale.value})`,
    };
  });

  const Days = ["一", "二", "三", "四", "五", "六", "日"];
  const Times = [
    "X",
    "1",
    "2",
    "3",
    "4",
    "Y",
    "5",
    "6",
    "7",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13",
  ];

  const CourseTable = computed(() => {
    var table: Course[][] = [];

    for (let i = 0; i < 7; i++) {
      var DayCourses: Course[] = [];
      const CoursesFromDay = props.courseList.filter((course) => {
        for (const day of course.course_time) {
          if (day[0] === Days[i]) return true;
        }
      });
      for (let j = 0; j < Times.length; j++) {
        const courses = CoursesFromDay.filter((course) => {
          for (const day of course.course_time) {
            if (day[0] === Days[i]) {
              if (day[1].includes(Times[j])) {
                console.log(Times[j], day[1], course.course_name);
                return true;
              }
            }
          }
        });

        if (courses.length === 0) {
          DayCourses.push({} as Course);
        } else {
          var course: Course = {
            ...courses[0],
            custom_Data: CustomDataStore.getCustomCourseData(courses[0].key),
          };
          DayCourses.push(course);
        }
      }
      table.push(DayCourses);
    }
    // logger.debug(table);
    return table;
  });

  onMounted(() => {
    if (outerCourseTable.value) {
      resizeObserver.value = new ResizeObserver((entries) => {
        const { width } = entries[0].contentRect;
        if (width > maxWidth.value) scale.value = 1;
        else scale.value = Math.floor(width) / maxWidth.value;
      });
      resizeObserver.value.observe(outerCourseTable.value);
    }
  });

  onUnmounted(() => {
    if (resizeObserver.value) {
      resizeObserver.value.disconnect();
    }
  });

  const props = defineProps<{
    courseList: BaseCourse[];
  }>();

  const emit = defineEmits<{
    (event: "onCourseClick", course: Course): void;
    (event: "onCourseClear"): void;

  }>();
</script>
<style lang="scss" scoped>
  .container {
    transform-origin: top left;
    width: 650px !important;
    min-width: 650px !important;
    max-width: 650px !important;
    padding: 10px;
  }
  .course-card {
    height: 80px;
    width: 100%;
    background-color: var(--heighlight-color);
    border-radius: 5px;
    box-shadow: 0 0 10px var(--shadow-color);
    padding: 10px;
  }
</style>
