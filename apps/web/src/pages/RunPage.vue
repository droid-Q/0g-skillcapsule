<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import { CheckCheck, DatabaseZap, Loader2 } from "lucide-vue-next";
import { getRun } from "../api";
import ProofDrawer from "../components/ProofDrawer.vue";
import { uploadJsonAsset } from "../storage";
import type { CapsuleRun } from "../types";

const route = useRoute();

const run = ref<CapsuleRun | null>(null);
const loading = ref(true);
const pinning = ref(false);
const error = ref("");
const showProof = ref(false);
const resultStorageRoot = ref<string | null>(null);
let pollTimer: number | null = null;

const runId = computed(() => route.params.id as string);

onMounted(async () => {
  resultStorageRoot.value = window.localStorage.getItem(`skillcapsule-run-proof:${runId.value}`);
  await refreshRun();
});

watch(runId, async () => {
  resultStorageRoot.value = window.localStorage.getItem(`skillcapsule-run-proof:${runId.value}`);
  await refreshRun();
});

onBeforeUnmount(() => {
  if (pollTimer) window.clearTimeout(pollTimer);
});

async function refreshRun() {
  loading.value = true;
  try {
    run.value = await getRun(runId.value);
    if (run.value.status === "pending" || run.value.status === "running") {
      pollTimer = window.setTimeout(refreshRun, 1200);
    } else if (run.value.status === "completed" && run.value.outputJson && !resultStorageRoot.value && !pinning.value) {
      await pinResult();
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Failed to load run.";
  } finally {
    loading.value = false;
  }
}

async function pinResult() {
  if (!run.value?.outputJson) return;
  pinning.value = true;
  try {
    const upload = await uploadJsonAsset(`${run.value.id}-result.json`, run.value.outputJson);
    resultStorageRoot.value = upload.rootHash;
    window.localStorage.setItem(`skillcapsule-run-proof:${run.value.id}`, upload.rootHash);
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Failed to pin result.";
  } finally {
    pinning.value = false;
  }
}
</script>

<template>
  <p v-if="error" class="error">{{ error }}</p>
  <p v-else-if="loading && !run" class="muted">Loading run result...</p>

  <template v-else-if="run">
    <section class="run-grid">
      <article class="hero">
        <p>run status</p>
        <h2>{{ run.capsuleTitle }}</h2>
        <p class="status">
          <CheckCheck :size="16" />
          {{ run.status }}
        </p>
        <div class="actions">
          <button class="proof" type="button" @click="showProof = true">Open proof drawer</button>
          <button class="pin" type="button" :disabled="pinning || !run.outputJson" @click="pinResult">
            <Loader2 v-if="pinning" :size="16" class="spin" />
            <DatabaseZap v-else :size="16" />
            {{ resultStorageRoot ? "Repin result to 0G Storage" : "Pin result to 0G Storage" }}
          </button>
        </div>
        <p class="task-label">Task</p>
        <p class="task">{{ run.taskInput }}</p>
        <p v-if="resultStorageRoot" class="storage-root">Result root: {{ resultStorageRoot }}</p>
      </article>

      <article class="panel">
        <h3>Structured output</h3>
        <pre>{{ run.outputText ?? "The run is still in progress." }}</pre>
      </article>
    </section>

    <article class="panel warnings">
      <h3>Warnings and audit notes</h3>
      <ul>
        <li v-for="warning in run.warnings" :key="warning">{{ warning }}</li>
      </ul>
    </article>

    <ProofDrawer :open="showProof" :proof="run.proof" :result-storage-root="resultStorageRoot" @close="showProof = false" />
  </template>
</template>

<style scoped>
.run-grid {
  display: grid;
  grid-template-columns: minmax(320px, 0.9fr) minmax(0, 1.1fr);
  gap: 18px;
}

.hero,
.panel {
  padding: 24px;
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(12, 17, 16, 0.84);
}

.hero p,
.hero h2,
.panel h3,
.task,
.storage-root {
  margin: 0;
}

.hero > p:first-child {
  color: var(--mint);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.76rem;
}

.hero h2 {
  margin-top: 8px;
  font-size: clamp(1.8rem, 4vw, 3rem);
}

.status,
.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.status {
  margin-top: 12px;
  align-items: center;
  color: var(--amber);
}

.actions {
  margin-top: 18px;
}

.proof,
.pin {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 999px;
  border: 1px solid var(--line);
  background: transparent;
  color: var(--sand);
  cursor: pointer;
}

.pin {
  background: rgba(142, 245, 198, 0.12);
}

.task-label {
  margin-top: 20px;
  color: rgba(244, 239, 226, 0.58);
  text-transform: uppercase;
  letter-spacing: 0.07em;
}

.task {
  margin-top: 10px;
  line-height: 1.7;
}

.storage-root {
  margin-top: 18px;
  color: var(--mint);
  word-break: break-word;
}

pre {
  margin: 0;
  white-space: pre-wrap;
  padding: 18px;
  border-radius: 22px;
  background: rgba(6, 8, 8, 0.88);
  min-height: 380px;
  line-height: 1.6;
}

.warnings {
  margin-top: 18px;
}

.warnings ul {
  margin: 0;
  padding-left: 18px;
  color: rgba(244, 239, 226, 0.78);
}

.muted,
.error {
  margin: 0 0 12px;
}

.error {
  color: var(--danger);
}

.spin {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 920px) {
  .run-grid {
    grid-template-columns: 1fr;
  }
}
</style>

