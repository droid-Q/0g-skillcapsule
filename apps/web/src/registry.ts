import { BrowserProvider, Contract, keccak256, toUtf8Bytes } from "ethers";
import { skillCapsuleRegistryAbi } from "./abi/SkillCapsuleRegistry";
import { getMetaMaskProvider } from "./wallet";

export interface PublishProof {
  registryContract: string;
  txHash: string;
  explorerUrl: string;
  mode: "0g" | "demo";
}

const REGISTRY_ADDRESS = import.meta.env.VITE_SKILLCAPSULE_REGISTRY_ADDRESS ?? "";
const EXPLORER_TX_BASE =
  import.meta.env.VITE_0G_CHAIN_EXPLORER_TX_BASE ?? "https://chainscan-galileo.0g.ai/tx/";

export function toCapsuleKey(capsuleId: string): string {
  return keccak256(toUtf8Bytes(capsuleId));
}

export async function publishCapsuleOnchain(params: {
  capsuleId: string;
  manifestRoot: string;
  agentTokenId?: string | null;
  version: number;
}): Promise<PublishProof> {
  const provider = getMetaMaskProvider();
  if (!provider || !REGISTRY_ADDRESS) {
    const txHash = `0xdemo${toCapsuleKey(params.capsuleId).slice(2, 18)}`;
    return {
      registryContract: REGISTRY_ADDRESS || "0x2700F6A3e505402C9daB154C5c6ab9cAEC98EF1F",
      txHash,
      explorerUrl: `${EXPLORER_TX_BASE}${txHash}`,
      mode: "demo"
    };
  }

  const browserProvider = new BrowserProvider(provider);
  await browserProvider.send("eth_requestAccounts", []);
  const signer = await browserProvider.getSigner();
  const contract = new Contract(REGISTRY_ADDRESS, skillCapsuleRegistryAbi, signer);
  const transaction = await contract.upsertCapsule(
    toCapsuleKey(params.capsuleId),
    params.manifestRoot,
    BigInt(params.agentTokenId ?? "0"),
    params.version,
    1
  );
  const receipt = await transaction.wait();
  return {
    registryContract: REGISTRY_ADDRESS,
    txHash: receipt?.hash ?? transaction.hash,
    explorerUrl: `${EXPLORER_TX_BASE}${receipt?.hash ?? transaction.hash}`,
    mode: "0g"
  };
}

