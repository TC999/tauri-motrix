import { defineConfig } from "@rslib/core";

export default defineConfig({
  lib: [
    {
      format: "esm",
      syntax: ["node 18"],
      dts: true,
    },
  ],
  source: {
    entry: {
      index: "./src/index.ts",
      mocks: "./src/mocks.ts",
    },
  },
});
