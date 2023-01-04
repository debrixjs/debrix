import {
	createMicroTicker,
	createScheduler,
	Scheduler,
	Task,
	Ticker,
} from './scheduler';

export type Diff = [name: string, diff: Diff][];

export interface Changes {
	additions: Diff;
	modifications: Diff;
	deletions: Diff;
}

export type SubscriptionListener<T = unknown> = (
	newValue?: T,
	oldValue?: T,
	changes?: Changes
) => void;

export interface Subscription {
	revoke(): void;
}

export type Extender = (cb: () => void) => void;

//#region link

const LINK = Symbol('link');
const CHAIN = Symbol('chain');

type Link = readonly [target: object, key: string | symbol] & { [LINK]: true };
type Chain = readonly Link[] & { [CHAIN]: true };

function createLink(target: object, key: string | symbol): Link {
	return Object.assign([target, key] as const, { [LINK]: true as const });
}

function createChain(...links: readonly Link[]): Chain {
	return Object.assign(links.slice(), { [CHAIN]: true as const });
}

function attachLink(chain: Chain | undefined, link: Link): Chain {
	return createChain(...(chain ?? []), link);
}

function lastLinkOf(chain: Chain): Link {
	return chain[chain.length - 1]!;
}

function isEqualLink(link1: Link, link2: Link) {
	return link1[0] === link2[0] && link1[1] === link2[1];
}

function isLink(value: unknown): value is Link {
	// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
	return !!value && typeof value === 'object' && LINK in value;
}

//#endregion link

export interface Reference<T = unknown> {
	get(): T;
	set(value: T): void;
	observe(listener: SubscriptionListener<T>): Subscription;
}

export interface ModelOptions {
	ticker?: Ticker;
}

enum EventType {
	Get,
	Modify,
}

interface Event2<T = unknown> {
	readonly type: EventType;
	readonly chain: Chain;
	readonly oldValue?: T;
	readonly newValue?: T;
	readonly changes?: Changes;
}

interface EventFilter {
	readonly type?: EventType;
	readonly link?: Link | readonly Link[];
}

interface Events {
	emit(event: Event2): void;
	on<T = unknown>(
		listener: (event: Event2<T>) => void,
		filter?: EventFilter
	): () => void;
	pipe(to: Events): void;
}

function createEvents(): Events {
	const listeners = new Set<
		readonly [(event: Event2<any>) => void, EventFilter | undefined]
	>();
	const copy = new Set<Events>();

	return {
		emit(event): void {
			// Listeners needs to be copied in order to prevent infinite loops.
			for (const [listener, filter] of Array.from(listeners)) {
				if (filter) {
					if (filter.link) {
						const link = lastLinkOf(event.chain);
						if (
							isLink(filter.link)
								? !isEqualLink(link, filter.link)
								: !filter.link.some((f) => isEqualLink(link, f))
						)
							continue;
					}

					if (filter.type && event.type !== filter.type) continue;
				}

				listener(event);
			}

			for (const events of copy) events.emit(event);
		},

		pipe(to: Events) {
			copy.add(to);
		},

		on(listener, filter): () => void {
			const value = [listener, filter] as const;
			listeners.add(value);

			return () => {
				listeners.delete(value);
			};
		},
	};
}

const ATTACH = new WeakMap<typeof Model, Set<PropertyKey>>();
const EXTENDERS = new WeakMap<typeof Model, Map<PropertyKey, Extender>>();

export function attach(target: Model, property: PropertyKey): void;
export function attach(target: unknown, property: PropertyKey): void {
	let set: Set<PropertyKey> | undefined;

	if ((set = ATTACH.get(target as typeof Model))) set.add(property);
	else ATTACH.set(target as typeof Model, new Set([property]));

	ignore(target as Model, property);
}

export function extend(
	extender: Extender
): (target: Model, property: PropertyKey) => void {
	return (target: unknown, property: PropertyKey): void => {
		let map: Map<PropertyKey, Extender> | undefined;

		if ((map = EXTENDERS.get(target as typeof Model))) {
			if (map.has(property)) throw new Error('can only extend once');
			else map.set(property, extender);
		} else {
			EXTENDERS.set(target as typeof Model, new Map([[property, extender]]));
		}
	};
}

export function ignore(...args: [target: Model, property: PropertyKey]): void {
	extend(() => {})(...args);
}

export abstract class Model {
	/** @internal */
	private declare $e;

	protected declare $ticker: Ticker;

	/** @internal */
	private declare $s: Scheduler;

	/** @internal */
	private declare $q;

	protected $schedule(task: Task, dedup?: Link): void {
		if (dedup) {
			let set: Set<symbol | string> | undefined;

			if (this.$q.size === 0) this.$s.enqueue_(() => this.$q.clear());

			if ((set = this.$q.get(dedup[0]))) {
				if (set.has(dedup[1])) return;
				else set.add(dedup[1]);
			} else {
				this.$q.set(dedup[0], new Set([dedup[1]]));
			}
		}

		this.$s.enqueue_(task);
	}

	private $scheduleEmit_(event: Event2): void {
		this.$schedule(() => this.$e.emit(event), lastLinkOf(event.chain));
	}

	protected $tick(): void {
		this.$s.flush_();
	}

	/** @internal */
	/* #__PURE__ */
	protected $magic(callback: () => void): (listener: () => void) => () => void {
		const filter: Link[] = [];
		const dispose = this.$e.on(
			(event) => filter.push(lastLinkOf(event.chain)),
			{ type: EventType.Get }
		);

		callback();
		dispose();

		return (listener) =>
			// In cases where filter is an empty array, the listener will never be called.
			this.$e.on(listener, { type: EventType.Modify, link: filter });
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	protected $observe<T>(
		target: T,
		listener: SubscriptionListener<T>
	): Subscription;
	protected $observe(link: Link, listener: SubscriptionListener): Subscription {
		return {
			revoke: this.$e.on(
				(event) => listener(event.newValue, event.oldValue, event.changes),
				{ type: EventType.Modify, link }
			),
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

	dispose?(): void;

	constructor(options?: ModelOptions) {
		this.$e = createEvents();
		this.$q = new Map<object, Set<symbol | string>>();
		this.$ticker = options?.ticker ?? createMicroTicker();
		this.$s = createScheduler(this.$ticker);

		const attach = ATTACH.get(Object.getPrototypeOf(this) as typeof Model);
		const extenders = EXTENDERS.get(
			Object.getPrototypeOf(this) as typeof Model
		);

		const extend = <T extends object>(object: T, chain?: Chain): T => {
			const isRoot = !chain;
			let returnLink = false;
			let returnChain = false;
			let silent = false;
			const _compted = new WeakMap<object, () => unknown>();

			const asComputed = (
				target: object,
				key: string | symbol,
				receiver: unknown,
				descriptior: PropertyDescriptor
			): unknown => {
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

						const oldValue = value;
						let newValue!: unknown;
						const observe = this.$magic(
							() => (newValue = Reflect.get(target, key, receiver) as unknown)
						);

						if (newValue !== value) value = newValue;

						revoke = observe(() => {
							dirty = true;
							this.$scheduleEmit_({
								chain: attachLink(chain, createLink(target, key)),
								type: EventType.Modify,
								oldValue: oldValue,
								newValue: value,
							});
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
								returnChain = true;
								break;
							}

							case '$ref': {
								returnLink = true;
								return <T>(link: Link): Reference<T> => ({
									get: (): T =>
										Reflect.get(
											...(link as readonly [
												target: object,
												key: string | symbol
											])
										) as T,
									set: (value: T): void =>
										void Reflect.set(
											...(link as readonly [
												target: object,
												key: string | symbol
											]),
											value
										),
									observe: (listener) => ({
										revoke: this.$e.on<T>(
											(event) =>
												listener(event.newValue, event.oldValue, event.changes),
											{ type: EventType.Modify, link }
										),
									}),
								});
							}

							case '$notify': {
								returnChain = true;
								return (chain: Chain): void =>
									this.$scheduleEmit_({
										type: EventType.Modify,
										chain,
									});
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

					if (returnChain) {
						returnChain = false;
						return attachLink(chain, createLink(target, key));
					}

					const descriptior = Object.getOwnPropertyDescriptor(
						Object.getPrototypeOf(target),
						key
					);

					if (descriptior?.get)
						return asComputed(target, key, receiver, descriptior);

					const value = Reflect.get(target, key, receiver);

					if (!silent) {
						this.$e.emit({
							type: EventType.Get,
							chain: attachLink(chain, createLink(target, key)),
							newValue: value,
						});
					}

					if (typeof value === 'object' && value !== null)
						return extend(value, attachLink(chain, createLink(target, key)));

					return value;
				},

				set: (target, key, newValue: unknown, receiver) => {
					const oldValue = Reflect.get(receiver, key) as unknown;
					const succeeded = Reflect.set(target, key, newValue, receiver);

					// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
					if (isRoot && attach?.has(key)) {
						// This is a unsafe convertion from any to Model.
						// However, explicitly checking if the value is a Model is too size consuming.
						const model = newValue as Model;
						model.$e.pipe(this.$e);
					}

					if (!silent) {
						const notify = () =>
							this.$scheduleEmit_({
								type: EventType.Modify,
								chain: attachLink(chain, createLink(target, key)),
								newValue,
								oldValue,
							});

						const extender = extenders?.get(key);
						if (extender) extender(notify);
						else notify();
					}

					return succeeded;
				},

				defineProperty: (target, property, attributes) => {
					// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
					if (isRoot && attach?.has(property)) {
						// This is a unsafe convertion from any to Model | undefined.
						// However, explicitly checking if the value is a Model is too size consuming.
						const model = (attributes.value ?? attributes.get?.()) as
							| Model
							| undefined;

						if (model) model.$e.pipe(this.$e);
					}

					return Reflect.defineProperty(target, property, attributes);
				},
			});
		};

		return extend(this);
	}
}
