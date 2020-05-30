import typescript from "rollup-plugin-typescript2";
import alias from "@rollup/plugin-alias";
import run from "@rollup/plugin-run";
import { resolve } from "path";
import { builtinModules } from "module";

const entries = [{ find: "src", replacement: resolve(__dirname, "src") }];
const dev = process.env.ROLLUP_WATCH === "true";

/** @type {import("rollup").RollupOptions} */
const config = {
  input: "src/main.ts",
  output: {
    file: "dist/index.js",
    format: "cjs",
  },
  plugins: [alias({ entries }), typescript(), dev && run()],
  external: [
    "@nestjs/core",
    "@nestjs/common",
    "@nestjs/graphql",
    "@google-cloud/datastore",
    "class-validator",
    "helmet",
    "cuid",
    ...builtinModules,
  ],
};

export default config;
