import { debug } from "./utils";

import { createMemoryHistory, createRouter } from "vue-router";
import { routes, handleHotUpdate } from "vue-router/auto-routes";

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
    linkActiveClass: "link-success",
});

router.afterEach((to, from) => {
    debug(`Navigated from '${from.fullPath}' to '${to.fullPath}'`);
});
if (import.meta.hot) {
    handleHotUpdate(router);
}
