<script setup lang="ts">
import { onMounted, ref } from "vue";
import { RouterLink } from "vue-router";
import { ArrowRight, Boxes, Cpu, ShieldEllipsis, Sparkle } from "lucide-vue-next";
import CapsuleCard from "../components/CapsuleCard.vue";
import { listCapsules } from "../api";
import type { Capsule } from "../types";

const capsules = ref<Capsule[]>([]);
const loading = ref(true);
const error = ref("");

onMounted(async () => {
  try {
    capsules.value = await listCapsules();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Failed to load capsules.";
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <section class="hero-grid">
    <article class="hero-card hero-main">
      <p>judge-facing narrative</p>
      <h2>Capsules package prompt, knowledge, chain proof, and reusable execution into one clean artifact.</h2>
      <div class="hero-actions">
        <RouterLink class="primary" to="/studio">
          Build your capsule
          <ArrowRight :size="16" />
        </RouterLink>
        <RouterLink class="secondary" :to="capsules[0] ? `/capsules/${capsules[0].id}` : '/'">
          View live demo capsule
        </RouterLink>
      </div>
    </article>
    <article class="hero-card hero-metrics">
      <div>
        <span>0G Compute</span>
        <strong>OpenAI-compatible router</strong>
      </div>
      <div>
        <span>0G Storage</span>
        <strong>Browser-side knowledge pinning</strong>
      </div>
      <div>
        <span>Onchain proof</span>
        <strong>Minimal registry with explorer trail</strong>
      </div>
    </article>
  </section>

  <section class="strip">
    <div><Boxes :size="16" /> marketplace-native</div>
    <div><Cpu :size="16" /> agent-as-a-service</div>
    <div><ShieldEllipsis :size="16" /> proof-first UX</div>
    <div><Sparkle :size="16" /> demo-ready outputs</div>
  </section>

  <section class="section-head">
    <div>
      <p>capsule library</p>
      <h2>High-signal capsules for founders, grant writers, and token teams.</h2>
    </div>
    <RouterLink to="/studio">Create one from scratch</RouterLink>
  </section>

  <p v-if="error" class="error">{{ error }}</p>
  <p v-else-if="loading" class="muted">Loading live capsules...</p>

  <section v-else class="grid">
    <RouterLink v-for="capsule in capsules" :key="capsule.id" :to="`/capsules/${capsule.id}`">
      <CapsuleCard :capsule="capsule" />
    </RouterLink>
  </section>
</template>

<style scoped>
.hero-grid,
.grid {
  display: grid;
  gap: 18px;
}

.hero-grid {
  grid-template-columns: minmax(0, 1.5fr) minmax(280px, 0.9fr);
}

.hero-card {
  border-radius: 34px;
  padding: 26px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(13, 18, 18, 0.8);
}

.hero-main {
  background:
    linear-gradient(140deg, rgba(142, 245, 198, 0.1), transparent 42%),
    linear-gradient(180deg, rgba(245, 179, 107, 0.1), transparent 60%),
    rgba(13, 18, 18, 0.88);
}

.hero-main p,
.section-head p {
  margin: 0;
  color: var(--amber);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.76rem;
}

.hero-main h2,
.section-head h2 {
  margin: 14px 0 0;
  max-width: 760px;
  font-size: clamp(1.8rem, 3vw, 3rem);
  line-height: 1.02;
}

.hero-actions {
  margin-top: 24px;
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.primary,
.secondary,
.section-head a {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 13px 16px;
  border-radius: 999px;
}

.primary {
  background: var(--sand);
  color: #111412;
  font-weight: 700;
}

.secondary,
.section-head a {
  border: 1px solid var(--line);
  color: rgba(244, 239, 226, 0.88);
}

.hero-metrics {
  display: grid;
  gap: 16px;
  align-content: center;
}

.hero-metrics span {
  color: rgba(244, 239, 226, 0.6);
  display: block;
  margin-bottom: 6px;
}

.hero-metrics strong {
  font-size: 1.12rem;
}

.strip,
.section-head {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
}

.strip {
  margin: 18px 0 34px;
}

.strip div {
  display: inline-flex;
  gap: 8px;
  align-items: center;
  padding: 10px 14px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.04);
}

.section-head {
  margin-bottom: 16px;
}

.grid {
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
}

.muted,
.error {
  margin: 12px 0 0;
}

.error {
  color: var(--danger);
}

@media (max-width: 920px) {
  .hero-grid {
    grid-template-columns: 1fr;
  }
}
</style>

