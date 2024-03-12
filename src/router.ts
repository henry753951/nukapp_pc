import { createRouter, createWebHashHistory } from "vue-router";
import Home from "./views/Home.vue";
import About from "./views/About.vue";
import Settings from "./views/Settings.vue";
import CourseSelection from "./views/CourseSelection.vue";
import ScoreView from "./views/ScoreView.vue";
import NotAuthorized from "./views/NotAuthorized.vue";
import md from "./views/md.vue";
import { useUserStore } from "./stores/user";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", component: Home, meta: { title: "Home", breadcrumb: [] } },
    {
      path: "/unauthorized",
      component: NotAuthorized,
      meta: { title: "未登入", breadcrumb: [] },
    },
    {
      path: "/dev",
      component: () => import("./views/ComponentDev.vue"),
    },
    {
      path: "/about",
      component: About,
      meta: { title: "About", breadcrumb: ["關於"] },
    },
    {
      name: "markdown",
      path: "/md",
      component: md,
      meta: { title: "", breadcrumb: [] },
    },
    {
      name: "settings",
      path: "/settings",
      component: Settings,
      meta: { title: "Settings", breadcrumb: ["設定"] },
    },
    {
      name: "course-selection",
      path: "/course-selection",
      component: CourseSelection,
      meta: { title: "Course Selection", breadcrumb: ["課程查詢"] },
    },
    {
      name: "score",
      path: "/score",
      component: ScoreView,
      meta: { title: "Score", breadcrumb: ["成績查詢"], needAuth: true },
    },
  ],
});

router.beforeEach((to, _from, next) => {
  const UserStore = useUserStore();
  if (typeof to.query.breadcrumb === "string") {
    to.meta.breadcrumb = to.query.breadcrumb.split(",");
  }

  if (to.meta.needAuth) {
    if (UserStore.user.isLoggedIn) {
      next();
      return;
    }
    next({
      path: "/unauthorized",
      query: {
        nextUrl: to.fullPath,
      },
    });
    return;
  }
  next();
  return;
});
