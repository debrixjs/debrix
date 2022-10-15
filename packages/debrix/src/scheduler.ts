export type Task = (...args: any) => any;

export interface Scheduler {
	flush(this: void): void
	enqueue(this: void, task: Task): void
}

export function createScheduler(): Scheduler {
	const queue = new Set<Task>();
	const microtask = Promise.resolve();
	let flushing = false;

	const next = (cb: () => void) => {
		void microtask.then(cb);
	};

	const flush = () => {
		for (const task of queue) {
			task();
			queue.delete(task);
		}
	};

	return {
		flush,

		enqueue: (task: Task) => {
			queue.add(task);

			if (!flushing) {
				next(() => {
					flushing = false;
					flush();
				});
			}
		}
	};
}
