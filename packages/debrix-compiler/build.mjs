// @ts-check

import path from 'node:path';
import os from 'node:os';
import { exec as _exec } from 'node:child_process';

import esbuild from 'esbuild';
import { createRequire } from 'node:module';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';

const require = createRequire(import.meta.url);

const production = process.argv.includes('--production');
const rootdir = path.resolve();

/** @type {import('esbuild').BuildOptions} */
const sharedConfig = {
	minify: production,
	bundle: true,
	external: ['dist/lib/debrix.node'],
	plugins: [
		{
			name: 'library',
			setup(build) {
				build.onResolve({ filter: /^[./]*lib/ }, args => ({
					path: args.path,
					external: true
				}));
			},
		}
	],
};

/**
 * @param {unknown} value 
 * @returns {value is string}
 */
function isString(value) {
	return typeof value === 'string';
}

/**
 * @param {Record<string, string>} files
 * @returns {import('esbuild').Plugin}
 */
function virtual(files) {
	return {
		name: 'virtual',
		setup(build) {
			const aliases = Object.keys(files);
			const filter = new RegExp(`^(${aliases.map(x => escapeRegExp(x)).join('|')})$`);

			build.onResolve({ filter }, args => ({
				namespace: 'virtual',
				path: args.path
			}));

			build.onLoad({ filter, namespace: 'virtual' }, args => ({
				contents: files[args.path]
			}));
		}
	};
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

				return { contents };
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
			production && '--release',
			'--message-format=json-render-diagnostics'
		].filter(isString).join(' '));

		await Promise.all([
			await esbuild.build(
				{
					...sharedConfig,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node.js')],
					format: 'cjs',
					outfile: path.resolve(rootdir, 'node/index.js'),
					plugins: [
						...sharedConfig.plugins || [],
						replace({
							replace: [
								[/\/\*\s*#ESM\s*\*\/[^]+?\/\*\s*\/ESM\s*\*\//g, '']
							],
							filter: /\.js$/
						})
					],
				}
			),

			await esbuild.build(
				{
					...sharedConfig,
					platform: 'node',
					entryPoints: [path.resolve(rootdir, 'src/node.js')],
					format: 'esm',
					outfile: path.resolve(rootdir, 'node/index.mjs')
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
			production && '--release',
			'./crates/wasm'
		].filter(isString).join(' '));

		const wasm = await readFile(path.resolve(dir, 'dist_wasm_bg.wasm'));
		const js = await readFile(path.resolve(dir, 'dist_wasm.js'), 'utf8');
		const bytesm = `
function __decode(value) {
	value = atob(value);
	const bytes = new Uint8Array(value.length);
	for (let i = 0; i < value.length; ++i)
		bytes[i] = value.charCodeAt(i);
	return bytes;
}

module.exports = __decode(\`${wasm.toString('base64')}\`);
`;

		await writeFile(
			path.resolve(rootdir, 'lib/debrix.wasm.js'),
			production ? (await esbuild.transform(bytesm, { minify: true })).code : bytesm
		);

		await rm(dir, { recursive: true, force: true });

		await Promise.all([
			await esbuild.build(
				{
					...sharedConfig,
					entryPoints: [path.resolve(rootdir, 'src/wasm.js')],
					format: 'cjs',
					outfile: path.resolve(rootdir, 'wasm/index.js'),
					plugins: [
						virtual({ '__debrix.wasm.js': js }),
						...sharedConfig.plugins || [],
					]
				}
			),

			await esbuild.build(
				{
					...sharedConfig,
					entryPoints: [path.resolve(rootdir, 'src/wasm.js')],
					format: 'esm',
					outfile: path.resolve(rootdir, 'wasm/index.mjs'),
					plugins: [
						virtual({ '__debrix.wasm.js': js }),
						...sharedConfig.plugins || [],
					]
				}
			),
		]);

		await writeFile(path.resolve(rootdir, 'wasm/index.d.ts'), 'export * from \'../types/wasm\';\n');
	}
].map(fn => fn()));
