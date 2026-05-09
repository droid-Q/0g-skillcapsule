# 0G SkillCapsule

0G SkillCapsule is a Vue + Rust hackathon project for the `0G APAC Hackathon` Track 3. It presents AI agent capabilities as reusable, publishable capsules with:

- `0G Compute Router` for OpenAI-compatible inference
- `0G Storage` for browser-side knowledge and result pinning
- a minimal onchain `SkillCapsuleRegistry` contract for explorer-visible publish proof

The product is designed for demo quality first: polished marketplace UI, creator workflow, proof drawer, seeded capsules, and graceful local fallbacks when 0G credentials are not configured.

## Monorepo layout

- `apps/web`: Vue 3 + Vite frontend
- `apps/server`: Rust + Axum API and run orchestrator
- `contracts`: Solidity registry contract plus compile/test/deploy scripts
- `docs`: architecture notes and demo script

## Local run

```bash
cd /Users/droid/RustroverProjects/0g-skillcapsule
cp .env.example .env
npm install --no-package-lock --prefix apps/web
npm install --no-package-lock --prefix contracts
cargo run --manifest-path apps/server/Cargo.toml
npm --prefix apps/web run dev
```

Open the Vite URL, usually `http://127.0.0.1:5179`.

## Environment notes

- Without `ZERO_G_ROUTER_API_KEY`, the Rust server returns local demo outputs so the run flow stays usable.
- With `VITE_0G_STORAGE_MODE=demo`, the frontend generates deterministic demo roots instead of sending wallet-signed uploads.
- To send a real publish transaction, set `VITE_SKILLCAPSULE_REGISTRY_ADDRESS` after deploying the contract.
- To deploy the registry:

```bash
cd contracts
SKILLCAPSULE_DEPLOY_RPC_URL=... \
SKILLCAPSULE_PRIVATE_KEY=... \
npm run deploy
```

## API surface

- `GET /api/capsules`
- `POST /api/capsules`
- `GET /api/capsules/:id`
- `POST /api/capsules/:id/publish`
- `POST /api/capsules/:id/run`
- `GET /api/runs/:id`
