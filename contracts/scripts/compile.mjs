import fs from "node:fs/promises";
import path from "node:path";
import solc from "solc";

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
    optimizer: {
      enabled: true,
      runs: 200
    },
    outputSelection: {
      "*": {
        "*": ["abi", "evm.bytecode"]
      }
    }
  }
};

const output = JSON.parse(solc.compile(JSON.stringify(input)));
if (output.errors?.some((entry) => entry.severity === "error")) {
  for (const error of output.errors) {
    console.error(error.formattedMessage);
  }
  process.exit(1);
}

const artifact = output.contracts["SkillCapsuleRegistry.sol"].SkillCapsuleRegistry;
const artifactDir = path.join(root, "artifacts");
await fs.mkdir(artifactDir, { recursive: true });
await fs.writeFile(
  path.join(artifactDir, "SkillCapsuleRegistry.json"),
  JSON.stringify(
    {
      abi: artifact.abi,
      bytecode: artifact.evm.bytecode.object
    },
    null,
    2
  )
);

console.log("Compiled SkillCapsuleRegistry -> artifacts/SkillCapsuleRegistry.json");
