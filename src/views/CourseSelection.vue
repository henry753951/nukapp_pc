<template>
  <div>
    <a-table
      sticky
      style="height: 100%"
      :row-selection="rowSelection"
      :columns="columns"
      :data-source="data"
      :loading="loading"
      :rowClassName="(_record: DataType) => {
        return checkCourseTimeConflictByCourse(_record) ? 'ConflictCourse' : ''
      }"
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
      onChange: (_page:any, _pageSize:any) => {scrollToTop()}
      }">
      <template #bodyCell="{ column, value, record }">
        <template v-if="column.dataIndex === 'teacher'">
          <span>{{ value.join(", ") }}</span>
        </template>
        <template v-else-if="column.dataIndex === 'course_name'">
          <a-space>
            <a
              :href="record.syllabus_link"
              target="_blank"
              rel="noopener noreferrer">
              {{ value }}
            </a>
            <a-tooltip
              placement="rightTop"
              v-if="record.online_number >= record.limit">
              <template #title>
                <span>線上:{{ record.online_number }} / 名額:{{ record.limit }} </span>
              </template>
              <a-tag color="red"> 超額 </a-tag>
            </a-tooltip>
          </a-space>
        </template>
        <template v-else-if="column.dataIndex === 'credits'">
          <a-space>
            <span>{{ record.credits }} 學分 </span>
            <a-tag
              :bordered="false"
              :color="record.requirement_type === '必' ? 'error' : 'cyan'">
              {{ record.requirement_type }}</a-tag
            >
          </a-space>
        </template>
        <template v-else>
          <span>{{ value }}</span>
        </template>
      </template>
    </a-table>
  </div>
</template>
<script lang="ts" setup>
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
  const SelectedCourseStore = useSelectedCourseStore();
  // data
  const data = ref<DataType[]>([]);
  const loading = ref(true);
  const selectedCourseKeys = ref(SelectedCourseStore.selectedCourseKeys);
  const selectedCourses = ref([] as DataType[]);
  // table
  interface DataType {
    key: string;
    department: string;
    course_id: string;
    department_code: string;
    grade: number;
    class_type: string;
    course_name: string;
    syllabus_link: string;
    credits: number;
    requirement_type: string;
    limit: number;
    registration_confirmed: number;
    online_number: number;
    balance: number;
    teacher: Array<string>;
    classroom: string;
    course_time: Array<[string, string[]]>;
    prerequisites: string;
    notes: string;
  }

  const columns: TableColumnType<DataType>[] = [
    {
      title: "課程名稱",
      dataIndex: "course_name",
    },
    {
      title: "老師",
      dataIndex: "teacher",
    },
    {
      title: "上課時間",
      dataIndex: "course_time",
    },
    {
      title: "資料時間 : 2021/09/01 00:00:00",
      dataIndex: "credits",
    },
  ];

  // methods
  const scrollToTop = inject<() => void>("scrollToTop")!;
  const checkCourseTimeConflict = (course_time: DataType["course_time"]) => {
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

  // computed
  const rowSelection = computed(() => {
    return {
      hideSelectAll: true,
      selectedRowKeys: unref(selectedCourseKeys),
      onChange: (selectedRowKeys: Key[], selectedRows: DataType[]) => {
        console.log("selectedRowKeys changed: ", selectedRowKeys);
        selectedCourseKeys.value = selectedRowKeys as DataType["key"][];
        selectedCourses.value = selectedRows;
      },
      getCheckboxProps: (record: DataType) => ({
        disabled: selectedCourseKeys.value.includes(record.key)
          ? false
          : checkCourseTimeConflict(record.course_time),
        name: record.course_name,
      }),
    };
  });

  const checkCourseTimeConflictByCourse = (course: DataType) => {
    return selectedCourseKeys.value.includes(course.key)
      ? false
      : checkCourseTimeConflict(course.course_time);
  };

  // lifecycle
  onMounted(async () => {
    let all_course = (await invoke("get_all_course")) as DataType[];
    console.log(all_course);
    data.value = all_course;
    loading.value = false;
    selectedCourses.value = all_course.filter((course) =>
      selectedCourseKeys.value.includes(course.key)
    );
  });

  watch(
    () => selectedCourseKeys.value,
    (newVal) => {
      SelectedCourseStore.selectedCourseKeys = newVal as string[];
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
</style>
