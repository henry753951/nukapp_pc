<template>
  <a-table :row-selection="rowSelection" :columns="columns" :data-source="data">
    <template #bodyCell="{ column, text }">
      <template v-if="column.dataIndex === 'name'">
        <a>{{ text }}</a>
      </template>
    </template>
  </a-table>
</template>
<script lang="ts" setup>
  import type { TableProps, TableColumnType } from "ant-design-vue";
  import { Key } from "ant-design-vue/es/_util/type";

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
  const data: DataType[] = [];

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
</script>
