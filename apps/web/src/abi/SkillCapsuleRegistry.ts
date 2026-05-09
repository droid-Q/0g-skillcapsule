export const skillCapsuleRegistryAbi = [
  {
    type: "function",
    name: "upsertCapsule",
    stateMutability: "nonpayable",
    inputs: [
      { name: "capsuleKey", type: "bytes32" },
      { name: "manifestRoot", type: "string" },
      { name: "agentTokenId", type: "uint256" },
      { name: "version", type: "uint64" },
      { name: "status", type: "uint8" }
    ],
    outputs: []
  }
] as const;

