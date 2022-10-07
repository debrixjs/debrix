// @ts-check

import { rm } from 'node:fs/promises';

await Promise.all([
	rm('./index.js', { force: true }),
	rm('./index.mjs', { force: true }),
	rm('./index.d.ts', { force: true }),
	rm('./binders/', { recursive: true, force: true }),
	rm('./types/', { recursive: true, force: true }),
]);
