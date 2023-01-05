// @ts-check

import 'esbuild-register/loader';
import { declarations, parallel } from '../../../utils/build';
import { build } from 'esbuild';
import { writeFile } from 'node:fs/promises';

/** @type {import('esbuild').BuildOptions} */
const shared = {
	entryPoints: ['./src/index.ts'],
	platform: 'node',
	bundle: true,
};

parallel(
	() => declarations(),

	() =>
		build({
			...shared,
			outfile: './index.js',
			format: 'cjs',
		}),

	() =>
		build({
			...shared,
			outfile: './index.mjs',
			format: 'esm',
		}),

	() =>
		writeFile(
			'./index.d.ts',
			"export * from './types';\nexport { default } from './types';\n"
		)
);
