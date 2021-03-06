/* eslint-disable */

import EventEmitter from "./event-emitter";
import Modifiers from "./modifiers";
import Scheduler, { Task } from "./scheduler";
import { type Subscription, type SubscriptionListener } from "./subscription";

type PropRef = (string | symbol)[];

function isEqualRef(x: PropRef, y: PropRef): boolean {
	return !x.find((key, i) => key !== y[i]);
}

interface Observable {
	observe(listener: SubscriptionListener): Subscription;
}

export interface Reference<T = unknown> extends Observable {
	get(): T
	set(value: T): boolean
	dispose(): void
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

export abstract class Model {
	static ignore(target: object, key: string | symbol) {
		assertNotIgnored(target, key, `Property ${String(key)} is already ignored.`);
		modifiers.set(target, key, 'ignore', true);
	}

	static effect(target: object, key: string | symbol) {
		assertNotIgnored(target, key);
		modifiers.set(target, key, 'effect', true);
	}

	static throttle(delay: number) {
		return (target: object, key: string | symbol) => {
			assertNotIgnored(target, key);
			assertNotModifier(target, key, 'throttle', `Property ${String(key)} is already throttled.`);
			assertNotModifier(target, key, 'debounce', `Cannot throttle property ${String(key)}. Property is already debounced.`);
			modifiers.set(target, key, 'throttle', delay);
		}
	}

	static debounce(delay: number) {
		return (target: object, key: string | symbol) => {
			assertNotIgnored(target, key);
			assertNotModifier(target, key, 'debounce', `Property ${String(key)} is already debounced.`);
			assertNotModifier(target, key, 'throttle', `Cannot debounce property ${String(key)}. Property is already throttled.`);
			modifiers.set(target, key, 'debounce', delay);
		}
	}

	static readonly(target: object, key: string | symbol) {
		assertNotModifier(target, key, 'readonly', `Property ${String(key)} is already readonly.`);
		modifiers.set(target, key, 'readonly', true);
	}

	static extend(extender: Extender<unknown>) {
		return (target: object, key: string | symbol) => {
			if (modifiers.has(target, key, 'extend'))
				modifiers.get(target, key, 'extend')!.push(extender);
			else
				modifiers.set(target, key, 'extend', [extender]);
		}
	}

	/** @internal */
	$__events = new EventEmitter<{ set: [ref: PropRef], get: [ref: PropRef] }>();

	/** @internal */
	$__scheduler = new Scheduler();

	/** @internal */
	$__observe(property: PropRef, listener: SubscriptionListener): Subscription {
		return this.$__events.on('set', (ref) => {
			if (isEqualRef(property, ref))
				listener();
		});
	}

	/** @internal */
	$__observeDependencies(callback: () => void, listener: SubscriptionListener): Subscription {
		let dependencies: PropRef[] = [];
		const dependencyListener = this.$__events.on('get', (ref) => dependencies.push(ref));
		callback();
		dependencyListener.revoke();

		const listeners = dependencies.map(dependency =>
			this.$__observe(dependency, () => {
				listener()
			})
		);

		return {
			revoke() {
				for (const listener of listeners)
					listener.revoke();
			},
		}
	}

	/**
	 * Creates a reference to the property.
	 * 
	 * @example
	 * ```typescript
	 * this.$reference(this.myProp)
	 * ```
	 * 
	 * @param target The property in the model to reference
	 */
	$reference(target: unknown): Reference {
		const ref = target as PropRef;
		let subscriptions: Subscription[] = [];
		let disposed = false;

		const assertNotDisposed = () => {
			if (disposed)
				throw new Error('Reference has been disposed.');
		}

		return {
			get: () => {
				assertNotDisposed();

				const keys = [...ref];
				let target = (this as any)[keys.shift()!];

				for (const key of keys) {
					if (key in target)
						target = target[key]
					else
						return undefined;
				}

				return target;
			},

			set: (value) => {
				assertNotDisposed();

				const keys = [...ref];
				const key = keys.pop()!;
				let target = keys.length ? (this as any)[keys.shift()!] : this as any;

				for (const key of keys) {
					if (key in target)
						target = target[key];
					else
						return false;
				}

				target[key] = value;
				return true;
			},

			observe: (listener) => {
				assertNotDisposed();

				const subscription = this.$__observe(ref, listener);
				subscriptions.push(subscription);
				return subscription;
			},

			dispose: () => {
				assertNotDisposed();

				disposed = true;
				for (const subscription of subscriptions)
					subscription.revoke();
			},
		}
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
		let subscriptions: SubscriptionListener[] = [];
		let value: T;
		let dirty = true;
		let dispose: (() => void) | undefined;
		let disposed = false;

		const assertNotDisposed = () => {
			if (disposed)
				throw new Error('Reference has been disposed.');
		}

		return {
			get: () => {
				assertNotDisposed();

				if (dirty) {
					let dependencies: PropRef[] = [];
					const listener = this.$__events.on('get', ref => dependencies.push(ref));
					const next = get();
					listener.revoke();
					dirty = false;

					if (next != value) {
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
		const extend = (object: any, parent?: PropRef): any => {
			const ref = (key: string | symbol): PropRef => [...parent ?? [], key];
			let refNext = false;

			const getters = new WeakMap<object, () => unknown>();

			const computed = (target: object, key: string | symbol, receiver: object, descriptior: PropertyDescriptor): unknown => {
				if (getters.has(descriptior.get!))
					return getters.get(descriptior.get!)!()

				let value: unknown;
				let dirty = true;
				let dispose: (() => void) | undefined;
				let debounceTimeout: number | undefined;
				let observers: SubscriptionListener[] = [];

				const extenders = modifiers.get(target, key, 'extend') ?? [];

				const computed = () => {
					if (dirty) {
						let dependencies: PropRef[] = [];
						const listener = this.$__events.on('get', ref => dependencies.push(ref));
						const next = Reflect.get(target, key, receiver);
						listener.revoke();
						dirty = false;

						if (next != value) {
							value = next;

							for (const extender of extenders) {
								if (extender.compute)
									value = extender.compute(value);
							}

							dispose?.();

							const listeners = dependencies.map(dependency => this.$__observe(dependency, async () => {
								dirty = true;

								for (const extender of extenders) {
									if (extender.recompute && !await extender.recompute()) {
										dirty = false;
										return;
									}

									if (extender.notify && !extender.notify(value))
										return;
								}

								const notify = () => {
									this.$__events.trigger('set', ref(key));
									for (const listener of observers)
										listener();
								}

								if (modifiers.has(target, key, 'throttle')) {
									const timeout = modifiers.get(target, key, 'throttle')!;

									setTimeout(notify, timeout);
								} else if (modifiers.has(target, key, 'debounce')) {
									const timeout = modifiers.get(target, key, 'debounce')!;

									if (debounceTimeout)
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
				}

				const observe = (listener: SubscriptionListener): Subscription => {
					const index = observers.push(listener) - 1;
					return {
						revoke() {
							delete observers[index];
						}
					}
				}

				for (const extender of extenders) {
					if (extender.initialize) {
						extender.initialize({
							get: computed,
							observe,
							dispose() {
								throw new Error('Cannot dispose of computed property.');
							},
						});
					}
				}

				getters.set(descriptior.get!, computed);

				return computed();
			}

			return new Proxy(object, {
				get: (target, key, receiver) => {
					if (!parent && key === this.$reference.name)
						refNext = true;

					if (
						(!parent && typeof key === 'string' && key.startsWith('$'))
						|| modifiers.get(target, key, 'ignore')
					)
						return Reflect.get(target, key, receiver);

					if (refNext) {
						refNext = false;
						return ref(key);
					}

					const get = (): unknown => {
						const descriptior = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(target), key);

						if (descriptior?.get)
							return computed(target, key, receiver, descriptior);

						return Reflect.get(target, key, receiver);
					}

					const value = get();
					this.$__events.trigger('get', ref(key));

					if (typeof value === 'object' && value !== null)
						return extend(value, ref(key));

					return value;
				},

				set: (target, key, value, receiver) => {
					const succeeded = Reflect.set(target, key, value, receiver);
					this.$schedule(() => {
						this.$__events.trigger('set', ref(key));
					});
					return succeeded;
				}
			});
		}

		const extended = extend(this);

		for (const key in this.constructor.prototype) {
			if (Object.prototype.hasOwnProperty.call(this.constructor.prototype, key)) {
				if (modifiers.get(this.constructor.prototype, key, 'effect'))
					Reflect.get(extended, key);
			}
		}

		return extended;
	}
}
