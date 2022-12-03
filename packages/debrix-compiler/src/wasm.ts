import { Build, InternalBuildObject, InternalErrorObject, Target, _createError, _validate } from './common';

function createWorkerURL(workerText: string) {
	return URL.createObjectURL(
		new Blob([workerText], { type: 'text/javascript' })
	);
}

let _nextHandle = 0;
function nextHandle() {
	return _nextHandle++;
}

declare const __WORKER_TEMPLATE: string;
const WORKER_TEMPLATE = __WORKER_TEMPLATE;

async function importWorkerThreads() {
	let worker_threads: typeof import('node:worker_threads');
	/* #ESM */worker_threads = await import('node:worker_threads');/* /ESM */
	// eslint-disable-next-line
	/* #CJS */worker_threads = require('node:worker_threads');/* /CJS */
	return worker_threads;
}

function isNodeJs() {
	return typeof process !== 'undefined';
}

let _service: ((input: string, target: Target) => Promise<Build>) | undefined;

async function ensureService(): Promise<(input: string, target: Target) => Promise<Build>> {
	if (_service !== undefined)
		return _service;

	if (isNodeJs()) {
		let workerText = WORKER_TEMPLATE;

		// if (isESM()) {
		// 	workerText = 'import worker_threads from "node:worker_threads";\n' + workerText;
		// } else {
		workerText = 'const worker_threads = require("node:worker_threads");\npostMessage = (data) => worker_threads.parentPort.postMessage(data);\n\n' + workerText;
		// }

		// parentPort

		workerText += '\nworker_threads.parentPort.on("message", LISTENER);\n';

		const worker_threads = await importWorkerThreads();
		const worker = new worker_threads.Worker(workerText, { eval: true });
		worker.unref();

		return _service = (input, target) => new Promise((resolve, reject) => {
			const handle = nextHandle();
			const listener = ([_handle, build, error]: [number, InternalBuildObject | undefined, InternalErrorObject | undefined]) => {
				if (_handle !== handle)
					return;

				worker.removeListener('message', listener);

				if (error !== undefined) {
					reject(error);
				} else {
					resolve(build!);
				}
			};

			worker.on('message', listener);
			worker.postMessage([handle, input, target]);
		});
	} else {
		const workerText = WORKER_TEMPLATE + `
onmessage = (event) => {
	LISTENER(event.data);
};
`;

		const worker = new Worker(createWorkerURL(workerText));
		const listeners = new Set<(ev: MessageEvent) => void>();

		worker.onmessage = (ev) => {
			for (const listener of listeners)
				listener(ev);
		};

		return _service = (input, target) => new Promise((resolve, reject) => {
			const handle = nextHandle();
			const listener = (event: MessageEvent) => {
				// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
				const [_handle, build, error] = event.data as [number, InternalBuildObject | undefined, InternalErrorObject | undefined];

				if (_handle !== handle)
					return;

				if (error !== undefined) {
					reject(_createError(error));
				} else {
					resolve(build!);
				}
				listeners.delete(listener);
			};

			listeners.add(listener);
			worker.postMessage([handle, input, target]);
		});
	}
}

/** Will initialize worker. Not required to be called or finished before calling build. */
export async function initialize(): Promise<void> {
	await ensureService();
}

export async function build(input: string, target = Target.Client): Promise<Build> {
	_validate(input, target);

	const service = await ensureService();
	return service(input, target);
}

export {
	type Build,
	CompilerError,
	type Error,
	type Mapping,
	ParserError,
	Target
} from './common';
