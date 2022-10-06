import typescript from 'rollup-plugin-typescript2';

const { PRODUCTION } = process.env;

export default {
	input: 'src/index.ts',
	output: [
		{
			name: 'debrix_internal',
			file: 'index.js',
			format: 'umd',
			minify: PRODUCTION
		},
		{
			file: 'index.mjs',
			format: 'esm',
			minify: PRODUCTION
		}
	],
	plugins: [
		typescript()
	]
};
