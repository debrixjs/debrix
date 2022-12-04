import { createMicroTicker, createScheduler, Scheduler, Task, Ticker } from './scheduler';

export type SubscriptionListener = () => void;

export interface Subscription {
	revoke(): void;
}

export type Extender = (cb: () => void) => void;

type Link = [target: object, key: string | symbol];
const createLink = (target: object, key: string | symbol): Link => [target, key];

const GET = 0, MODIFY = 1;

export interface Reference<T = unknown> {
	get(): T
	set(value: T): void
	observe(listener: SubscriptionListener): Subscription;
}

export interface ModelOptions {
	ticker?: Ticker
}

function get<K, V>(map: Map<K, V>, key: K, _default: () => unknown): V {
	let value: V;
	map.has(key) ? value = map.get(key)! : map.set(key, value = _default() as V);
	return value;
}

interface Events {
	emit(type: 0 | 1, link: Link): void
	on(type: 0 | 1, link: Link, listener: () => void): () => void
	on(type: 0 | 1, link: null, listener: (link: Link) => void): () => void
	pipe(to: Events): void
}

function createEvents(): Events {
	const onAll = [
		new Set<(link: Link) => void>(),
		new Set<(link: Link) => void>()
	] as const;
	const onLink = new Map<object, Map<string | symbol, [Set<() => void>, Set<() => void>]>>();
	const copy = new Set<Events>();

	const getOnLink = (type: 0 | 1, link: Link) =>
		get(get(onLink, link[0], () => new Map()), link[1], () => [new Set, new Set])[type];

	return {
		emit(type: 0 | 1, link: Link): void {
			for (const events of copy)
				// eslint-disable-next-line prefer-rest-params
				(events.emit as (...args: unknown[]) => void)(...arguments);

			for (const listener of onAll[type])
				listener(link);

			for (const listener of getOnLink(type, link))
				listener();
		},

		pipe(to: Events) {
			copy.add(to);
		},

		on(type: 0 | 1, link: Link | null, listener: (link: Link) => void): () => void {
			const target = link ? getOnLink(type, link) : onAll[type];
			target.add(listener);
			return () => target.delete(listener);
		}
	};
}

const ATTACH = new WeakMap<typeof Model, Set<PropertyKey>>();
const EXTENDERS = new WeakMap<typeof Model, Map<PropertyKey, Extender>>();

export function attach(target: Model, property: PropertyKey): void;
export function attach(target: unknown, property: PropertyKey): void {
	let set: Set<PropertyKey> | undefined;

	if (set = ATTACH.get(target as typeof Model))
		set.add(property);
	else
		ATTACH.set(target as typeof Model, new Set([property]));

	ignore(target as Model, property);
}

export function extend(extender: Extender): (target: Model, property: PropertyKey) => void {
	return (target: unknown, property: PropertyKey): void => {
		let map: Map<PropertyKey, Extender> | undefined;

		if (map = EXTENDERS.get(target as typeof Model)) {
			if (map.has(property))
				throw new Error('can only extend once');
			else
				map.set(property, extender);
		} else {
			EXTENDERS.set(target as typeof Model, new Map([[property, extender]]));
		}
	};
}

export function ignore(...args: [target: Model, property: PropertyKey]): void {
	extend(() => { })(...args);
}

export abstract class Model {
	/** @internal */
	private declare $e;

	/** @internal */
	private $observe_(link: Link, listener: SubscriptionListener): () => void {
		return this.$e.on(MODIFY, link, listener);
	}

	protected declare $ticker: Ticker;

	/** @internal */
	private declare $s: Scheduler;

	/** @internal */
	private declare $q;

	protected $schedule(task: Task, dedup?: Link): void {
		if (dedup) {
			let set: Set<symbol | string> | undefined;

			if (this.$q.size === 0)
				this.$s.enqueue_(() => this.$q.clear());

			if (set = this.$q.get(dedup[0])) {
				if (set.has(dedup[1])) return;
				else set.add(dedup[1]);
			} else {
				this.$q.set(dedup[0], new Set([dedup[1]]));
			}
		}

		this.$s.enqueue_(task);
	}

	private $scheduleEmit_(type: 0 | 1, link: Link): void {
		this.$schedule(() => this.$e.emit(type, link), link);
	}

	protected $tick(): void {
		this.$s.flush_();
	}

	/** @internal */
	/* #__PURE__ */
	protected $magic(callback: () => void): (listener: SubscriptionListener) => (() => void) {
		const links: Link[] = [];
		const dispose = this.$e.on(GET, null, (link: Link) => links.push(link));

		callback();
		dispose();

		return (listener) => {
			const observers = links.map(link => this.$observe_(link satisfies Link, listener));
			return () => {
				for (const revoke of observers)
					revoke();
			};
		};
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	protected $observe(target: unknown, listener: SubscriptionListener): Subscription;
	protected $observe(target: Link, listener: SubscriptionListener): Subscription {
		return {
			revoke: this.$observe_(target, listener)
		};
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	protected $notify(target: unknown): void {
		throw new Error('proxy was bypassed');
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	protected $ref<T>(target: T): Reference<T> {
		throw new Error('proxy was bypassed');
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	protected $silent<T>(target: T): T {
		throw new Error('proxy was bypassed');
	}

	abstract dispose?(): void;

	constructor(options?: ModelOptions) {
		this.$e = createEvents();
		this.$q = new Map<object, Set<symbol | string>>();
		this.$ticker = options?.ticker ?? createMicroTicker();
		this.$s = createScheduler(this.$ticker);

		const attach = ATTACH.get(Object.getPrototypeOf(this) as typeof Model);
		const extenders = EXTENDERS.get(Object.getPrototypeOf(this) as typeof Model);

		const extend = <T extends object>(object: T, isRoot: boolean): T => {
			let returnLink = false;
			let silent = false;
			const _compted = new WeakMap<object, () => unknown>();

			const asComputed = (target: object, key: string | symbol, receiver: unknown, descriptior: PropertyDescriptor): unknown => {
				// The function is accually safe to reference because the key cannot be accessed in a weakmap.
				/* eslint-disable @typescript-eslint/unbound-method */
				if (_compted.has(descriptior.get!))
					return _compted.get(descriptior.get!)!();


				let value: unknown;
				let revoke: (() => void) | undefined;
				let dirty = true;

				const get = () => {
					if (dirty) {
						revoke?.();
						dirty = false;

						let next!: unknown;
						const observe = this.$magic(() => next = Reflect.get(target, key, receiver) as unknown);

						if (next !== value)
							value = next;

						revoke = observe(() => {
							dirty = true;
							this.$scheduleEmit_(MODIFY, createLink(target, key));
						});
					}

					return value;
				};

				_compted.set(descriptior.get!, get);
				/* eslint-enable */

				return get();
			};

			return new Proxy(object, {
				get: (target: T, key, receiver: T) => {
					if (isRoot) {
						switch (key) {
							case '$observe': {
								returnLink = true;
								break;
							}

							case '$ref': {
								returnLink = true;
								return <T>(link: Link): Reference<T> => ({
									get: (): T => Reflect.get(...link) as T,
									set: (value: T): void => void Reflect.set(...link, value),
									observe: (listener) => ({
										revoke: this.$observe_(link /* satisfies Link */, listener)
									})
								});
							}

							case '$notify': {
								returnLink = true;
								return (link: Link): void =>
									this.$scheduleEmit_(MODIFY, link);
							}

							case '$silent': {
								silent = true;
								return <T>(value: T): T => {
									silent = false;
									return value;
								};
							}
						}

						if (typeof key === 'string' && key.startsWith('$'))
							return Reflect.get(target, key, receiver) as unknown;
					}

					if (returnLink) {
						returnLink = false;
						return createLink(target, key);
					}

					const descriptior = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(target), key);

					if (descriptior?.get)
						return asComputed(target, key, receiver, descriptior);

					const value = Reflect.get(target, key, receiver);

					if (!silent) {
						let extender;
						if (extender = extenders?.get(key)) {
							extender(() => this.$e.emit(GET, createLink(target, key)));
						} else {
							this.$e.emit(GET, createLink(target, key));
						}
					}

					if (typeof value === 'object' && value !== null)
						return extend(value, false);

					return value;
				},

				set: (target, key, value, receiver) => {
					const succeeded = Reflect.set(target, key, value, receiver);

					// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
					if (isRoot && attach?.has(key)) {
						// This is a unsafe convertion from any to Model.
						// However, explicitly checking if the value is a Model is too size consuming.
						const model = value as Model;
						model.$e.pipe(this.$e);
					}

					if (!silent) {
						let extender;
						if (extender = extenders?.get(key)) {
							extender(() => this.$scheduleEmit_(MODIFY, createLink(target, key)));
						} else {
							this.$scheduleEmit_(MODIFY, createLink(target, key));
						}
					}

					return succeeded;
				},

				defineProperty: (target, property, attributes) => {
					// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
					if (isRoot && attach?.has(property)) {
						// This is a unsafe convertion from any to Model | undefined.
						// However, explicitly checking if the value is a Model is too size consuming.
						const model = (attributes.value ?? attributes.get?.()) as Model | undefined;

						if (model)
							model.$e.pipe(this.$e);
					}

					return Reflect.defineProperty(target, property, attributes);
				},
			});
		};

		return extend(this, true);
	}
}
