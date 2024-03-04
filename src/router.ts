import { createRouter, createWebHashHistory } from "vue-router";
import Home from "./views/Home.vue";
import About from "./views/About.vue";
import Settings from "./views/Settings.vue";
import CourseSelection from "./views/CourseSelection.vue";
import ScoreView from "./views/ScoreView.vue";
import QQ from "./views/QQ.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", component: Home, meta: { title: "Home", breadcrumb: [] } },
    {
      path: "/about",
      component: About,
      meta: { title: "About", breadcrumb: ["關於"] },
    },
    {
      path: "/QQ",
      component: QQ,
      meta: { title: "QQ", breadcrumb: ["關於一代PWA"] },
    },
    {
      path: "/settings",
      component: Settings,
      meta: { title: "Settings", breadcrumb: ["設定"] },
    },
    {
      path: "/course-selection",
      component: CourseSelection,
      meta: { title: "Course Selection", breadcrumb: ["課程查詢"] },
    },
    {
      path: "/score",
      component: ScoreView,
      meta: { title: "Score", breadcrumb: ["成績查詢˙"] },
    },
  ],
});
