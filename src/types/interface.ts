interface BaseCourse {
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
interface Course extends BaseCourse {
  custom_Data: CustomData;
}

interface CustomData {
  course_name: string | null;
  location: string | null;
  teacher: string | null;
  time: string | null;
  requirement_type: string | null;
  //
  students_note: string;
  color: string;
}



export type { BaseCourse, Course, CustomData };
