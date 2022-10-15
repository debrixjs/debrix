import { createEventEmitter } from './event-emitter';
import { createModifierMap } from './modifier-map';
import { createScheduler } from './scheduler';
import { type Subscription, type SubscriptionListener } from './subscription';

type Link = [target: object, key: string | symbol];

function createLink(target: object, key: string | symbol): Link {
	return [target, key];
}

function isEqualLink(x: Link, y: Link): boolean {
	return x[0] === y[0] && x[1] === y[1];
}

interface Observable {
	observe(listener: SubscriptionListener): Subscription;
}

export interface Reference<T = unknown> extends Observable {
	get(): T
	set(value: T): void
}

export interface Computed<T = unknown> extends Observable {
	get(): T
	dispose(): void
}

export interface Extender<T> {
	notify?(value: T): boolean | Promise<boolean>
	recompute?(): boolean | Promise<boolean>
	compute?(value: T): T
	init?(target: Computed<T>): void
}

const modifiers = createModifierMap<{
	/* ignore   */ i?: boolean;
	/* effect   */ ef?: boolean;
	/* throttle */ t?: number;
	/* debounce */ d?: number;
	/* readonly */ r?: boolean;
	/* extend   */ e?: Extender<unknown>[];
}>();

export function ignore(target: object, key: string | symbol) {
	modifiers.set(target, key, 'i', true);
}

export function effect(target: object, key: string | symbol) {
	modifiers.set(target, key, 'ef', true);
}

export function throttle(delay: number) {
	return (target: object, key: string | symbol) => {
		modifiers.set(target, key, 't', delay);
	};
}

export function debounce(delay: number) {
	return (target: object, key: string | symbol) => {
		modifiers.set(target, key, 'd', delay);
	};
}

export function readonly(target: object, key: string | symbol) {
	modifiers.set(target, key, 'r', true);
}

export function extend(extender: Extender<unknown>) {
	return (target: object, key: string | symbol) => {
		let extenders: Extender<unknown>[] | undefined;

		if (extenders = modifiers.get(target, key, 'e'))
			extenders.push(extender);
		else
			modifiers.set(target, key, 'e', [extender]);
	};
}

export abstract class Model {
	$schedule!: (task: () => void) => void;
	$tick!: () => void;
	$silent!: <T>(target: T) => T;
	$ref!: <T>(target: T) => Reference<T>;

	/** @internal */
	$events = createEventEmitter<{ set: [link: Link], get: [link: Link] }>();

	/** @internal */
	/* #__PURE__ */
	$magic(callback: () => void): (listener: SubscriptionListener) => () => void {
		const links: Link[] = [];
		const temp = this.$events.on('get', link => links.push(link));
		callback();
		temp.revoke();

		return (listener) => {
			const subs = links.map(link => this.$observe(link /* satisfies Link */, listener));
			return () => subs.forEach(sub => sub.revoke());
		};
	}

	$observe(target: unknown, listener: () => void): Subscription {
		return this.$events.on('set', (link) => {
			if (isEqualLink(target as Link, link))
				listener();
		});
	}

	/** Creates an computed _value_ which recomputes the next (animation) frame. */
	$computed<T>(get: () => T): Computed<T> {
		const subscriptions = new Set<SubscriptionListener>();
		let value: T;
		let dirty = true;
		let revoke: (() => void) | undefined;
		let disposed = false;

		const assertNotDisposed = () => {
			if (disposed)
				throw new Error('computed is disposed');
		};

		const recompute = () => {
			dirty = false;

			let next!: T;
			const observe = this.$magic(() => next = get());

			if (next !== value) {
				value = next;

				revoke?.();

				revoke = observe(() => {
					dirty = true;

					this.$schedule(() => {
						for (const listener of subscriptions)
							listener();
					});
				});
			}
		};

		return {
			get: () => {
				assertNotDisposed();

				if (dirty)
					recompute();

				return value;
			},

			observe: (listener) => {
				assertNotDisposed();
				subscriptions.add(listener);
				return {
					revoke() {
						subscriptions.delete(listener);
					}
				};
			},

			dispose: () => {
				assertNotDisposed();

				disposed = true;
				revoke?.();
			},
		};
	}

	constructor() {
		const scheduler = createScheduler();
		this.$schedule = scheduler.enqueue;
		this.$tick = scheduler.flush;

		const extend = <T extends object>(object: T, isRoot: boolean): T => {
			let getRef = false;
			let silent = false;

			const getters = new WeakMap<object, () => unknown>();

			const computed = (target: object, key: string | symbol, receiver: unknown, descriptior: PropertyDescriptor): unknown => {
				// The function is accually safe to reference because the key cannot be accessed in a weakmap.
				// eslint-disable-next-line @typescript-eslint/unbound-method
				if (getters.has(descriptior.get!))

					// The function is accually safe to reference because the key cannot be accessed in a weakmap.
					// eslint-disable-next-line @typescript-eslint/unbound-method
					return getters.get(descriptior.get!)!();

				let value: unknown;
				let dirty = true;
				let dispose: (() => void) | undefined;
				let debounceHandle: number | undefined;
				const subs = new Set<SubscriptionListener>();

				const extenders = modifiers.get(target, key, 'e') ?? [];

				const recompute = () => {
					dirty = false;

					let next!: unknown;
					const observe = this.$magic(() => next = Reflect.get(target, key, receiver) as unknown);

					if (next !== value) {
						value = next;

						for (const extender of extenders) {
							if (extender.compute)
								value = extender.compute(value);
						}

						dispose?.();

						// eslint-disable-next-line @typescript-eslint/no-misused-promises
						dispose = observe(async () => {
							dirty = true;

							for (const extender of extenders) {
								if (extender.recompute && !await extender.recompute()) {
									dirty = false;
									return;
								}

								if (extender.notify && !await extender.notify(value))
									return;
							}

							const notify = () => {
								this.$events.trigger('set', createLink(target, key));
								for (const listener of subs)
									listener();
							};

							let timeout: number | undefined;

							if ((timeout = modifiers.get(target, key, 't')) !== undefined) {
								setTimeout(notify, timeout);
							} else if ((timeout = modifiers.get(target, key, 'd')) !== undefined) {
								if (debounceHandle !== undefined)
									clearTimeout(debounceHandle);

								debounceHandle = setTimeout(notify, timeout);
							} else {
								this.$schedule(notify);
							}
						});
					}
				};

				const get = () => {
					if (dirty)
						recompute();

					return value;
				};

				const observe = (listener: SubscriptionListener): Subscription => {
					subs.add(listener);
					return {
						revoke() {
							subs.delete(listener);
						}
					};
				};

				for (const extender of extenders) {
					if (extender.init) {
						extender.init({
							get,
							observe,
							dispose() {
								throw new Error('cannot dispose');
							},
						});
					}
				}

				// The function is accually safe to reference because the key cannot be accessed in a weakmap.
				// eslint-disable-next-line @typescript-eslint/unbound-method
				getters.set(descriptior.get!, get);

				return get();
			};

			return new Proxy(object, {
				get: (target: T, key, receiver: T) => {
					const createReference = <T>(link: Link): Reference<T> => ({
						get: (): T => Reflect.get(...link) as T,
						set: (value: T): void => void Reflect.set(...link, value),
						observe: (listener) => this.$observe(link /* satisfies Link */, listener)
					});

					if (isRoot) {
						if (key === '$ref') {
							getRef = true;
							return (link: Link): Reference => {
								return createReference(link);
							};
						}

						if (key === '$silent') {
							silent = true;
							return (value: unknown) => {
								silent = false;
								return value;
							};
						}

						if (
							(typeof key === 'string' && key.startsWith('$'))
							|| modifiers.get(target, key, 'i') === true
						)
							return Reflect.get(target, key, receiver) as unknown;
					}

					if (getRef) {
						getRef = false;
						return createLink(receiver, key);
					}

					const get = (): unknown => {
						const descriptior = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(target), key);

						if (descriptior?.get)
							return computed(target, key, receiver, descriptior);

						return Reflect.get(target, key, receiver);
					};

					const value = get();

					if (!silent)
						this.$events.trigger('get', createLink(target, key));

					if (typeof value === 'object' && value !== null)
						return extend(value, false);

					return value;
				},

				set: (target, key, value, receiver) => {
					const succeeded = Reflect.set(target, key, value, receiver);
					this.$schedule(() => {
						this.$events.trigger('set', createLink(target, key));
					});
					return succeeded;
				}
			});
		};

		const extended = extend(this, true);

		for (const key in this.constructor.prototype) {
			if (Object.prototype.hasOwnProperty.call(this.constructor.prototype, key)) {
				if (modifiers.get(this.constructor.prototype as object, key, 'ef') === true)
					Reflect.get(extended, key);
			}
		}

		return extended;
	}
}
