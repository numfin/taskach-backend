import typescript from "rollup-plugin-typescript2";
import alias from "@rollup/plugin-alias";
import run from "@rollup/plugin-run";
import { resolve } from "path";
import externals from "rollup-plugin-node-externals";

const entries = [
  { find: "src", replacement: resolve(__dirname, "src") },
  { find: "lib", replacement: resolve(__dirname, "lib") },
];
const dev = process.env.ROLLUP_WATCH === "true";

/** @type {import("rollup").RollupOptions} */
const config = {
  input: "src/main.ts",
  output: {
    file: "dist/index.js",
    format: "cjs",
  },
  plugins: [
    externals({ deps: true }),
    alias({ entries }),
    typescript(),
    dev && run(),
  ],
};

export default config;
