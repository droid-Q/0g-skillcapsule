import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import ganache from "ganache";
import solc from "solc";
import { ethers } from "ethers";

async function compileContract() {
  const root = path.resolve(import.meta.dirname, "..");
  const sourcePath = path.join(root, "contracts", "SkillCapsuleRegistry.sol");
  const source = await fs.readFile(sourcePath, "utf8");
  const input = {
    language: "Solidity",
    sources: {
      "SkillCapsuleRegistry.sol": {
        content: source
      }
    },
    settings: {
      evmVersion: "paris",
      outputSelection: {
        "*": {
          "*": ["abi", "evm.bytecode"]
        }
      }
    }
  };
  const output = JSON.parse(solc.compile(JSON.stringify(input)));
  return output.contracts["SkillCapsuleRegistry.sol"].SkillCapsuleRegistry;
}

async function deploy() {
  const artifact = await compileContract();
  const provider = ganache.provider({ logging: { quiet: true } });
  const browserProvider = new ethers.BrowserProvider(provider);
  const signer = await browserProvider.getSigner(0);
  const other = await browserProvider.getSigner(1);
  const factory = new ethers.ContractFactory(artifact.abi, artifact.evm.bytecode.object, signer);
  const contract = await factory.deploy();
  await contract.waitForDeployment();
  return { contract, signer, other };
}

test("upserts a capsule and exposes it via getCapsule", async () => {
  const { contract, signer } = await deploy();
  const capsuleKey = ethers.keccak256(ethers.toUtf8Bytes("capsule-a"));
  await (await contract.upsertCapsule(capsuleKey, "zg://manifest", 12n, 1, 1)).wait();
  const record = await contract.getCapsule(capsuleKey);
  assert.equal(record.owner, await signer.getAddress());
  assert.equal(record.manifestRoot, "zg://manifest");
  assert.equal(record.agentTokenId, 12n);
  assert.equal(record.version, 1n);
  assert.equal(record.status, 1n);
});

test("blocks non-owner updates after first publish", async () => {
  const { contract, other } = await deploy();
  const capsuleKey = ethers.keccak256(ethers.toUtf8Bytes("capsule-b"));
  await (await contract.upsertCapsule(capsuleKey, "zg://manifest", 0n, 1, 1)).wait();
  await assert.rejects(contract.connect(other).upsertCapsule(capsuleKey, "zg://manifest-v2", 1n, 2, 1));
});

test("allows the owner to pause a capsule", async () => {
  const { contract } = await deploy();
  const capsuleKey = ethers.keccak256(ethers.toUtf8Bytes("capsule-c"));
  await (await contract.upsertCapsule(capsuleKey, "zg://manifest", 0n, 1, 1)).wait();
  await (await contract.setCapsuleStatus(capsuleKey, 2)).wait();
  const record = await contract.getCapsule(capsuleKey);
  assert.equal(record.status, 2n);
});
