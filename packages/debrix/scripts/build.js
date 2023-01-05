// @ts-check

import esbuild from 'esbuild';
import { declarations, parallel } from '../../../utils/build';
import { mkdir, writeFile } from 'node:fs/promises';

const minify = process.argv.includes('--minify');
if (minify) console.log('NOTE! Production build should NOT be minified!');

/** @type {Partial<esbuild.BuildOptions>} */
const shared = {
	platform: 'browser',
	target: 'es2015',
	bundle: true,
	minify,
	// Everything ending in '_', but not starting with '$'
	mangleProps: /^[^$].*_$/,
};

parallel(
	() => declarations(),

	() =>
		esbuild.build({
			...shared,
			entryPoints: ['./src/index.ts'],
			outfile: './index.js',
			format: 'cjs',
		}),

	() =>
		esbuild.build({
			...shared,
			entryPoints: ['./src/index.ts'],
			outfile: './index.mjs',
			format: 'esm',
		}),

	() =>
		esbuild.build({
			...shared,
			entryPoints: ['./src/binders.ts'],
			outfile: './binders/index.js',
			format: 'cjs',
		}),

	() =>
		esbuild.build({
			entryPoints: ['./src/binders.ts'],
			outfile: './binders/index.mjs',
			format: 'esm',
		}),

	() => writeFile('./index.d.ts', "export * from './types';\n"),

	async () => {
		await mkdir('./binders/', { recursive: true });
		await writeFile(
			'./binders/index.d.ts',
			"export * from '../types/binders';\n"
		);
	}
);
