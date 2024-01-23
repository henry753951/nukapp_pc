import { createRouter, createWebHashHistory } from "vue-router";
import Home from "./views/Home.vue";
import About from "./views/About.vue";
import Settings from "./views/Settings.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", component: Home },
    { path: "/about", component: About },
    { path: "/settings", component: Settings },
  ],
});
