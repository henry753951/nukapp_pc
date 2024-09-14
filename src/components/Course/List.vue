<template>
  <div
    class="flex flex-col w-full"
    style="background-color: rgba(255, 255, 255, 0.1)"
  >
    <a-table
      sticky
      @resizeColumn="handleResizeColumn"
      :row-selection="rowSelection"
      :columns="columns"
      :data-source="filteredData"
      :loading="loading"
      :scroll="{ x: 'max-content' }"
      :rowClassName="getRowClassName"
      :locale="localeSettings"
      :pagination="paginationSettings"
    >
      <template #title>
        <a-flex vertical class="w-full">
          <a-alert
            v-show="!loading"
            :message="'資料最近更新時間: ' + lastUpdateTime"
            type="info"
            show-icon
            closable
          />
          <a-flex gap="5" class="my-2">
            <a-input-search
              placeholder="課程名稱或代碼"
              style="width: 200px; margin-right: auto"
              v-model:value="filter.tempSearch"
              @search="updateSearch"
            />
            <a-select
              style="width: 200px"
              placeholder="課程分類"
              v-model:value="filter.category.value"
              :options="filter.category.options"
              @select="departmentSelect"
            />
            <a-select
              style="width: 100px"
              placeholder="年級"
              v-model:value="filter.Year.value"
              :options="filter.Year.options"
              :disabled="filter.Year.disabled"
            />
            <a-popover title="資料狀態">
              <template #content>
                <span>最近更新時間 : {{ lastUpdateTime }}</span>
              </template>
              <a-button @click="emits('refresh')">更新資料</a-button>
            </a-popover>
          </a-flex>
        </a-flex>
      </template>

      <template #bodyCell="{ column, value, record }">
        <template v-if="column.dataIndex === 'course_name'">
          <a-flex vertical>
            <a-space>
              <a
                :href="`https://course.nuk.edu.tw/QueryCourse/${record.syllabus_link}`"
                target="_tauri"
                rel="noopener noreferrer"
              >
                {{ value }}
              </a>
              <a-tooltip
                placement="rightTop"
                v-if="record.online_number >= record.limit"
              >
                <template #title>
                  線上:{{ record.online_number }} / 名額:{{ record.limit }}
                </template>
                <a-tag color="red">超額</a-tag>
              </a-tooltip>
            </a-space>
            <div>{{ record.teacher.join(", ") }}</div>
          </a-flex>
        </template>

        <template v-else-if="column.dataIndex === 'course_time'">
          <a-space>
            <span v-for="time in value" :key="time[0]">
              <span class="pr-1">{{ time[0] }}</span>
              <span>{{ time[1].join(",") }}</span>
              <br />
            </span>
          </a-space>
        </template>

        <template v-else-if="column.dataIndex === 'credits'">
          <a-flex class="w-full" gap="5">
            <a-space>
              <span>{{ record.credits }} 學分</span>
              <a-tag :bordered="false" :color="getRequirementTagColor(record)">
                {{ record.requirement_type }}
              </a-tag>
            </a-space>
            <a-button shape="circle" @click="showCourseModal(record)">
              <a-flex align="center" justify="center">
                <vue-feather type="info" size="18"></vue-feather>
              </a-flex>
            </a-button>
          </a-flex>
        </template>

        <template v-else>
          <span>{{ value }}</span>
        </template>
      </template>
    </a-table>
  </div>
</template>

<script lang="ts" setup>
import VueFeather from "vue-feather";
import SchoolData from "~/static.data";
import type { BaseCourse } from "~/types/interface";
import type { NotificationPlacement, TableColumnType } from "ant-design-vue";

// Logger setup
const logger = useLogger("CourseList");

// Store and reactive variables
const SelectedCourseStore = useSelectedCourseStore();
const selectedCourses = computed(() => {
  if (!data.value) return [];
  return data.value?.filter((course) =>
    SelectedCourseStore.selectedCourseKeys.includes(course.key)
  );
});
const data = defineModel("courses_data", {
  type: Array as () => BaseCourse[] | null,
  default: () => [] as BaseCourse[],
});
const props = defineProps<{
  lastUpdateTime?: string;
  loading: boolean;
}>();

// Filter and search settings
const filter = reactive({
  tempSearch: "",
  search: "",
  category: {
    options: [{ label: "不限", value: "不限" }, ...SchoolData.department],
    value: "不限",
  },
  Year: {
    disabled: false,
    options: [
      { label: "不限", value: 0 },
      { label: "一年級", value: 1 },
      { label: "二年級", value: 2 },
      { label: "三年級", value: 3 },
      { label: "四年級", value: 4 },
      { label: "碩一", value: 5 },
      { label: "碩二", value: 6 },
    ],
    value: 0,
  },
});

// Computed properties and helper methods
const selectedCourseKeys = computed({
  get: () => SelectedCourseStore.selectedCourseKeys,
  set: (value) => {
    SelectedCourseStore.selectedCourseKeys = value;
  },
});

const columns: TableColumnType<BaseCourse>[] = reactive([
  {
    title: "課程名稱",
    dataIndex: "course_name",
    resizable: true,
    minWidth: 200,
    maxWidth: 500,
    width: 0,
  },
  { title: "上課時間", dataIndex: "course_time", resizable: true, width: 0 },
  { title: "", dataIndex: "credits", width: 0 },
]);

const getRequirementTagColor = (record: BaseCourse) => {
  return record.requirement_type === "必" ? "red" : "cyan";
};

const getRowClassName = (record: BaseCourse) => {
  return checkCourseTimeConflictByCourse(record) ? "ConflictCourse" : "";
};

// Locale settings for the table
const localeSettings = {
  emptyText: "沒有課程",
  filterConfirm: "確定",
  filterReset: "重置",
  filterTitle: "篩選",
  selectAll: "全選",
  selectInvert: "反選",
  sortTitle: "排序",
};

// Pagination settings for the table
const paginationSettings = {
  position: ["bottomRight"],
  defaultPageSize: 20,
  onChange: (_page: any, _pageSize: any) => {
    logger.info("Page Changed");
  },
};

// Table row selection settings
const rowSelection = computed(() => ({
  hideSelectAll: true,
  selectedRowKeys: selectedCourseKeys.value,
  preserveSelectedRowKeys: true,
  onChange: (selectedRowKeys: string[], _selectedRows: BaseCourse[]) => {
    logger.info("SelectedRowKeys changed: ", selectedRowKeys);
    selectedCourseKeys.value = selectedRowKeys;
    selectedCourses.value =
      data.value?.filter((course) =>
        selectedCourseKeys.value.includes(course.key)
      ) || [];
  },
  getCheckboxProps: (record: BaseCourse) => ({
    disabled: selectedCourseKeys.value.includes(record.key)
      ? false
      : checkCourseTimeConflict(record.course_time),
    name: record.course_name,
  }),
}));

// Helper methods
const departmentSelect = (value: string) => {
  const isDisabled = SchoolData.department.some((dept) =>
    dept.options.some((option) => option.value === value)
  );
  filter.Year.disabled = isDisabled;
  if (isDisabled) filter.Year.value = 0;
};

const handleResizeColumn = (w: number, col: { width: number }) => {
  col.width = w;
};

const showCourseModal = (record: BaseCourse) => {
  logger.info("Show Course Modal", record);
};

const checkCourseTimeConflictByCourse = (course: BaseCourse) => {
  return selectedCourseKeys.value.includes(course.key)
    ? false
    : checkCourseTimeConflict(course.course_time);
};

const checkCourseTimeConflict = (course_time: BaseCourse["course_time"]) => {
  if (!course_time) return false;
  let conflict = false;
  try {
    for (const selectedCourse of selectedCourses.value) {
      for (const selectedCourseTime of selectedCourse.course_time) {
        if (selectedCourseTime[0] === course_time[0][0]) {
          if (
            selectedCourseTime[1].some((hour) =>
              course_time[0][1].includes(hour)
            )
          ) {
            conflict = true;
            break;
          }
        }
      }
    }
    return conflict;
  } catch (e) {
    logger.error(
      "Error in checkCourseTimeConflict",
      e,
      course_time,
      selectedCourses.value,
      selectedCourseKeys.value
    );
    return false;
  }
};
const emits = defineEmits<{
  (event: "refresh"): void;
}>();

// Filtered data computation
const filteredData = computed(() => {
  if (!data.value) return [];
  let filtered = data.value.filter(
    (course) =>
      course.course_name.includes(filter.search) ||
      `${course.department}${course.course_id}`.includes(filter.search)
  );
  if (filter.category.value !== "不限") {
    filtered = filtered.filter(
      (course) => course.department === filter.category.value.toUpperCase()
    );
  }
  if (filter.Year.value !== 0) {
    filtered = filtered.filter((course) => course.grade === filter.Year.value);
  }
  return filtered;
});

// Watch for search changes
watch(
  () => filter.tempSearch,
  (newVal, oldVal) => {
    if (!newVal && oldVal) {
      filter.search = "";
    }
  }
);

const updateSearch = () => {
  filter.search = filter.tempSearch;
};
</script>

<style lang="scss">
.ant-table-row-selected:hover {
  .ant-table-cell {
    background-color: #00000020 !important;
  }
}

// all ant-table elements
.ant-table-cell {
  font-family: "JetBrains Mono";
  span {
    font-family: "JetBrains Mono";
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
