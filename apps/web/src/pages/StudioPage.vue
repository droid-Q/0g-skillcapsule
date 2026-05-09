<script setup lang="ts">
import { computed, ref } from "vue";
import { useRouter } from "vue-router";
import { Loader2, UploadCloud, WandSparkles } from "lucide-vue-next";
import { createCapsule } from "../api";
import { uploadFileAsset, uploadJsonAsset } from "../storage";
import { connectMetaMask, getConnectedMetaMaskAccount } from "../wallet";

const router = useRouter();

interface TemplateOption {
  label: string;
  title: string;
  tagline: string;
  systemPrompt: string;
  resultSchema: string;
  theme: string;
}

const templates: TemplateOption[] = [
  {
    label: "Pitch capsule",
    title: "Pitch Doctor",
    tagline: "Turn a rough build idea into a founder-grade pitch.",
    systemPrompt: "Turn messy project notes into a sharp, investor-ready narrative with a credible product arc.",
    resultSchema: "hook: string, elevatorPitch: string, demoTalkingPoints: string[], closingLine: string",
    theme: "editorial sunburst"
  },
  {
    label: "Grant capsule",
    title: "Grant Copilot",
    tagline: "Generate strategic hackathon and grant answers without sounding generic.",
    systemPrompt: "Write clear application language that emphasizes traction, architecture, and why this matters now.",
    resultSchema: "problem: string, applicationSummary: string, milestones: string[], whyNow: string",
    theme: "graphite citrus"
  },
  {
    label: "Token capsule",
    title: "Token Explainer",
    tagline: "Explain token design in plain English with balanced risk framing.",
    systemPrompt: "Translate token mechanics and governance ideas into an accessible explanation with practical risk notes.",
    resultSchema: "plainEnglishSummary: string, strengths: string[], risks: string[], recommendedNarrative: string",
    theme: "mint evidence"
  }
] ;

const selectedTemplate = ref(templates[0]);
const title = ref(selectedTemplate.value.title);
const tagline = ref(selectedTemplate.value.tagline);
const systemPrompt = ref(selectedTemplate.value.systemPrompt);
const resultSchema = ref(selectedTemplate.value.resultSchema);
const theme = ref(selectedTemplate.value.theme);
const knowledgeFile = ref<File | null>(null);
const coverFile = ref<File | null>(null);
const creatorWallet = ref("");
const loading = ref(false);
const error = ref("");
const uploadSummary = ref<string[]>([]);

const canSubmit = computed(() =>
  Boolean(title.value.trim() && tagline.value.trim() && systemPrompt.value.trim() && resultSchema.value.trim() && knowledgeFile.value)
);

function applyTemplate(index: number) {
  selectedTemplate.value = templates[index];
  title.value = selectedTemplate.value.title;
  tagline.value = selectedTemplate.value.tagline;
  systemPrompt.value = selectedTemplate.value.systemPrompt;
  resultSchema.value = selectedTemplate.value.resultSchema;
  theme.value = selectedTemplate.value.theme;
}

async function hydrateWallet() {
  creatorWallet.value = creatorWallet.value || (await getConnectedMetaMaskAccount()) || (await connectMetaMask());
}

async function submitCapsule() {
  if (!canSubmit.value || !knowledgeFile.value) return;
  loading.value = true;
  error.value = "";
  uploadSummary.value = [];

  try {
    await hydrateWallet();
    const knowledge = await uploadFileAsset(knowledgeFile.value);
    uploadSummary.value.push(`${knowledge.label}: ${knowledge.rootHash}`);

    const cover = coverFile.value ? await uploadFileAsset(coverFile.value) : null;
    if (cover) {
      uploadSummary.value.push(`${cover.label}: ${cover.rootHash}`);
    }

    const manifest = {
      title: title.value,
      tagline: tagline.value,
      systemPrompt: systemPrompt.value,
      knowledgeRoots: [knowledge.rootHash],
      resultSchema: resultSchema.value,
      creatorWallet: creatorWallet.value,
      agentTokenId: import.meta.env.VITE_DEFAULT_AGENT_TOKEN_ID ?? "0",
      coverAssetRoot: cover?.rootHash ?? null
    };

    const manifestUpload = await uploadJsonAsset(`${title.value}-manifest.json`, manifest);
    uploadSummary.value.push(`manifest: ${manifestUpload.rootHash}`);

    const capsule = await createCapsule({
      theme: theme.value,
      manifestRoot: manifestUpload.rootHash,
      manifest
    });
    await router.push(`/capsules/${capsule.id}`);
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Capsule creation failed.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <section class="studio-grid">
    <article class="panel">
      <p>creator studio</p>
      <h2>Compose a capsule, pin its knowledge pack, then ship the manifest with proof.</h2>
      <div class="template-row">
        <button
          v-for="(template, index) in templates"
          :key="template.label"
          type="button"
          :class="{ active: selectedTemplate.label === template.label }"
          @click="applyTemplate(index)"
        >
          <WandSparkles :size="15" />
          {{ template.label }}
        </button>
      </div>
    </article>

    <article class="panel form-panel">
      <label>
        Title
        <input v-model="title" />
      </label>
      <label>
        Tagline
        <input v-model="tagline" />
      </label>
      <label>
        Theme
        <input v-model="theme" />
      </label>
      <label>
        Creator wallet
        <input v-model="creatorWallet" placeholder="Connect or paste a wallet" />
      </label>
      <label>
        System prompt
        <textarea v-model="systemPrompt" rows="6"></textarea>
      </label>
      <label>
        Result schema
        <textarea v-model="resultSchema" rows="5"></textarea>
      </label>
      <label>
        Knowledge file
        <input type="file" accept=".txt,.md,.pdf,.json" @change="knowledgeFile = ($event.target as HTMLInputElement).files?.[0] ?? null" />
      </label>
      <label>
        Cover asset
        <input type="file" accept="image/*" @change="coverFile = ($event.target as HTMLInputElement).files?.[0] ?? null" />
      </label>
      <button class="submit" type="button" :disabled="loading || !canSubmit" @click="submitCapsule">
        <Loader2 v-if="loading" :size="16" class="spin" />
        <UploadCloud v-else :size="16" />
        Upload to 0G and create capsule
      </button>
      <p v-if="error" class="error">{{ error }}</p>
      <ul v-if="uploadSummary.length" class="summary">
        <li v-for="item in uploadSummary" :key="item">{{ item }}</li>
      </ul>
    </article>
  </section>
</template>

<style scoped>
.studio-grid {
  display: grid;
  grid-template-columns: minmax(320px, 0.9fr) minmax(0, 1.2fr);
  gap: 18px;
}

.panel {
  padding: 24px;
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(12, 17, 16, 0.84);
}

.panel p,
.panel h2 {
  margin: 0;
}

.panel p {
  color: var(--mint);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.78rem;
}

.panel h2 {
  margin-top: 10px;
  font-size: clamp(1.8rem, 3vw, 2.8rem);
  line-height: 1.03;
}

.template-row {
  margin-top: 20px;
  display: grid;
  gap: 10px;
}

.template-row button,
.submit {
  display: inline-flex;
  gap: 8px;
  align-items: center;
  justify-content: center;
  padding: 13px 14px;
  border-radius: 18px;
  border: 1px solid var(--line);
  background: transparent;
  color: var(--sand);
  cursor: pointer;
}

.template-row button.active {
  background: rgba(142, 245, 198, 0.12);
}

.form-panel {
  display: grid;
  gap: 14px;
}

label {
  display: grid;
  gap: 8px;
  color: rgba(244, 239, 226, 0.74);
}

input,
textarea {
  width: 100%;
  padding: 14px 16px;
  border-radius: 18px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(6, 8, 8, 0.88);
  color: var(--sand);
}

.submit {
  margin-top: 8px;
  background: var(--sand);
  color: #111412;
  font-weight: 700;
}

.submit:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.summary {
  margin: 0;
  padding-left: 18px;
  color: rgba(244, 239, 226, 0.78);
}

.error {
  margin: 0;
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
  .studio-grid {
    grid-template-columns: 1fr;
  }
}
</style>
