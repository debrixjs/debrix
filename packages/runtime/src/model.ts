import { Accessor } from "./binding";
import EventEmitter from "./event-emitter";
import { Revokable } from "./lifecycle";
import { Subscribable, Subscription, SubscriptionListener } from "./subscription";

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
	$resubscribe(listener: SubscriptionListener): () => Subscription {
		const deps: [get: Ref[], del: Ref[]] = [[], []];
		const revs: [get: Revokable, del: Revokable] = [
			this.#events.on("get", ref => deps[0].push(ref)),
			this.#events.on("delete", ref => deps[1].push(ref))
		];
		return () => {
			for (const rev of revs) rev.revoke();
			const subs = [
				this.#subscribe(deps[0], value => listener(value)),
				this.#subscribe(deps[1], (value, refx) => {
					deps[1].splice(deps[1].findIndex(refy => isSameRef(refx, refy)), 1);
					listener(value);
				})
			];
			return {
				revoke() {
					for (const sub of subs) sub.revoke();
				}
			};
		};
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	$reference<T>(target: T): Accessor<T> {
		throw new Error("'$reference' was never implemented by proxy!");
	}

	constructor(protected props?: Props) {
		/** If true, wrapped objects will return refs instead of values. */
		let returnRef = false;

		const wrap = <T extends object>(target: T, parent?: Ref): T => {
			const getRef = (key: PropertyKey): Ref => createRef([...parent ?? [], key]);

			return new Proxy(target, {
				get: (target, key, receiver) => {
					const ref = getRef(key);

					if (!parent) {
						// All properties will return refs instead of values.
						// REMEMBER! 'returnRef' needs to be set to false as soon as the
						// function is no longer expected to receive refs.
						returnRef = true;

						if (key === "$subscribe") {
							return (...args: unknown[]) => {
								returnRef = false;
								return (this.$subscribe as (...args: unknown[]) => unknown)(...args);
							};
						}

						if (key === "$reference") {
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

					if (returnRef) return ref;

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
						const endResub = this.$resubscribe(value => this.#events.trigger("set", ref, value));
						let value = Reflect.get(target, key, receiver);

						// Continue to trap child objects to all capture events.
						if (value !== null && typeof value === "object" && Object.prototype.hasOwnProperty.call(value, WRAPPED))
							value = wrap(value, ref);

						// Remember the subscription so that it can be revoked when getting the property. See code above.
						this.#resubscriptions.set(ref, endResub());

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
