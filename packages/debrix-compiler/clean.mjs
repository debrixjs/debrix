// @ts-check

import { rm } from 'node:fs/promises';

await Promise.all([
	rm('./index.js', { force: true }),
	rm('./index.mjs', { force: true }),
	rm('./index.d.ts', { force: true }),
	rm('./lib/', { recursive: true, force: true }),
	rm('./node/', { recursive: true, force: true }),
	rm('./wasm/', { recursive: true, force: true }),
]);
