import { base64 } from 'rollup-plugin-base64';
import { terser } from 'rollup-plugin-terser';

const { RELEASE } = process.env;

export default {
	input: 'lib/mod.js',
	output: {
		name: 'debrixc-wasm',
		file: 'index.js',
		format: 'umd'
	},
	plugins: [
		base64({
			include: '**/*.wasm'
		}),
		RELEASE && terser({
			keep_classnames: true,
			keep_fnames: true
		})
	]
};
