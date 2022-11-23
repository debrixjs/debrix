export type Task = (...args: any) => any;

export interface Scheduler {
	flush_(this: void): void
	enqueue_(this: void, task: Task): void
}

export type Ticker = (callback: () => void) => void;

export function createScheduler(tick: Ticker): Scheduler {
	const queue = new Set<Task>();
	let flushing = false;

	const flush_ = () => {
		for (const task of Array.from(queue).slice()) {
			task();
			queue.delete(task);
		}
	};

	return {
		tick,
		flush_,

		enqueue_: (task: Task) => {
			queue.add(task);

			if (!flushing) {
				flushing = true;
				tick(() => {
					flushing = false;
					flush_();
				});
			}
		}
	} as Scheduler;
}

export function createFrameTicker(): (callback: () => void) => void {
	return (cb) => requestAnimationFrame(cb);
}

export function createMicroTicker(): (callback: () => void) => void {
	const promise = Promise.resolve();
	return (cb) => void promise.then(cb);
}
