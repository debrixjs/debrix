// @ts-check

import esbuild from 'esbuild';
import { exec as _exec } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import path from 'node:path';

const require = createRequire(import.meta.url);

/**
 * @param {string} command 
 * @param {import('node:child_process').ExecOptions} [options]
 * @returns {Promise<{ stdout: any, stderr: any }>}
 */
function exec(command, options) {
	return new Promise((resolve, reject) => {
		const proc = _exec(command, options, (err, stdout, stderr) => {
			if (err)
				reject(err);
			else
				resolve({ stdout, stderr });
		});

		proc.stdout?.pipe(process.stdout);
		proc.stderr?.pipe(process.stderr);
	});
}

/**
 * @param {unknown} value 
 * @returns {value is string}
 */
function isString(value) {
	return typeof value === 'string';
}

const minify = process.argv.includes('--minify');

if (minify)
	console.log('NOTE! Production build should NOT be minified!');

await Promise.all([
	() => exec([
		'node',
		require.resolve('typescript/lib/tsc.js'),
		'--declaration',
		'--emitDeclarationOnly',
		'--outDir types'
	].filter(isString).join(' ')),

	() => esbuild.build({
		entryPoints: ['./src/index.ts'],
		outfile: './index.js',
		format: 'cjs',
		platform: 'browser',
		target: 'es2015',
		bundle: true,
		minify
	}),

	() => esbuild.build({
		entryPoints: ['./src/index.ts'],
		outfile: './index.mjs',
		format: 'esm',
		platform: 'browser',
		target: 'es2015',
		bundle: true,
		minify
	}),

	() => esbuild.build({
		entryPoints: ['./src/binders.ts'],
		outfile: './binders/index.js',
		format: 'cjs',
		platform: 'browser',
		target: 'es2015',
		bundle: true,
		minify
	}),

	() => esbuild.build({
		entryPoints: ['./src/binders.ts'],
		outfile: './binders/index.mjs',
		format: 'esm',
		platform: 'browser',
		target: 'es2015',
		bundle: true
	}),

	() => writeFile('./index.d.ts', 'export * from \'./types\';\n'),

	async () => {
		await mkdir('./binders/', { recursive: true });
		await writeFile('./binders/index.d.ts', 'export * from \'../types/binders\';\n');
	},
].map(fn => fn()));
