<template>
  <div class="flex flex-col h-full">
    <VueSplitter v-model:percent="limitedPercent">
      <template #left-pane>
        <div
          class="flex flex-col h-full w-full"
          style="background-color: rgba(255, 255, 255, 0.1)">
          <a-table
            sticky
            @resizeColumn="handleResizeColumn"
            :row-selection="rowSelection"
            :columns="columns"
            :data-source="filteredData"
            :loading="loading"
            :scroll="{ x: 'max-content' }"
            :rowClassName="
              (_record: BaseCourse) => {
                return checkCourseTimeConflictByCourse(_record)
                  ? 'ConflictCourse'
                  : '';
              }
            "
            :locale="{
              emptyText: '沒有課程',
              filterConfirm: '確定',
              filterReset: '重置',
              filterTitle: '篩選',
              selectAll: '全選',
              selectInvert: '反選',
              sortTitle: '排序',
            }"
            :pagination="{
              position: ['bottomRight'],
              defaultPageSize: 20,
              onChange: (_page: any, _pageSize: any) => {
                scrollToTop();
              },
            }">
            <template #title>
              <a-flex vertical class="w-full">
                <a-alert
                  v-show="!loading"
                  :message="'資料最近更新時間: ' + lastUpdateTime"
                  type="info"
                  show-icon
                  closable>
                </a-alert>

                <a-flex gap="5" class="my-2">
                  <a-input-search
                    placeholder="課程名稱或代碼"
                    style="width: 200px; margin-right: auto"
                    v-model:value="filter.tempSearch"
                    @search="filter.search = filter.tempSearch" />
                  <a-select
                    style="width: 200px"
                    placeholder="課程分類"
                    @select="department_select"
                    :options="filter.category.options"
                    v-model:value="filter.category.value" />
                  <a-select
                    style="width: 100px"
                    placeholder="年級"
                    :options="filter.Year.options"
                    v-model:value="filter.Year.value"
                    :disabled="filter.Year.disabled" />
                  <a-popover title="資料狀態">
                    <template #content>
                      <span>最近更新時間 : {{ lastUpdateTime }}</span>
                    </template>
                    <a-button @click="getData(true)">更新資料</a-button>
                  </a-popover>
                </a-flex>
              </a-flex>
            </template>

            <template #bodyCell="{ column, value, record }">
              <!-- course_name -->
              <template v-if="column.dataIndex === 'course_name'">
                <a-flex vertical>
                  <a-space>
                    <a
                      :href="
                        'https://course.nuk.edu.tw/QueryCourse/' +
                        record.syllabus_link
                      "
                      target="_blank"
                      rel="noopener noreferrer">
                      {{ value }}
                    </a>
                    <a-tooltip
                      placement="rightTop"
                      v-if="record.online_number >= record.limit">
                      <template #title>
                        <span
                          >線上:{{ record.online_number }} / 名額:{{
                            record.limit
                          }}
                        </span>
                      </template>
                      <a-tag color="red"> 超額 </a-tag>
                    </a-tooltip>
                  </a-space>
                  <div>
                    {{ record.teacher.join(", ") }}
                  </div>
                </a-flex>
              </template>
              <!-- course_time -->
              <template v-else-if="column.dataIndex === 'course_time'">
                <a-space>
                  <span v-for="time in value">
                    <span class="pr-1">{{ time[0] }} </span>
                    <span>{{ time[1].join(",") }}</span>
                    <br />
                  </span>
                </a-space>
              </template>
              <!-- credits / requirement_type-->
              <template v-else-if="column.dataIndex === 'credits'">
                <a-flex class="w-full" gap="5">
                  <a-space>
                    <span>{{ record.credits }} 學分 </span>
                    <a-tag
                      :bordered="false"
                      :color="
                        record.requirement_type === '必' ? 'error' : 'cyan'
                      ">
                      {{ record.requirement_type }}</a-tag
                    >
                  </a-space>
                  <a-button shape="circle" @click="showCourseModal(record)">
                    <a-flex align="center" justify="center">
                      <vue-feather type="info" size="18"></vue-feather>
                    </a-flex>
                  </a-button>
                </a-flex>
              </template>
              <!-- other -->
              <template v-else>
                <span>{{ value }}</span>
              </template>
            </template>
          </a-table>
        </div>
      </template>
      <template #right-pane>
        <div
          class="h-full overflow-y-auto flex flex-col items-center"
          style="scrollbar-gutter: stable both-edges">
          <CourseTable
            :course-list="selectedCourses"
            @on-course-click="showCourseModal"
            @on-course-clear="clearSelectdCourse" />
        </div>
      </template>
    </VueSplitter>
    <a-drawer
      :title="CheckCourseModalData.course.course_name"
      placement="right"
      :closable="true"
      :open="CheckCourseModalData.open"
      :get-container="true"
      @close="onCloseCourseModal">
      <CourseDetail :course="CheckCourseModalData.course" />
    </a-drawer>
  </div>
</template>
<script lang="ts" setup>
  import { message } from "ant-design-vue";
  import { NotificationPlacement, notification } from "ant-design-vue";

  import { BaseCourse, Course } from "../interface";
  import { invoke } from "@tauri-apps/api/core";
  import type { TableProps, TableColumnType } from "ant-design-vue";
  import { Key } from "ant-design-vue/es/_util/type";
  import {
    ref,
    onMounted,
    inject,
    watch,
    unref,
    reactive,
    computed,
  } from "vue";
  import { useSelectedCourseStore } from "../stores/selectedCourse";
  import { logger } from "../logger";
  import VueFeather from "vue-feather";
  import SchoolData from "../data";

  // Components
  import CourseTable from "../components/CourseTable.vue";
  import VueSplitter from "@rmp135/vue-splitter";
  import CourseDetail from "../components/CourseDetail.vue";
  // data
  const SelectedCourseStore = useSelectedCourseStore();

  const lastUpdateTime = ref("");
  const data = ref<BaseCourse[]>([]);
  const loading = ref(true);
  const selectedCourseKeys = ref(SelectedCourseStore.selectedCourseKeys);
  const selectedCourses = ref([] as BaseCourse[]);

  const filter = reactive({
    tempSearch: "",
    search: "",
    category: {
      options: [{ label: "不限", value: "不限" }, ...SchoolData.department],
      value: "不限",
    },
    Year: {
      disabled: false,
      options: ["不限", "一年級", "二年級", "三年級", "四年級"].map(
        (option, index) => ({ value: index, label: option })
      ),
      value: 0,
    },
  });

  const CheckCourseModalData = reactive({
    course: {} as BaseCourse,
    open: false,
  });

  const columns: TableColumnType<BaseCourse>[] = reactive([
    {
      title: "課程名稱",
      dataIndex: "course_name",
      resizable: true,
      minWidth: 200,
      maxWidth: 500,
    },
    {
      title: "上課時間",
      dataIndex: "course_time",
      resizable: true,
    },
    {
      title: "",
      dataIndex: "credits",
    },
  ]);

  // methods
  const scrollToTop = inject<() => void>("scrollToTop")!;
  const checkCourseTimeConflict = (course_time: BaseCourse["course_time"]) => {
    let conflict = false;

    for (let selectedCourse of selectedCourses.value) {
      for (let selectedCourseTime of selectedCourse.course_time) {
        for (let newCourseTime of course_time) {
          // Check if the days are the same
          if (selectedCourseTime[0] === newCourseTime[0]) {
            // Check if the hours overlap
            let selectedHours = new Set(selectedCourseTime[1]);
            for (let hour of newCourseTime[1]) {
              if (selectedHours.has(hour)) {
                conflict = true;
                break;
              }
            }
          }
          if (conflict) break;
        }
        if (conflict) break;
      }
    }

    return conflict;
  };
  const department_select = (value: string) => {
    if (
      SchoolData.department[0].options.some(
        (option) => option.value === value
      ) ||
      SchoolData.department[1].options.some((option) => option.value === value)
    ) {
      filter.Year.value = 0;
      filter.Year.disabled = true;
    } else {
      filter.Year.disabled = false;
    }
  };
  function handleResizeColumn(w: any, col: { width: any }) {
    col.width = w;
  }
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
    selectedCourseKeys.value = [];
    selectedCourses.value = [];
  };
  // computed
  const filteredData = computed(() => {
    let _data = data.value.filter((course) => {
      return (
        course.course_name.includes(filter.search) ||
        `${course.department}${course.course_id}`.includes(filter.search)
      );
    });

    if (filter.category.value !== "不限") {
      _data = _data.filter((course) => {
        return course.department === filter.category.value.toUpperCase();
      });
    }

    if (filter.Year.value !== 0) {
      _data = _data.filter((course) => {
        return course.grade === filter.Year.value;
      });
    }

    return _data;
  });

  const rowSelection = computed(() => {
    return {
      hideSelectAll: true,
      selectedRowKeys: unref(selectedCourseKeys),
      preserveSelectedRowKeys: true,
      onChange: (selectedRowKeys: Key[], _selectedRows: BaseCourse[]) => {
        logger.info("SelectedRowKeys changed: ", selectedRowKeys);
        selectedCourseKeys.value = selectedRowKeys as BaseCourse["key"][];
        selectedCourses.value = data.value.filter((course) =>
          selectedCourseKeys.value.includes(course.key)
        );
      },
      getCheckboxProps: (record: BaseCourse) => ({
        disabled: selectedCourseKeys.value.includes(record.key)
          ? false
          : checkCourseTimeConflict(record.course_time),
        name: record.course_name,
      }),
    };
  });

  const checkCourseTimeConflictByCourse = (course: BaseCourse) => {
    return selectedCourseKeys.value.includes(course.key)
      ? false
      : checkCourseTimeConflict(course.course_time);
  };

  const getData = async (refresh = false) => {
    loading.value = true;

    let jsondata = (await invoke("get_all_course", {
      refresh: refresh,
    })) as { course_ls: BaseCourse[]; updateTime: string };
    console.log(jsondata);
    let all_course = jsondata.course_ls;
    data.value = all_course;
    lastUpdateTime.value = jsondata.updateTime;
    loading.value = false;
    selectedCourses.value = all_course.filter((course) =>
      selectedCourseKeys.value.includes(course.key)
    );

    if (refresh) {
      logger.info("Refreshed");
      notification.info({
        message: `已更新資料`,
        description: "資料皆為學校官方資料，僅供參考，若有錯誤請至課務系統查詢",
        placement: "bottomLeft" as NotificationPlacement,
      });
    }
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
  // lifecycle
  onMounted(() => {
    getData();
  });

  watch(
    () => selectedCourseKeys.value,
    (newVal) => {
      SelectedCourseStore.selectedCourseKeys = newVal as string[];
    }
  );
  watch(
    () => filter.tempSearch,
    (newVal, oldVal) => {
      if (newVal === "" && oldVal !== "") {
        filter.search = "";
      }
    }
  );
</script>

<style lang="scss">
  .ant-table-row-selected:hover {
    .ant-table-cell {
      background-color: #00000020 !important;
    }
  }

  // all ant-table elements
  .ant-table-cell {
    span {
      font-family: "JetBrains Mono";
    }
    font-family: "JetBrains Mono";
  }
  .ConflictCourse {
    &:hover {
      .ant-table-cell {
        transition: none;
        background-color: #ff7e800c !important;
      }
    }
    .ant-table-cell {
      transition: none;
      background-color: #ff7e800c !important;
    }
  }
  .ant-table-sticky-scroll-bar {
    display: none;
  }
  .ant-table-sticky-scroll {
    display: none !important;
  }
  .ant-table-pagination {
    padding: 15px;
  }
</style>
