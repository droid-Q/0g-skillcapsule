export type CapsuleStatus = "draft" | "published" | "paused";
export type RunStatus = "pending" | "running" | "completed" | "failed";

export interface CapsuleManifest {
  title: string;
  tagline: string;
  systemPrompt: string;
  knowledgeRoots: string[];
  resultSchema: string;
  creatorWallet: string;
  agentTokenId?: string | null;
  coverAssetRoot?: string | null;
}

export interface Capsule {
  id: string;
  slug: string;
  theme: string;
  version: number;
  status: CapsuleStatus;
  manifestRoot: string;
  manifest: CapsuleManifest;
  registryContract?: string | null;
  publishTxHash?: string | null;
  explorerUrl?: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface ProofRecord {
  manifestRoot: string;
  knowledgeRoots: string[];
  coverAssetRoot?: string | null;
  registryContract?: string | null;
  publishTxHash?: string | null;
  explorerUrl?: string | null;
  resultStorageRoot?: string | null;
  computeMode: string;
  computeModel: string;
  proofHash: string;
}

export interface CapsuleRun {
  id: string;
  capsuleId: string;
  capsuleTitle: string;
  status: RunStatus;
  taskInput: string;
  outputJson?: Record<string, unknown> | null;
  outputText?: string | null;
  warnings: string[];
  proof: ProofRecord;
  createdAt: string;
  updatedAt: string;
}

export interface CreateCapsulePayload {
  theme: string;
  manifestRoot: string;
  manifest: CapsuleManifest;
}

export interface PublishCapsulePayload {
  registryContract: string;
  txHash: string;
  explorerUrl: string;
  agentTokenId?: string | null;
  version?: number;
}

export interface CreateRunPayload {
  taskInput: string;
}

export interface UploadAssetResult {
  rootHash: string;
  txHash?: string | null;
  mode: "0g" | "demo";
  label: string;
}

