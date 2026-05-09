import { demoRootFromText } from "./storage";

describe("demoRootFromText", () => {
  it("creates a deterministic demo root", async () => {
    await expect(demoRootFromText("skillcapsule")).resolves.toBe(
      "zg://demo/3ea184238c781972cc6cfb1038fbba57edd0aed1645a673dd9e4db2ef7f229f8"
    );
  });
});
