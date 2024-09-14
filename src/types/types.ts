type Semester = {
  學期: string;
  修習學分數: string;
  實得學分數: string;
  平均成績: string;
  排名: string;
  選修課程: Course[];
  必修課程: Course[];
};

type Course = {
  課號: string;
  課程名稱: string;
  學分數: string;
  修別: string;
  期中成績: string;
  學期成績: string;
  備註: string;
};

type SchoolYear = {
  學期: string;
  修習學分數: string;
  實得學分數: string;
  平均成績: string;
  排名: string;
  課程: Course[];
};

type StudentGrades = {
  歷年總修習學分數: string;
  歷年實得學分數: string;
  歷年平均成績: string;
  歷年全班排名: string;
  截至學期: string;
  各學期: SchoolYear[];
};
type CourseProgress = {
  選修: { total: number; finished: number };
  必修: { total: number; finished: number };
};
export type { Semester, Course, SchoolYear, StudentGrades, CourseProgress };
