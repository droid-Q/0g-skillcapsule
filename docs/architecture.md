# Architecture

## Product loop

1. Creator uploads a knowledge file in the browser.
2. Frontend pins the file to `0G Storage` or falls back to deterministic demo roots.
3. Frontend uploads the manifest bundle and submits `manifestRoot + manifest summary` to the Rust API.
4. Creator publishes the capsule to `SkillCapsuleRegistry`, then the publish proof is synced back to the catalog.
5. User runs a capsule; Rust calls the `0G Compute Router` or returns seeded demo output if no API key is configured.
6. The run result page pins structured output back to storage and shows all proof in the drawer.

## Runtime split

- `apps/web`
  - Wallet connect
  - 0G Storage browser uploads
  - Contract publishing
  - Marketplace, detail page, studio, run result page
- `apps/server`
  - Capsule catalog
  - Run orchestration
  - 0G Compute Router calls
  - SQLite persistence
- `contracts`
  - Minimal registry for `capsuleId -> manifestRoot + owner + version + status`

## Main data objects

- `Capsule`
  - Marketplace-visible entity with version, status, theme, manifest root, and publish proof
- `CapsuleManifest`
  - Title, tagline, system prompt, knowledge roots, result schema, creator wallet, optional agent token, optional cover root
- `CapsuleRun`
  - Task input, run status, structured output, warnings, proof bundle
- `ProofRecord`
  - Manifest root, knowledge roots, optional explorer link, optional result root, compute metadata, derived proof hash

