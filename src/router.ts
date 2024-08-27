import { createRouter, createWebHashHistory } from "vue-router";

import { useUserStore } from "./stores/user";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      component: () => import("./views/Home.vue"),
      meta: { title: "Home", breadcrumb: [] },
    },
    {
      path: "/unauthorized",
      component: () => import("./views/NotAuthorized.vue"),
      meta: { title: "未登入", breadcrumb: [] },
    },
    {
      path: "/dev",
      component: () => import("./views/ComponentDev.vue"),
    },
    {
      path: "/about",
      component: () => import("./views/About.vue"),
      meta: { title: "About", breadcrumb: ["關於"] },
    },
    {
      name: "markdown",
      path: "/md",
      component: () => import("./views/md.vue"),
      meta: { title: "", breadcrumb: [] },
    },
    {
      name: "settings",
      path: "/settings",
      component: () => import("./views/Settings.vue"),
      meta: { title: "Settings", breadcrumb: ["設定"] },
    },
    {
      name: "course-selection",
      path: "/course-selection",
      component: () => import("./views/CourseSelection.vue"),
      meta: { title: "Course Selection", breadcrumb: ["課程查詢"] },
    },
    {
      name: "score",
      path: "/score",
      component: () => import("./views/ScoreView.vue"),
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
