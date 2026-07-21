import { defineConfig } from "orval"

export default defineConfig({
  api: {
    input: "../openapi.json",

    output: {
      target: "./src/api/generated.ts",
      schemas: "./src/api/model",
      client: "axios",
    },
  },
})
