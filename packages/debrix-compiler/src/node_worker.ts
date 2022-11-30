/* eslint-disable @typescript-eslint/ban-ts-comment */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

/* #ESM */
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
/* /ESM */

declare const NATIVE_MODULE_PATH: string;

// @ts-ignore
// eslint-disable-next-line @typescript-eslint/no-var-requires
const compiler = require(NATIVE_MODULE_PATH) as import('debrix.node');

import { parentPort } from 'node:worker_threads';
import type { InternalBuildObject, InternalErrorObject } from './common';

parentPort!.on('message', ([handle, input, target]) => {
	let build: InternalBuildObject | undefined,
		error: InternalErrorObject | undefined;

	try {
		build = compiler.build(input, target);
	} catch (err) {
		error = err;
	}
	
	parentPort!.postMessage([handle, build, error]);
});
