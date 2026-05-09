import type { UploadAssetResult } from "./types";
import { getMetaMaskProvider } from "./wallet";

const STORAGE_MODE = import.meta.env.VITE_0G_STORAGE_MODE ?? "demo";
const RPC_URL = import.meta.env.VITE_0G_STORAGE_RPC_URL ?? "https://evmrpc-testnet.0g.ai";
const INDEXER_RPC =
  import.meta.env.VITE_0G_STORAGE_INDEXER_RPC ?? "https://indexer-storage-testnet-turbo.0g.ai";

async function subtleCrypto(): Promise<Crypto> {
  if (typeof globalThis.crypto !== "undefined") {
    return globalThis.crypto;
  }
  const { webcrypto } = await import("node:crypto");
  return webcrypto as Crypto;
}

async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const cryptoApi = await subtleCrypto();
  const normalized = new Uint8Array(bytes.byteLength);
  normalized.set(bytes);
  const digest = await cryptoApi.subtle.digest("SHA-256", normalized);
  return Array.from(new Uint8Array(digest))
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");
}

export async function demoRootFromText(input: string): Promise<string> {
  const hex = await sha256Hex(new TextEncoder().encode(input));
  return `zg://demo/${hex}`;
}

async function fallbackUpload(label: string, bytes: Uint8Array): Promise<UploadAssetResult> {
  const rootHash = `zg://demo/${await sha256Hex(bytes)}`;
  return {
    rootHash,
    txHash: null,
    mode: "demo",
    label
  };
}

export async function uploadFileAsset(file: File): Promise<UploadAssetResult> {
  const bytes = new Uint8Array(await file.arrayBuffer());
  if (STORAGE_MODE === "demo" || !getMetaMaskProvider()) {
    return fallbackUpload(file.name, bytes);
  }

  try {
    const [{ Blob: ZgBlob, Indexer }, ethers] = await Promise.all([
      import("@0gfoundation/0g-storage-ts-sdk/browser"),
      import("ethers")
    ]);
    const provider = new ethers.BrowserProvider(window.ethereum!);
    await provider.send("eth_requestAccounts", []);
    const signer = await provider.getSigner();
    const indexer = new Indexer(INDEXER_RPC);
    const zgBlob = new ZgBlob(file);
    const [, treeErr] = await zgBlob.merkleTree();
    if (treeErr) {
      throw new Error(String(treeErr));
    }
    const [tx, uploadErr] = await indexer.upload(zgBlob, RPC_URL, signer);
    if (uploadErr) {
      throw new Error(String(uploadErr));
    }
    const rootHash = "rootHash" in tx ? tx.rootHash : tx.rootHashes[0];
    const txHash = "txHash" in tx ? tx.txHash : tx.txHashes[0];
    return {
      rootHash,
      txHash,
      mode: "0g",
      label: file.name
    };
  } catch (error) {
    console.warn("0G upload failed, falling back to demo mode.", error);
    return fallbackUpload(file.name, bytes);
  }
}

export async function uploadJsonAsset(name: string, payload: unknown): Promise<UploadAssetResult> {
  const content = JSON.stringify(payload, null, 2);
  if (typeof File !== "undefined") {
    const file = new File([content], name, { type: "application/json" });
    return uploadFileAsset(file);
  }
  return fallbackUpload(name, new TextEncoder().encode(content));
}
