export type Task = (...args: any) => any;

export default class Scheduler {
	protected queue = new Set<Task>();
	protected microtask = Promise.resolve();
	protected flushing = false;

	protected next(cb: () => void) {
		void this.microtask.then(cb);
	}

	flush() {
		for (const task of this.queue) {
			task();
			this.queue.delete(task);
		}
	}

	enqueue(task: Task) {
		this.queue.add(task);

		if (!this.flushing) {
			this.next(() => {
				this.flushing = false;
				this.flush();
			});
		}
	}
}
