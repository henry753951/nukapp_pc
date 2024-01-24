<template>
  <a-table
    :row-selection="rowSelection"
    :columns="columns"
    :data-source="data"
    :locale="{
      emptyText: '沒有課程',
      filterConfirm: '確定',
      filterReset: '重置',
      filterTitle: '篩選',
      selectAll: '全選',
      selectInvert: '反選',
      sortTitle: '排序',
    }"
    
    :pagination="{ position: ['bottomRight'],onChange: (_page:any, _pageSize:any) => {scrollToTop()},}"
    sticky
    :loading="loading">
    <template #bodyCell="{ column, text }">
      <template v-if="column.dataIndex === 'name'">
        <a>{{ text }}</a>
      </template>
    </template>
  </a-table>
</template>
<script lang="ts" setup>
  import { invoke } from "@tauri-apps/api/core";
  import type { TableProps, TableColumnType } from "ant-design-vue";
  import { Key } from "ant-design-vue/es/_util/type";
  import { ref, onMounted, inject } from "vue";

  interface DataType {
    key: string;
    department: string;
    course_id: string;
    department_code: string;
    grade: string;
    class_type: string;
    course_name: string;
    syllabus_link: string;
    credits: string;
    requirement_type: string;
    limit: string;
    registration_confirmed: string;
    online_number: string;
    balance: string;
    teacher: string;
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
  ];
  const data = ref<DataType[]>([]);
  const loading = ref(true);

  const rowSelection: TableProps["rowSelection"] = {
    onChange: (selectedRowKeys: Key[], selectedRows: DataType[]) => {
      console.log(
        `selectedRowKeys: ${selectedRowKeys}`,
        "selectedRows: ",
        selectedRows
      );
    },
    getCheckboxProps: (record: DataType) => ({
      disabled: false,
      name: record.course_name,
    }),
  };

  const scrollToTop = inject<() => void>("scrollToTop")!;
  onMounted(async () => {
    let all_course = (await invoke("get_all_course")) as DataType[];
    console.log(all_course);
    data.value = all_course;
    loading.value = false;
  });
</script>
