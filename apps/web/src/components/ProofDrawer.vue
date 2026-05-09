<script setup lang="ts">
import { computed } from "vue";
import { Link2, ShieldCheck, X } from "lucide-vue-next";
import type { ProofRecord } from "../types";

const props = defineProps<{
  open: boolean;
  proof: ProofRecord;
  resultStorageRoot?: string | null;
}>();

const emit = defineEmits<{
  close: [];
}>();

const rows = computed(() => [
  ["Manifest root", props.proof.manifestRoot],
  ["Knowledge roots", props.proof.knowledgeRoots.join("\n")],
  ["Registry contract", props.proof.registryContract ?? "Pending publish"],
  ["Publish tx", props.proof.publishTxHash ?? "Pending publish"],
  ["Result storage", props.resultStorageRoot ?? props.proof.resultStorageRoot ?? "Not pinned yet"],
  ["Compute mode", props.proof.computeMode],
  ["Model", props.proof.computeModel],
  ["Proof hash", props.proof.proofHash]
]);
</script>

<template>
  <transition name="drawer">
    <aside v-if="open" class="drawer">
      <div class="backdrop" @click="emit('close')"></div>
      <section class="panel">
        <header>
          <div>
            <p>Proof Drawer</p>
            <h3>Everything the judge needs to trust the run.</h3>
          </div>
          <button type="button" @click="emit('close')">
            <X :size="18" />
          </button>
        </header>
        <div class="summary">
          <span><ShieldCheck :size="16" /> verifiable artifacts</span>
          <span><Link2 :size="16" /> chain + storage + compute</span>
        </div>
        <dl>
          <template v-for="[label, value] in rows" :key="label">
            <dt>{{ label }}</dt>
            <dd>{{ value }}</dd>
          </template>
        </dl>
        <a v-if="proof.explorerUrl" class="explorer" :href="proof.explorerUrl" target="_blank" rel="noreferrer">
          Open explorer proof
        </a>
      </section>
    </aside>
  </transition>
</template>

<style scoped>
.drawer {
  position: fixed;
  inset: 0;
  z-index: 30;
}

.backdrop {
  position: absolute;
  inset: 0;
  background: rgba(3, 6, 6, 0.7);
  backdrop-filter: blur(10px);
}

.panel {
  position: absolute;
  top: 18px;
  right: 18px;
  bottom: 18px;
  width: min(520px, calc(100vw - 24px));
  padding: 24px;
  border-radius: 32px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(14, 18, 18, 0.96);
  overflow: auto;
}

header,
.summary {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
}

header p,
header h3 {
  margin: 0;
}

header p {
  color: var(--mint);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.76rem;
}

header h3 {
  margin-top: 6px;
  font-size: 1.3rem;
}

header button {
  display: inline-grid;
  place-items: center;
  width: 40px;
  height: 40px;
  border-radius: 999px;
  border: 1px solid var(--line);
  background: transparent;
  color: var(--sand);
  cursor: pointer;
}

.summary {
  margin: 18px 0 22px;
  flex-wrap: wrap;
}

.summary span,
.explorer {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 999px;
  background: rgba(142, 245, 198, 0.08);
}

dl {
  margin: 0;
  display: grid;
  gap: 12px;
}

dt {
  color: rgba(244, 239, 226, 0.56);
  font-size: 0.84rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
}

dd {
  margin: 0;
  white-space: pre-wrap;
  padding: 14px 16px;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.04);
  line-height: 1.55;
  word-break: break-word;
}

.explorer {
  margin-top: 18px;
  justify-content: center;
}

.drawer-enter-active,
.drawer-leave-active {
  transition: opacity 0.22s ease;
}

.drawer-enter-active .panel,
.drawer-leave-active .panel {
  transition: transform 0.22s ease, opacity 0.22s ease;
}

.drawer-enter-from,
.drawer-leave-to {
  opacity: 0;
}

.drawer-enter-from .panel,
.drawer-leave-to .panel {
  transform: translateX(18px);
  opacity: 0;
}
</style>

