import { Accessor } from "./binding";
import EventEmitter from "./event-emitter";
import { Revokable } from "./lifecycle";
import { Subscribable, Subscription, SubscriptionListener, UnknownSubscriptionListener } from "./subscription";

export function prop(target: Model, key: string): void {
	Object.defineProperty(target, key, {
		get() {
			return (target as typeof target & { props: typeof target extends Model<infer Props> ? Props : never }).props?.[key];
		}
	});
}

/**
 * Symbol placed on objects wrapped by {@link Model}.
 * 
 * @internal
 * Avoid using this externally. It is meant to be a last resort. Please read the docs before using.
 */
export const WRAPPED = Symbol("debrix");

type Ref = readonly PropertyKey[] & { readonly __brand: unique symbol };

function isSameRef(refx: Ref, refy: Ref): boolean {
	return refx.length === refy.length && refx.find((x, i) => x === refy[i]) === undefined;
}

function createRef(to: readonly PropertyKey[]) {
	return to as Ref;
}

export abstract class Model<Props extends object = Record<string, unknown>> implements Subscribable {
	/** Resubscriptions for getter properties. */
	#resubscriptions = new Map<Ref, Subscription>();
	/** Local state event emitter. */
	#events = new EventEmitter<{
		set: [ref: Ref, value: unknown],
		get: [ref: Ref],
		delete: [ref: Ref]
	}>();

	#subscribe(refs: Ref[], listener: (value: unknown, ref: Ref) => void): Subscription {
		const lis = (ref: Ref, value?: unknown) => {
			if (refs.find(target => target === ref)) listener(value, ref);
		};
		const revs = (["set", "delete"] as const).map(ev => this.#events.on(ev, lis));
		return {
			revoke() {
				for (const rev of revs) rev.revoke();
			}
		};
	}

	/**
	 * Subscribes for further changes on the provided property.
	 * 
	 * ```typescript
	 * const subscription = this.subscribe(this.name, () => ...)
	 * ...
	 * subscription.revoke()
	 * ```
	 */
	$subscribe(target: /* Ref */ unknown, listener: SubscriptionListener): Subscription {
		return this.#subscribe([target as Ref], value => listener(value));
	}

	/**
	 * Detects dependencies from when this method is called until the returned callback is called.
	 * Subscribes to the dependencies for further changes.
	 * 
	 * ```typescript
	 * const end_resubscription = this.resubscribe(() => {
	 * 	// called when a dependency changes
	 * })
	 * ...
	 * const subscription = end_resubscription()
	 * ...
	 * subscription.revoke()
	 * ```
	 */
	$subscribeWhile(inner: () => void, listener: UnknownSubscriptionListener): Subscription {
		const deps: [get: Ref[], del: Ref[]] = [[], []];
		const revs: [get: Revokable, del: Revokable] = [
			this.#events.on("get", ref => deps[0].push(ref)),
			this.#events.on("delete", ref => deps[1].push(ref))
		];

		inner();

		for (const rev of revs) rev.revoke();
		const subs = [
			this.#subscribe(deps[0], () => listener()),
			this.#subscribe(deps[1], (_value, refx) => {
				deps[1].splice(deps[1].findIndex(refy => isSameRef(refx, refy)), 1);
				listener();
			})
		];

		return {
			revoke() {
				for (const sub of subs) sub.revoke();
			}
		};
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	$accessor<T>(target: T): Accessor<T> {
		throw new Error(`'${this.$accessor.name}' is not implemented!`);
	}

	$computed<T>(callback: () => T): Accessor<T> {
		const listeners: (SubscriptionListener | null)[] = [];
		let prev: Subscription | undefined;
		const accessor: Accessor<T> = {
			get: () => {
				prev?.revoke();

				let value: T;
				prev = this.$subscribeWhile(
					() => value = callback(),
					() => {
						const value = accessor.get();
						for (const listener of listeners)
							listener?.(value);
					}
				);
				return value!;
			},
			subscribe(listener) {
				const index = listeners.push(listener) - 1;
				return {
					revoke() {
						listeners[index] = null;
					}
				};
			}
		};
		return accessor;
	}

	constructor(protected props?: Props) {
		/** If true, wrapped objects will return refs instead of values. */
		let returnRef = false;

		const wrap = <T extends object>(target: T, parent?: Ref): T => {
			const getRef = (key: PropertyKey): Ref => createRef([...parent ?? [], key]);

			return new Proxy(target, {
				get: (target, key, receiver) => {
					const ref = getRef(key);

					if (returnRef) return ref;

					if (!parent) {
						if (key === this.$subscribe.name) {
							returnRef = true;
							return (...args: unknown[]) => {
								returnRef = false;
								return (this.$subscribe as (...args: unknown[]) => unknown)(...args);
							};
						}

						if (key === this.$accessor.name) {
							returnRef = true;
							return (ref: Ref) => {
								returnRef = false;

								return {
									get() {
										let newTarget: any = target;

										for (const key of ref)
											newTarget = Reflect.get(newTarget, key);

										return newTarget;
									},

									set(value: unknown) {
										let newTarget: any = target;

										for (const key of ref.slice(0, -1))
											newTarget = Reflect.get(newTarget, key);

										Reflect.set(newTarget, key, value);
									}
								};
							};
						}
					}

					const descriptor = Reflect.getOwnPropertyDescriptor(target, key);
					if (!descriptor) return undefined;
					this.#events.trigger("get", ref);

					// If the descriptor has a setter, the property is treated as a "computed" property.
					if (descriptor.get) {
						// Revoke the old subscription.
						if (this.#resubscriptions.has(ref))
							this.#resubscriptions.get(ref)!.revoke();

						// This trigger the "virtual" set. The value has not changed yet because it is a computed.
						// Instead the "set" event is triggered to let others know that the computed will possibly
						// return another result, because the variables/factors has changed.
						let value: unknown;
						const subscription = this.$subscribeWhile(
							() => value = Reflect.get(target, key, receiver),
							() => this.#events.trigger("set", ref, value)
						);

						// Remember the subscription so that it can be revoked when getting the property.
						this.#resubscriptions.set(ref, subscription);

						// Continue to trap child objects to all capture events.
						if (value !== null && typeof value === "object" && Object.prototype.hasOwnProperty.call(value, WRAPPED))
							value = wrap(value, ref);

						return value;
					} else {
						return Reflect.get(target, key, receiver);
					}
				},

				set: (target, key, value, receiver) => {
					const ref = getRef(key);
					const success = Reflect.set(target, key, value, receiver);
					if (success) this.#events.trigger("set", value, ref);
					return success;
				},

				deleteProperty: (target, key) => {
					const ref = getRef(key);
					const success = Reflect.deleteProperty(target, key);
					if (success) this.#events.trigger("delete", ref);
					return success;
				}
			});
		};

		return wrap(this);
	}
}
