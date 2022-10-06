import { Accessor } from './binding';
import EventEmitter from './event-emitter';
import Modifiers from './modifiers';
import Scheduler, { Task } from './scheduler';
import { type Subscription, type SubscriptionListener } from './subscription';

interface Link {
	target: object,
	key: string | symbol,
}

function createLink(target: object, key: string | symbol): Link {
	return { target, key };
}

function isEqualLink(x: Link, y: Link): boolean {
	return x.target === y.target && x.key === y.key;
}

interface Observable {
	observe(listener: SubscriptionListener): Subscription;
}

export type Referenced<T> = T extends object ? { [K in keyof T]: Referenced<T[K]> } : Accessor<T>;

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
	initialize?(target: Computed<T>): void
}

interface ModifierMap {
	ignore?: boolean;
	effect?: boolean;
	throttle?: number;
	debounce?: number;
	readonly?: boolean;
	extend?: Extender<unknown>[];
}

const modifiers = new Modifiers<ModifierMap>();

function assertNotModifier(target: object, key: string | symbol, modifier: keyof ModifierMap, message: string) {
	if (modifiers.has(target, key, modifier))
		throw new Error(message);
}

function assertNotIgnored(target: object, key: string | symbol, message = `Property ${String(key)} is ignored.`) {
	assertNotModifier(target, key, 'ignore', message);
}

export function ignore(target: object, key: string | symbol) {
	assertNotIgnored(target, key, `Property ${String(key)} is already ignored.`);
	modifiers.set(target, key, 'ignore', true);
}

export function effect(target: object, key: string | symbol) {
	assertNotIgnored(target, key);
	modifiers.set(target, key, 'effect', true);
}

export function throttle(delay: number) {
	return (target: object, key: string | symbol) => {
		assertNotIgnored(target, key);
		assertNotModifier(target, key, 'throttle', `Property ${String(key)} is already throttled.`);
		assertNotModifier(target, key, 'debounce', `Cannot throttle property ${String(key)}. Property is already debounced.`);
		modifiers.set(target, key, 'throttle', delay);
	};
}

export function debounce(delay: number) {
	return (target: object, key: string | symbol) => {
		assertNotIgnored(target, key);
		assertNotModifier(target, key, 'debounce', `Property ${String(key)} is already debounced.`);
		assertNotModifier(target, key, 'throttle', `Cannot debounce property ${String(key)}. Property is already throttled.`);
		modifiers.set(target, key, 'debounce', delay);
	};
}

export function readonly(target: object, key: string | symbol) {
	assertNotModifier(target, key, 'readonly', `Property ${String(key)} is already readonly.`);
	modifiers.set(target, key, 'readonly', true);
}

export function extend(extender: Extender<unknown>) {
	return (target: object, key: string | symbol) => {
		if (modifiers.has(target, key, 'extend'))
			modifiers.get(target, key, 'extend')!.push(extender);
		else
			modifiers.set(target, key, 'extend', [extender]);
	};
}

export abstract class Model {
	/** @internal */
	$__events = new EventEmitter<{ set: [link: Link], get: [link: Link] }>();

	/** @internal */
	$__scheduler = new Scheduler();

	/** @internal */
	$__observe(property: Link, listener: SubscriptionListener): Subscription {
		return this.$__events.on('set', (link) => {
			if (isEqualLink(property, link))
				listener();
		});
	}

	/** @internal */
	$__observeDependencies(callback: () => void, listener: SubscriptionListener): Subscription {
		const dependencies: Link[] = [];
		const dependencyListener = this.$__events.on('get', (link) => dependencies.push(link));
		callback();
		dependencyListener.revoke();

		const listeners = dependencies.map(dependency =>
			this.$__observe(dependency, () => {
				listener();
			})
		);

		return {
			revoke() {
				for (const listener of listeners)
					listener.revoke();
			},
		};
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	$silent<T>(target: T): T {
		throw new Error('not implemented');
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	$ref<T>(target: T): Reference<T> {
		throw new Error('not implemented');
	}

	$observe(target: unknown, listener: () => void): Subscription {
		return this.$__observe(target as Link, listener);
	}

	/**
	 * Creates an computed _value_ which recomputes when a dependency in the current model changes.
	 * 
	 * @example
	 * ```typescript
	 * // This will recompute when the value of `x` or `y` changes.
	 * this.$computed(() => {
	 * 	return this.x + this.y
	 * })
	 * ```
	 */
	$computed<T>(get: () => T): Computed<T> {
		const subscriptions: SubscriptionListener[] = [];
		let value: T;
		let dirty = true;
		let dispose: (() => void) | undefined;
		let disposed = false;

		const assertNotDisposed = () => {
			if (disposed)
				throw new Error('Reference has been disposed.');
		};

		return {
			get: () => {
				assertNotDisposed();

				if (dirty) {
					const dependencies: Link[] = [];
					const listener = this.$__events.on('get', link => dependencies.push(link));
					const next = get();
					listener.revoke();
					dirty = false;

					if (next !== value) {
						value = next;

						dispose?.();

						const listeners = dependencies.map(dependency => this.$__observe(dependency, () => {
							dirty = true;
							this.$__scheduler.enqueue(() => {
								subscriptions.forEach(listener => listener());
							});
						}));

						dispose = () => listeners.map(l => l.revoke());
					}
				}

				return value;
			},

			observe: (listener) => {
				assertNotDisposed();

				const index = subscriptions.push(listener) - 1;
				return {
					revoke() {
						delete subscriptions[index];
					}
				};
			},

			dispose: () => {
				assertNotDisposed();

				disposed = true;
				dispose?.();
			},
		};
	}

	$schedule(task: Task): void {
		this.$__scheduler.enqueue(task);
	}

	$tick(): void {
		this.$__scheduler.flush();
	}

	constructor() {
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
				let debounceTimeout: number | undefined;
				const observers: (SubscriptionListener | undefined)[] = [];

				const extenders = modifiers.get(target, key, 'extend') ?? [];

				const get = () => {
					if (dirty) {
						const dependencies: Link[] = [];
						const listener = this.$__events.on('get', link => dependencies.push(link));
						const next: unknown = Reflect.get(target, key, receiver);
						listener.revoke();
						dirty = false;

						if (next !== value) {
							value = next;

							for (const extender of extenders) {
								if (extender.compute)
									value = extender.compute(value);
							}

							dispose?.();

							// eslint-disable-next-line @typescript-eslint/no-misused-promises
							const listeners = dependencies.map(dependency => this.$__observe(dependency, async () => {
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
									this.$__events.trigger('set', createLink(target, key));
									for (const listener of observers)
										listener?.();
								};

								if (modifiers.has(target, key, 'throttle')) {
									const timeout = modifiers.get(target, key, 'throttle')!;

									setTimeout(notify, timeout);
								} else if (modifiers.has(target, key, 'debounce')) {
									const timeout = modifiers.get(target, key, 'debounce')!;

									if (debounceTimeout !== undefined)
										clearTimeout(debounceTimeout);

									debounceTimeout = setTimeout(notify, timeout);
								} else {
									this.$schedule(notify);
								}
							}));

							dispose = () => listeners.map(l => l.revoke());
						}
					}

					return value;
				};

				const observe = (listener: SubscriptionListener): Subscription => {
					const index = observers.push(listener) - 1;
					return {
						revoke() {
							observers[index] = undefined;
						}
					};
				};

				for (const extender of extenders) {
					if (extender.initialize) {
						extender.initialize({
							get: get,
							observe,
							dispose() {
								throw new Error('Cannot dispose of computed property.');
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
						get: (): T => Reflect.get(link.target, link.key) as T,
						set: (value: T): void => void Reflect.set(link.target, link.key, value),
						observe: (listener) => this.$__observe(link, listener)
					});

					if (isRoot) {
						if (key === this.$ref.name) {
							getRef = true;
							return (link: Link): Reference => {
								return createReference(link);
							};
						}

						if (key === this.$silent.name) {
							silent = true;
							return (value: unknown) => {
								silent = false;
								return value;
							};
						}

						if (
							(typeof key === 'string' && key.startsWith('$'))
							|| modifiers.get(target, key, 'ignore') === true
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
						this.$__events.trigger('get', createLink(target, key));

					if (typeof value === 'object' && value !== null)
						return extend(value, false);

					return value;
				},

				set: (target, key, value, receiver) => {
					const succeeded = Reflect.set(target, key, value, receiver);
					this.$schedule(() => {
						this.$__events.trigger('set', createLink(target, key));
					});
					return succeeded;
				}
			});
		};

		const extended = extend(this, true);

		for (const key in this.constructor.prototype) {
			if (Object.prototype.hasOwnProperty.call(this.constructor.prototype, key)) {
				if (modifiers.get(this.constructor.prototype as object, key, 'effect') === true)
					Reflect.get(extended, key);
			}
		}

		return extended;
	}
}
