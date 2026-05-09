<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { CheckCircle2, Loader2, Rocket, ScrollText, ShieldCheck, UploadCloud } from "lucide-vue-next";
import { getCapsule, publishCapsule, runCapsule } from "../api";
import ProofDrawer from "../components/ProofDrawer.vue";
import { publishCapsuleOnchain } from "../registry";
import { connectMetaMask, formatWalletAddress, getConnectedMetaMaskAccount } from "../wallet";
import type { Capsule } from "../types";

const route = useRoute();
const router = useRouter();

const capsule = ref<Capsule | null>(null);
const loading = ref(true);
const error = ref("");
const taskInput = ref("Turn this capsule into a 3-minute demo with persuasive talking points and a clean result structure.");
const showProof = ref(false);
const running = ref(false);
const publishing = ref(false);
const walletAddress = ref<string | null>(null);
const statusMessage = ref("");

const capsuleId = computed(() => route.params.id as string);

onMounted(async () => {
  walletAddress.value = await getConnectedMetaMaskAccount().catch(() => null);
  await loadCapsule();
});

watch(capsuleId, async () => {
  await loadCapsule();
});

async function loadCapsule() {
  loading.value = true;
  error.value = "";
  try {
    capsule.value = await getCapsule(capsuleId.value);
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Failed to load capsule.";
  } finally {
    loading.value = false;
  }
}

async function ensureWallet() {
  walletAddress.value = walletAddress.value ?? (await connectMetaMask());
}

async function publishNow() {
  if (!capsule.value) return;
  publishing.value = true;
  error.value = "";
  statusMessage.value = "";
  try {
    await ensureWallet();
    const proof = await publishCapsuleOnchain({
      capsuleId: capsule.value.id,
      manifestRoot: capsule.value.manifestRoot,
      agentTokenId: capsule.value.manifest.agentTokenId,
      version: capsule.value.version + 1
    });
    capsule.value = await publishCapsule(capsule.value.id, {
      registryContract: proof.registryContract,
      txHash: proof.txHash,
      explorerUrl: proof.explorerUrl,
      agentTokenId: capsule.value.manifest.agentTokenId,
      version: capsule.value.version + 1
    });
    statusMessage.value =
      proof.mode === "demo"
        ? "Published in demo mode. Add a registry address to send a real transaction."
        : "Published onchain and synced back to the catalog.";
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Publish failed.";
  } finally {
    publishing.value = false;
  }
}

async function runNow() {
  if (!capsule.value) return;
  running.value = true;
  error.value = "";
  try {
    const run = await runCapsule(capsule.value.id, { taskInput: taskInput.value });
    await router.push(`/runs/${run.id}`);
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Run failed.";
  } finally {
    running.value = false;
  }
}
</script>

<template>
  <p v-if="error" class="error">{{ error }}</p>
  <p v-else-if="loading" class="muted">Loading capsule detail...</p>

  <template v-else-if="capsule">
    <section class="detail-grid">
      <article class="hero">
        <p>{{ capsule.theme }}</p>
        <h2>{{ capsule.manifest.title }}</h2>
        <p class="tagline">{{ capsule.manifest.tagline }}</p>
        <div class="row">
          <span><ShieldCheck :size="16" /> {{ capsule.status }}</span>
          <span><ScrollText :size="16" /> {{ capsule.manifest.knowledgeRoots.length }} storage roots</span>
          <span><CheckCircle2 :size="16" /> v{{ capsule.version }}</span>
        </div>
        <div class="actions">
          <button class="primary" type="button" :disabled="running" @click="runNow">
            <Loader2 v-if="running" :size="16" class="spin" />
            <Rocket v-else :size="16" />
            Run capsule
          </button>
          <button class="secondary" type="button" :disabled="publishing" @click="publishNow">
            <Loader2 v-if="publishing" :size="16" class="spin" />
            <UploadCloud v-else :size="16" />
            {{ capsule.status === 'published' ? 'Republish proof' : 'Publish to 0G chain' }}
          </button>
          <button class="ghost" type="button" @click="showProof = true">Open proof drawer</button>
        </div>
        <p v-if="walletAddress" class="wallet-hint">Connected as {{ formatWalletAddress(walletAddress) }}</p>
        <p v-if="statusMessage" class="success">{{ statusMessage }}</p>
      </article>

      <article class="panel">
        <h3>System prompt</h3>
        <p>{{ capsule.manifest.systemPrompt }}</p>
        <h3>Result schema</h3>
        <pre>{{ capsule.manifest.resultSchema }}</pre>
      </article>
    </section>

    <section class="lower-grid">
      <article class="panel">
        <h3>Knowledge pack</h3>
        <ul>
          <li v-for="root in capsule.manifest.knowledgeRoots" :key="root">{{ root }}</li>
        </ul>
      </article>
      <article class="panel run-panel">
        <h3>Run input</h3>
        <textarea v-model="taskInput" rows="8"></textarea>
      </article>
    </section>

    <ProofDrawer :open="showProof" :proof="{
      manifestRoot: capsule.manifestRoot,
      knowledgeRoots: capsule.manifest.knowledgeRoots,
      coverAssetRoot: capsule.manifest.coverAssetRoot,
      registryContract: capsule.registryContract,
      publishTxHash: capsule.publishTxHash,
      explorerUrl: capsule.explorerUrl,
      computeMode: '0G Compute Router',
      computeModel: 'configured at runtime',
      proofHash: `${capsule.id}:${capsule.version}`
    }" @close="showProof = false" />
  </template>
</template>

<style scoped>
.detail-grid,
.lower-grid {
  display: grid;
  gap: 18px;
}

.detail-grid {
  grid-template-columns: minmax(0, 1.3fr) minmax(320px, 0.9fr);
}

.hero,
.panel {
  padding: 24px;
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(13, 18, 18, 0.84);
}

.hero {
  background:
    linear-gradient(145deg, rgba(142, 245, 198, 0.09), transparent 38%),
    linear-gradient(190deg, rgba(245, 179, 107, 0.08), transparent 55%),
    rgba(13, 18, 18, 0.9);
}

.hero p,
.hero h2,
.panel h3,
.panel p,
.wallet-hint,
.success {
  margin: 0;
}

.hero > p:first-child {
  color: var(--mint);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 0.78rem;
}

.hero h2 {
  margin-top: 10px;
  font-size: clamp(2rem, 5vw, 3.4rem);
  line-height: 0.96;
}

.tagline {
  margin-top: 12px;
  max-width: 700px;
  color: rgba(244, 239, 226, 0.72);
  line-height: 1.65;
}

.row,
.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.row {
  margin-top: 20px;
}

.row span,
.primary,
.secondary,
.ghost {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  border-radius: 999px;
}

.row span {
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.04);
}

.actions {
  margin-top: 22px;
}

.primary,
.secondary,
.ghost {
  padding: 12px 16px;
  border: 1px solid transparent;
  cursor: pointer;
}

.primary {
  background: var(--sand);
  color: #101413;
}

.secondary {
  background: rgba(142, 245, 198, 0.1);
  color: var(--sand);
  border-color: rgba(142, 245, 198, 0.2);
}

.ghost {
  background: transparent;
  color: rgba(244, 239, 226, 0.8);
  border-color: var(--line);
}

.wallet-hint,
.success {
  margin-top: 14px;
}

.wallet-hint {
  color: rgba(244, 239, 226, 0.64);
}

.success {
  color: var(--mint);
}

.panel h3 {
  margin-bottom: 10px;
}

pre,
textarea,
li {
  color: rgba(244, 239, 226, 0.84);
}

pre,
textarea {
  width: 100%;
  padding: 16px;
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(6, 8, 8, 0.88);
  white-space: pre-wrap;
}

textarea {
  min-height: 240px;
  resize: vertical;
}

ul {
  padding-left: 18px;
}

.lower-grid {
  margin-top: 18px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
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
  .detail-grid,
  .lower-grid {
    grid-template-columns: 1fr;
  }
}
</style>

