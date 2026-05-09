<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { RouterLink, RouterView, useRoute } from "vue-router";
import { ArrowUpRight, Wallet2 } from "lucide-vue-next";
import { connectMetaMask, formatWalletAddress, getConnectedMetaMaskAccount, getMetaMaskProvider } from "./wallet";

const route = useRoute();
const account = ref<string | null>(null);
const error = ref("");

const routeLabel = computed(() => {
  if (route.name === "studio") return "Creator Studio";
  if (route.name === "run-detail") return "Run Result";
  if (route.name === "capsule-detail") return "Capsule Detail";
  return "Marketplace";
});

onMounted(async () => {
  account.value = await getConnectedMetaMaskAccount().catch(() => null);
  getMetaMaskProvider()?.on?.("accountsChanged", handleAccountsChanged);
});

onBeforeUnmount(() => {
  getMetaMaskProvider()?.removeListener?.("accountsChanged", handleAccountsChanged);
});

function handleAccountsChanged(accounts: string[]) {
  account.value = accounts[0] ?? null;
}

async function connectWallet() {
  error.value = "";
  try {
    account.value = await connectMetaMask();
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Wallet connection failed.";
  }
}
</script>

<template>
  <div class="shell">
    <div class="ambient ambient-a"></div>
    <div class="ambient ambient-b"></div>
    <header class="topbar">
      <RouterLink class="brand" to="/">
        <span class="brand-mark">0G</span>
        <div>
          <p>SkillCapsule</p>
          <small>proof-native agent marketplace</small>
        </div>
      </RouterLink>
      <nav class="nav">
        <RouterLink to="/">Marketplace</RouterLink>
        <RouterLink to="/studio">Creator Studio</RouterLink>
        <a href="https://www.hackquest.io/zh-cn/hackathons/0G-APAC-Hackathon" target="_blank" rel="noreferrer">
          Hackathon
          <ArrowUpRight :size="14" />
        </a>
      </nav>
      <button class="wallet" type="button" @click="connectWallet">
        <Wallet2 :size="16" />
        <span>{{ account ? formatWalletAddress(account) : "Connect wallet" }}</span>
      </button>
    </header>

    <section class="headline">
      <div>
        <p class="kicker">Track 3 · Agentic Economy & Autonomous Applications</p>
        <h1>Build capsules that feel like products, not prompts.</h1>
      </div>
      <div class="route-pill">{{ routeLabel }}</div>
    </section>

    <p v-if="error" class="wallet-error">{{ error }}</p>

    <RouterView />
  </div>
</template>

<style>
@import url("https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;700;800&family=Newsreader:opsz,wght@6..72,500;6..72,700&display=swap");

:root {
  color-scheme: dark;
  --bg: #0d1110;
  --panel: rgba(15, 20, 19, 0.82);
  --panel-strong: rgba(24, 30, 29, 0.96);
  --line: rgba(200, 246, 221, 0.14);
  --mint: #8ef5c6;
  --amber: #f5b36b;
  --sand: #f4efe2;
  --ink: #c7d2cb;
  --danger: #ff8c72;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 320px;
  background:
    radial-gradient(circle at top left, rgba(142, 245, 198, 0.18), transparent 26%),
    radial-gradient(circle at 85% 15%, rgba(245, 179, 107, 0.16), transparent 24%),
    linear-gradient(180deg, #101514, #090b0b 58%, #060707);
  color: var(--sand);
  font-family: "Manrope", sans-serif;
}

a {
  color: inherit;
  text-decoration: none;
}

button,
input,
textarea {
  font: inherit;
}
</style>

<style scoped>
.shell {
  position: relative;
  min-height: 100vh;
  padding: 28px clamp(18px, 3vw, 42px) 48px;
}

.ambient {
  position: fixed;
  inset: auto;
  border-radius: 999px;
  filter: blur(70px);
  opacity: 0.42;
  pointer-events: none;
}

.ambient-a {
  top: 120px;
  right: 8%;
  width: 220px;
  height: 220px;
  background: rgba(142, 245, 198, 0.22);
}

.ambient-b {
  bottom: 0;
  left: -20px;
  width: 240px;
  height: 240px;
  background: rgba(245, 179, 107, 0.16);
}

.topbar,
.headline {
  position: relative;
  z-index: 1;
}

.topbar {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 18px;
  align-items: center;
}

.brand {
  display: inline-flex;
  align-items: center;
  gap: 12px;
}

.brand-mark {
  display: inline-grid;
  place-items: center;
  width: 42px;
  height: 42px;
  border-radius: 14px;
  background: linear-gradient(135deg, rgba(142, 245, 198, 0.26), rgba(245, 179, 107, 0.3));
  border: 1px solid rgba(255, 255, 255, 0.12);
  font-weight: 800;
  letter-spacing: 0.06em;
}

.brand p,
.brand small,
.kicker {
  margin: 0;
}

.brand p {
  font-weight: 800;
}

.brand small {
  color: rgba(244, 239, 226, 0.64);
}

.nav {
  justify-self: center;
  display: inline-flex;
  gap: 10px;
  padding: 8px;
  border-radius: 999px;
  border: 1px solid var(--line);
  background: rgba(16, 21, 20, 0.72);
}

.nav a {
  display: inline-flex;
  gap: 6px;
  align-items: center;
  padding: 10px 14px;
  border-radius: 999px;
  color: rgba(244, 239, 226, 0.8);
}

.nav a.router-link-active {
  background: rgba(142, 245, 198, 0.14);
  color: var(--sand);
}

.wallet {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-radius: 999px;
  border: 1px solid rgba(142, 245, 198, 0.28);
  background: rgba(12, 17, 16, 0.82);
  color: var(--sand);
  cursor: pointer;
}

.headline {
  margin: 48px 0 34px;
  display: flex;
  justify-content: space-between;
  gap: 20px;
  align-items: end;
}

.headline h1 {
  max-width: 800px;
  margin: 10px 0 0;
  font-family: "Newsreader", serif;
  font-size: clamp(2.8rem, 7vw, 5.5rem);
  line-height: 0.94;
  font-weight: 700;
}

.kicker {
  color: var(--mint);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 0.82rem;
}

.route-pill {
  align-self: start;
  padding: 10px 14px;
  border-radius: 999px;
  border: 1px solid rgba(245, 179, 107, 0.24);
  color: var(--amber);
}

.wallet-error {
  margin: -8px 0 20px;
  color: var(--danger);
}

@media (max-width: 960px) {
  .topbar,
  .headline {
    grid-template-columns: 1fr;
    display: grid;
  }

  .nav {
    justify-self: start;
    flex-wrap: wrap;
  }
}
</style>

