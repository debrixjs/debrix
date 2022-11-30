// @ts-check

import path from 'node:path';
import os from 'node:os';
import { exec as _exec } from 'node:child_process';

import esbuild from 'esbuild';
import { NodeResolvePlugin } from '@esbuild-plugins/node-resolve';
import { createRequire } from 'node:module';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';

const require = createRequire(import.meta.url);
const rootdir = path.resolve();

/** @returns {esbuild.Plugin} */
function external() {
	return {
		name: 'external',
		setup(build) {
			build.onResolve({ filter: /\?external$/ }, (args) => {
				return {
					path: args.path.slice(0, -9),
					external: true
				};
			});
		}
	};
}

/**
 * @returns {import('esbuild').Plugin}
 */
function nodeResolve(options) {
	return NodeResolvePlugin({
		extensions: ['.js', '.ts'],
		onResolved: (id) => {
			if (id.startsWith('node:'))
				return { external: true };
		},
		...options,
	});
}

/**
 * @param {unknown} value 
 * @returns {value is string}
 */
function isString(value) {
	return typeof value === 'string';
}

/**
 * @param {Record<string, string | Uint8Array>} files
 * @returns {import('esbuild').Plugin}
 */
function virtual(files) {
	return {
		name: 'virtual',
		setup(build) {
			const aliases = Object.keys(files);
			const filter = new RegExp(`^(${aliases.map(x => escapeRegExp(x)).join('|')})$`);

			build.onResolve({ filter }, args => {
				return {
					namespace: 'virtual',
					path: args.path
				};
			});

			build.onLoad({ filter, namespace: 'virtual' }, args => {
				return {
					contents: files[args.path],
				};
			});
		}
	};
}

function extToLoader(ext) {
	switch (ext) {
		case '.js':
			return 'js';

		case '.ts':
			return 'ts';

		default:
			return 'default';
	}
}

/**
 * @param {{ replace: [RegExp | string, string][], filter: RegExp }} config
 * @returns {import('esbuild').Plugin}
 */
function replace({ replace, filter }) {
	return {
		name: 'replace',
		setup(build) {
			build.onLoad({ filter }, async args => {
				let contents = await readFile(args.path, 'utf8');

				for (const [searchValue, replaceValue] of replace)
					contents = contents.replace(searchValue, replaceValue);

				return {
					contents,
					loader: extToLoader(path.extname(args.path))
				};
			});
		}
	};
}

/**
 * @param {string} string 
 * @returns {string}
 */
function escapeRegExp(string) {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

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

await rm(path.resolve(rootdir, 'wasm'), { recursive: true, force: true });
await mkdir(path.resolve(rootdir, 'wasm'), { recursive: true });

await Promise.all([
	() => exec([
		'node',
		require.resolve('typescript/lib/tsc.js'),
		'--noEmit false',
		'--declaration',
		'--emitDeclarationOnly',
		'--outDir types'
	].filter(isString).join(' ')),
	async () => {
		await exec([
			'node',
			require.resolve('cargo-cp-artifact/bin/cargo-cp-artifact.js'),
			'-a', 'cdylib',
			'dist_node',
			path.resolve(rootdir, 'lib/debrix.node'),
			'--',
			'cargo',
			'build',
			'--package', 'dist_node',
			'--quiet',
			'--release',
			'--message-format=json-render-diagnostics'
		].filter(isString).join(' '));

		const nativeModule = path.posix.relative(
			path.posix.resolve('node/'),
			path.posix.resolve('lib/debrix.node')
		);

		await Promise.all([
			esbuild.build(
				{
					bundle: true,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node_worker.ts')],
					format: 'cjs',
					outfile: path.resolve(rootdir, 'node/worker.js'),
					define: {
						NATIVE_MODULE_PATH: JSON.stringify(nativeModule + '?external')
					},
					plugins: [
						replace({
							replace: [
								[/\/\*\s*#ESM\s*\*\/[^]+?\/\*\s*\/ESM\s*\*\//g, '']
							],
							filter: /\.[tj]s$/
						}),
						external(),
						nodeResolve(),
					],
				}
			),

			esbuild.build(
				{
					bundle: true,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node_worker.ts')],
					format: 'esm',
					outfile: path.resolve(rootdir, 'node/worker.mjs'),
					define: {
						NATIVE_MODULE_PATH: JSON.stringify(nativeModule)
					},
					plugins: [
						external(),
						nodeResolve(),
					],
				}
			),

			esbuild.build(
				{
					bundle: true,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node.ts')],
					format: 'cjs',
					outfile: path.resolve(rootdir, 'node/index.js'),
					define: {
						WORKER_URL: '"./worker.mjs"'
					},
					plugins: [
						nodeResolve(),
					],
				}
			),

			esbuild.build(
				{
					bundle: true,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node.ts')],
					format: 'esm',
					outfile: path.resolve(rootdir, 'node/index.mjs'),
					define: {
						WORKER_URL: '"./worker.mjs"'
					},
					plugins: [
						nodeResolve()
					],
				}
			),
		]);

		await writeFile(path.resolve(rootdir, 'index.js'), 'module.exports = require(\'./node\');\n');
		await writeFile(path.resolve(rootdir, 'index.mjs'), 'export * from \'./node/index.mjs\';\n');
		await writeFile(path.resolve(rootdir, 'index.d.ts'), 'export * from \'./types/node\';\n');
		await writeFile(path.resolve(rootdir, 'node/index.d.ts'), 'export * from \'../types/node\';\n');
	},
	async () => {
		const dir = await mkdtemp(path.join(os.tmpdir(), 'debrix-'));

		await exec([
			'wasm-pack',
			'--quiet',
			'build',
			'-d', dir.toString(),
			'-t', 'web',
			'--no-typescript',
			'--release',
			'./crates/wasm'
		].filter(isString).join(' '));

		const wasm = await readFile(path.resolve(dir, 'dist_wasm_bg.wasm'));
		const wasmLibMod = await readFile(path.resolve(dir, 'dist_wasm.js'), 'utf8');
		const wasmMod = `function __decode(value) {
	value = atob(value);
	const bytes = new Uint8Array(value.length);
	for (let i = 0; i < value.length; ++i)
		bytes[i] = value.charCodeAt(i);
	return bytes;
}

const __WASM_DECODED = \`${wasm.toString('base64')}\`;
module.exports = __decode(__WASM_DECODED);
`;

		await rm(dir, { recursive: true, force: true });

		const workerTextCJS = (
			await esbuild.build(
				{
					bundle: true,
					entryPoints: [path.resolve(rootdir, 'src/wasm_worker.ts')],
					format: 'cjs',
					write: false,
					plugins: [
						virtual({
							'debrix.wasm.lib': wasmLibMod,
							'debrix.wasm': wasmMod,
						}),
						nodeResolve()
					]
				}
			)
		).outputFiles[0].text;

		await Promise.all([
			await esbuild.build(
				{
					bundle: true,
					entryPoints: [path.resolve(rootdir, 'src/wasm.ts')],
					format: 'cjs',
					outfile: path.resolve(rootdir, 'wasm/index.js'),
					define: {
						__WORKER_TEMPLATE: JSON.stringify(workerTextCJS),
						__IS_ESM: 'false'
					},
					plugins: [
						replace({
							replace: [
								[/\/\*\s*#ESM\s*\*\/[^]+?\/\*\s*\/ESM\s*\*\//g, '']
							],
							filter: /\.[tj]s$/
						}),
						nodeResolve()
					],
				}
			),

			await esbuild.build(
				{
					bundle: true,
					external: ['dist/lib/debrix.node'],
					entryPoints: [path.resolve(rootdir, 'src/wasm.ts')],
					format: 'esm',
					outfile: path.resolve(rootdir, 'wasm/index.mjs'),
					define: {
						__WORKER_TEMPLATE: JSON.stringify(workerTextCJS),
						__IS_ESM: 'true'
					},
					plugins: [
						replace({
							replace: [
								[/\/\*\s*#CJS\s*\*\/[^]+?\/\*\s*\/CJS\s*\*\//g, '']
							],
							filter: /\.[tj]s$/
						}),
						nodeResolve()
					],
				}
			),
		]);

		await writeFile(path.resolve(rootdir, 'wasm/index.d.ts'), 'export * from \'../types/wasm\';\n');
	}
].map(fn => fn()));
