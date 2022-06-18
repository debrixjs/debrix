import typescript from "rollup-plugin-typescript2";

const { RELEASE } = process.env;

export default {
	input: "src/index.ts",
	output: [
		{
			name: "debrix_internal",
			file: "index.js",
			format: "umd"
		},
		{
			file: "index.mjs",
			format: "esm"
		}
	],
	plugins: [
		typescript()
	]
};
