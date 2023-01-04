import {
	Model,
	ModelOptions,
	Subscription,
	SubscriptionListener,
} from './model';
import { createFrameTicker } from './scheduler';

export interface Computed<T = unknown> {
	get(): T;
	dispose(): void;
	observe(listener: SubscriptionListener<T>): Subscription;
}

export abstract class ViewModel extends Model {
	constructor(options: ModelOptions = {}) {
		options.ticker ??= createFrameTicker();
		super(options);
	}

	$computed<T>(get: () => T): Computed<T> {
		const listeners = new Set<SubscriptionListener<T>>();
		let value: T;
		let revoke: (() => void) | undefined;
		let dirty = true;

		return {
			get: () => {
				if (dirty) {
					revoke?.();
					dirty = false;

					let next!: T;
					const observe = this.$magic(() => (next = get()));

					if (next !== value) value = next;

					revoke = observe(() => {
						dirty = true;

						for (const listener of listeners) listener();
					});
				}

				return value;
			},

			observe: (listener) => {
				listeners.add(listener);
				return {
					revoke() {
						listeners.delete(listener);
					},
				};
			},

			dispose: () => {
				revoke?.();
			},
		};
	}
}
