import { createRouter, createWebHistory } from "vue-router";
import MarketplacePage from "./pages/MarketplacePage.vue";
import CapsuleDetailPage from "./pages/CapsuleDetailPage.vue";
import StudioPage from "./pages/StudioPage.vue";
import RunPage from "./pages/RunPage.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", name: "marketplace", component: MarketplacePage },
    { path: "/capsules/:id", name: "capsule-detail", component: CapsuleDetailPage, props: true },
    { path: "/studio", name: "studio", component: StudioPage },
    { path: "/runs/:id", name: "run-detail", component: RunPage, props: true }
  ]
});

