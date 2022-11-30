import { Build, InternalBuildObject, InternalErrorObject, Target, _createError, _validate } from './common';
import { Worker } from 'node:worker_threads';

declare const WORKER_URL: string;

let _nextHandle = 0;
function nextHandle() {
	return _nextHandle++;
}

const worker = new Worker(new URL(WORKER_URL, import.meta.url));
worker.unref();

export function build(input: string, target: Target = Target.Client): Promise<Build> {
	return new Promise<Build>((resolve, reject) => {
		_validate(input, target);

		const handle = nextHandle();
		const listener = ([_handle, build, error]: [number, InternalBuildObject | undefined, InternalErrorObject | undefined]) => {
			if (_handle !== handle)
				return;

			worker.removeListener('message', listener);

			if (error !== undefined) {
				reject(_createError(error));
			} else {
				resolve(build!);
			}
		};

		worker.on('message', listener);
		worker.postMessage([handle, input, target]);
	});
}

export { type Build, CompilerError, type Error, type Mapping, ParserError, Target } from './common';
