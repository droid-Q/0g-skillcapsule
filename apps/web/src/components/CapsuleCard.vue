<script setup lang="ts">
import { computed } from "vue";
import { ArrowUpRight, Layers3, ShieldCheck, Sparkles } from "lucide-vue-next";
import type { Capsule } from "../types";

const props = defineProps<{
  capsule: Capsule;
}>();

const badges = computed(() => [
  props.capsule.status,
  `${props.capsule.manifest.knowledgeRoots.length} roots`,
  `v${props.capsule.version}`
]);
</script>

<template>
  <article class="card">
    <div class="card-top">
      <p>{{ capsule.theme }}</p>
      <ArrowUpRight :size="16" />
    </div>
    <h3>{{ capsule.manifest.title }}</h3>
    <p class="tagline">{{ capsule.manifest.tagline }}</p>
    <div class="fact-row">
      <span><Sparkles :size="15" /> Agentic output</span>
      <span><Layers3 :size="15" /> 0G storage roots</span>
      <span><ShieldCheck :size="15" /> proof drawer</span>
    </div>
    <div class="badges">
      <span v-for="badge in badges" :key="badge">{{ badge }}</span>
    </div>
  </article>
</template>

<style scoped>
.card {
  padding: 24px;
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background:
    linear-gradient(180deg, rgba(20, 26, 25, 0.96), rgba(11, 14, 14, 0.92)),
    rgba(15, 18, 18, 0.84);
  box-shadow: 0 28px 60px rgba(0, 0, 0, 0.26);
  min-height: 280px;
}

.card-top,
.fact-row,
.badges {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  align-items: center;
  justify-content: space-between;
}

.card-top p,
.tagline {
  margin: 0;
}

.card-top p {
  color: var(--mint);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 0.74rem;
}

h3 {
  margin: 18px 0 10px;
  font-size: 1.45rem;
}

.tagline {
  min-height: 74px;
  color: rgba(244, 239, 226, 0.72);
  line-height: 1.6;
}

.fact-row {
  margin-top: 20px;
  justify-content: flex-start;
}

.fact-row span,
.badges span {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 9px 12px;
  border-radius: 999px;
}

.fact-row span {
  background: rgba(142, 245, 198, 0.08);
  color: rgba(244, 239, 226, 0.82);
}

.badges {
  margin-top: 18px;
  justify-content: flex-start;
}

.badges span {
  background: rgba(245, 179, 107, 0.08);
  color: var(--amber);
}
</style>

