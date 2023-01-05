// @ts-check

import path from 'node:path';
import os from 'node:os';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import {
	declarations,
	build,
	exec,
	parallel,
	replacePlugin,
	externalPlugin,
	nodeResolvePlugin,
	virtualPlugin,
	buildOptions,
	buildToBuf,
} from '../../../utils/build';

const shared = buildOptions({
	plugins: [externalPlugin(), nodeResolvePlugin()],
});

const cjsShared = buildOptions(
	{
		plugins: [
			replacePlugin({
				replace: [[/\/\*\s*#ESM\s*\*\/[^]+?\/\*\s*\/ESM\s*\*\//g, '']],
				filter: /\.[tj]s$/,
			}),
		],
	},
	shared,
	{
		format: 'cjs',
	}
);

const esmShared = buildOptions(
	{
		plugins: [
			replacePlugin({
				replace: [[/\/\*\s*#CJS\s*\*\/[^]+?\/\*\s*\/CJS\s*\*\//g, '']],
				filter: /\.[tj]s$/,
			}),
		],
	},
	shared,
	{
		format: 'esm',
	}
);

parallel(
	() => declarations(),

	// Build NODE distribution
	async () => {
		await exec([
			'node',
			require.resolve('cargo-cp-artifact/bin/cargo-cp-artifact.js'),
			'-a',
			'cdylib',
			'dist_node',
			'lib/debrix.node',
			'--',
			'cargo',
			'build',
			'--package',
			'dist_node',
			'--quiet',
			'--release',
			'--message-format=json-render-diagnostics',
		]);

		const _shared = buildOptions({
			bundle: true,
			platform: 'node',
		});

		const workerShared = buildOptions(_shared, {
			entryPoints: ['src/node_worker.ts'],
			define: {
				NATIVE_MODULE_PATH: JSON.stringify(
					path.posix.relative(
						path.posix.resolve('node/'),
						path.posix.resolve('lib/debrix.node')
					) + '?external'
				),
			},
		});

		const parentShared = buildOptions(_shared, {
			entryPoints: ['src/node.ts'],
		});

		await parallel(
			() =>
				build(
					buildOptions(cjsShared, workerShared, {
						outfile: 'node/worker.js',
					})
				),

			() =>
				build(
					buildOptions(esmShared, workerShared, {
						outfile: 'node/worker.mjs',
					})
				),

			() =>
				build(
					buildOptions(cjsShared, parentShared, {
						outfile: 'node/index.js',
						define: {
							WORKER_URL: '"./worker.mjs"',
						},
					})
				),

			() =>
				build(
					buildOptions(esmShared, parentShared, {
						outfile: 'node/index.mjs',
						define: {
							WORKER_URL: '"./worker.mjs"',
						},
					})
				)
		);
	},
	async () => {
		parallel(
			() => writeFile('index.js', "module.exports = require('./node');\n"),
			() => writeFile('index.mjs', "export * from './node/index.mjs';\n"),
			() => writeFile('index.d.ts', "export * from './types/node';\n"),
			() => writeFile('node/index.d.ts', "export * from '../types/node';\n")
		);
	},

	// Build WASM distribution
	async () => {
		await mkdir('wasm', { recursive: true });

		// Generate wasm and wasm lib modules.
		const tempdir = await mkdtemp(path.join(os.tmpdir(), 'debrix-'));
		await exec([
			'wasm-pack',
			'--quiet',
			'build',
			'-d',
			tempdir.toString(),
			'-t',
			'web',
			'--no-typescript',
			'--release',
			'./crates/wasm',
		]);
		const wasm = await readFile(path.resolve(tempdir, 'dist_wasm_bg.wasm'));
		const wasmLibFile = await readFile(
			path.resolve(tempdir, 'dist_wasm.js'),
			'utf8'
		);
		const wasmFile = `
			function __decode(value) {
				value = atob(value);
				const bytes = new Uint8Array(value.length);
				for (let i = 0; i < value.length; ++i)
					bytes[i] = value.charCodeAt(i);
				return bytes;
			}

			const __WASM_DECODED = \`${wasm.toString('base64')}\`;
			module.exports = __decode(__WASM_DECODED);
		`;
		await rm(tempdir, { recursive: true, force: true });

		// Bundle workers into plain text (buffer).
		const workerCjsFile = await buildToBuf(
			buildOptions(
				{
					plugins: [
						virtualPlugin({
							'debrix.wasm.lib': wasmLibFile,
							'debrix.wasm': wasmFile,
						}),
					],
				},
				cjsShared,
				{
					entryPoints: ['src/wasm_worker.ts'],
				}
			)
		);
		const workerEsmFile = await buildToBuf(
			buildOptions(
				{
					plugins: [
						virtualPlugin({
							'debrix.wasm.lib': wasmLibFile,
							'debrix.wasm': wasmFile,
						}),
					],
				},
				esmShared,
				{
					entryPoints: ['src/wasm_worker.ts'],
				}
			)
		);

		const textDecoder = new TextDecoder();

		await parallel(
			() =>
				build(
					buildOptions(cjsShared, {
						entryPoints: ['src/wasm.ts'],
						outfile: 'wasm/index.js',
						define: {
							__WORKER_TEMPLATE: JSON.stringify(
								textDecoder.decode(workerCjsFile)
							),
						},
					})
				),

			() =>
				build(
					buildOptions(esmShared, {
						entryPoints: ['src/wasm.ts'],
						outfile: 'wasm/index.mjs',
						define: {
							__WORKER_TEMPLATE: JSON.stringify(
								textDecoder.decode(workerEsmFile)
							),
						},
					})
				)
		);
	},
	() => writeFile('wasm/index.d.ts', "export * from '../types/wasm';\n")
);
