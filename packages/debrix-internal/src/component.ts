/* eslint-disable */

import { Changes, Component as _Component, ComponentAttrs, ComponentOptions as _ComponentOptions, Computed, Model, ViewModel } from 'debrix';
import { destroy, detach, insert } from './document';
import { entries, FRAGMENT, Fragment, hasOwn } from './utils';

class _SelfAttrs extends Model {
	[name: string]: any

	/** @internal */
	constructor(initial: ComponentAttrs) {
		super();

		let prev: Record<string, string | Computed<string>> = {};
		let _oldValue: Record<string, string> = {};

		const apply = (
			newValue2?: Record<string, string>,
			oldValue2?: Record<string, string>,
			changes?: Changes
		) => {
			const newValue = newValue2 ?? initial._!.get();
			const oldValue = oldValue2 ?? _oldValue;
			_oldValue = newValue;

			if (changes) {
				for (const [key] of [...changes.additions, ...changes.modifications]) {
					if (!hasOwn(prev, key))
						prev[key] = this[key];

					this[key] = newValue[key];
				}

				for (const [key] of changes.deletions) {
					if (hasOwn(prev, key)) {
						this[key] = prev[key];
						delete prev[key];
					} else {
						delete this[key];
					}
				}
			} else {
				for (const key of new Set([...Object.keys(oldValue), ...Object.keys(newValue)])) {
					const inOld = hasOwn(oldValue, key);
					const inNew = hasOwn(newValue, key);

					if (inOld) {
						if (inNew) {
							this[key] = newValue[key];
						} else if (hasOwn(prev, key)) {
							this[key] = prev[key];
							delete prev[key];
						} else {
							delete this[key];
						}
					} else if (inNew) {
						prev[key] = this[key];
						this[key] = newValue[key];
					}
				}
			}
		};

		if (initial._) {
			apply();
			initial._.observe(apply);
		}

		for (const [key, value] of entries(initial)) {
			if (key === '_')
				continue;

			if (typeof value === 'string') {
				this[key] = value;
				continue;
			}

			const computed = value as Computed<string>;
			const apply = (newValue?: string) => {
				this[key] = newValue ?? value.get();
			};

			apply();
			computed.observe(apply);
		}
	}
}

function createSelfAttrs(initial: ComponentAttrs) {
	return new _SelfAttrs(initial) as SelfAttrs;
}

export type SelfAttrs = _SelfAttrs & Record<string, string | undefined>;

export interface Self<T extends ViewModel> {
	[__family: symbol]: T | undefined
	attrs: SelfAttrs,
	slots: Record<string, () => readonly ChildNode[]>,
}

export interface ComponentOptions<T extends ViewModel> extends _ComponentOptions<T> {
	__family?: T | false
}

export class Component<E extends Element, T extends ViewModel> implements _Component<T>, Fragment {
	/** @internal */
	readonly [FRAGMENT] = true;

	/** @internal */
	protected _element: E | undefined;

	/** @internal */
	protected _data: T | undefined;

	constructor(options?: ComponentOptions<T>) {
		const constructor = this.constructor as {
			new(): Component<E, T>
			prototype: Component<E, T>
			readonly render: (self: Self<T>) => E
			readonly model?: {
				new(): T,
				prototype: T
			}
			readonly __family?: symbol
		};

		if (options?.data) {
			this._data = options.data;
		} else if (constructor.model) {
			this._data = new constructor.model();
		} else if (options?.__family) {
			this._data = options.__family;
		}

		const self: Self<T> = {
			attrs: createSelfAttrs(options?.attrs ?? {}),
			slots: options?.slots ?? {}
		};

		if (this._data)
			(self.attrs as any).$e.pipe((this._data as any).$e);

		if (constructor.__family)
			self[constructor.__family] = this._data;

		this._element = constructor.render.call(this._data, self);
	}

	insert(target: ParentNode, previous?: ChildNode | null): void {
		insert(target, previous ?? null, this._element!);
	}

	/** @internal */
	detach(target: ParentNode): void {
		detach(target, this._element!);
	}

	destroy(): void {
		this._data!.dispose?.();
		destroy(this._element!);
	}
}
