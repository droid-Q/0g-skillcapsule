import { toCapsuleKey } from "./registry";

describe("toCapsuleKey", () => {
  it("returns a deterministic bytes32 hash", () => {
    expect(toCapsuleKey("capsule-123")).toBe(
      "0x3c6268a7345f6a56b8fb22772db60c0b51396b06a40f8e60eb2c36fa65f32039"
    );
  });
});
