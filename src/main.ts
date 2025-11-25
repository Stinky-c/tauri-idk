import { debug } from "./utils";
import { createApp } from "vue";
import App from "./App.vue";
import "./app.css";

//#region router
import { createMemoryHistory, createRouter } from "vue-router";

import AboutView from "./pages/AboutView.vue";
import LandingView from "./pages/LandingView.vue";

const routes = [
    { path: "/", component: LandingView },
    { path: "/about", component: AboutView },
];

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
});
router.afterEach((to, from) => {
    debug(`Navigated from '${from.fullPath}' to '${to.fullPath}'`);
});
//#endregion

createApp(App).use(router).mount("#app");
