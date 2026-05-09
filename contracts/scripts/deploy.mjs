import fs from "node:fs/promises";
import path from "node:path";
import { ethers } from "ethers";
import solc from "solc";

const rpcUrl = process.env.SKILLCAPSULE_DEPLOY_RPC_URL;
const privateKey = process.env.SKILLCAPSULE_PRIVATE_KEY;

if (!rpcUrl || !privateKey) {
  console.error("Set SKILLCAPSULE_DEPLOY_RPC_URL and SKILLCAPSULE_PRIVATE_KEY before deploying.");
  process.exit(1);
}

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
const artifact = output.contracts["SkillCapsuleRegistry.sol"].SkillCapsuleRegistry;

const provider = new ethers.JsonRpcProvider(rpcUrl);
const wallet = new ethers.Wallet(privateKey, provider);
const factory = new ethers.ContractFactory(artifact.abi, artifact.evm.bytecode.object, wallet);
const contract = await factory.deploy();
await contract.waitForDeployment();

console.log(JSON.stringify({
  address: await contract.getAddress(),
  deployer: wallet.address,
  network: (await provider.getNetwork()).chainId.toString()
}, null, 2));
