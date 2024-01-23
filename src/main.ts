import { createApp } from "vue";
import { router } from "./router";
import VueFeather from "vue-feather";
import { createPinia } from "pinia";

import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

import "./styles.css";
import "./variables.scss";

import App from "./App.vue";
const pinia = createPinia();
const app = createApp(App);


pinia.use(piniaPluginPersistedstate);
app.use(router);
app.use(pinia);

app.component(VueFeather.name, VueFeather);
app.mount("#app");
