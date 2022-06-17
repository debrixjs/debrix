import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";

const dev = process.env.ROLLUP_WATCH === "true";

export default {
	input: "src/index.ts",
	output: [{
		file: "index.js",
		format: "cjs",
		sourcemap: dev,
		exports: "named"
	}, {
		file: "index.mjs",
		format: "esm",
		sourcemap: dev
	}],
	plugins: [
		resolve({ browser: true }),
		typescript()
	]
};
