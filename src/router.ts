import { createRouter, createWebHashHistory } from "vue-router";
import Home from "./views/Home.vue";
import About from "./views/About.vue";
import Settings from "./views/Settings.vue";
import CourseSelection from "./views/CourseSelection.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", component: Home, meta: { title: "Home" } },
    { path: "/about", component: About, meta: { title: "About" } },
    { path: "/settings", component: Settings, meta: { title: "Settings" } },
    {
      path: "/course-selection",
      component: CourseSelection,
      meta: { title: "Course Selection" },
    },
  ],
});
