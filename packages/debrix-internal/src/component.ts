/* eslint-disable @typescript-eslint/no-dynamic-delete */
import {
	Changes,
	Component as _Component,
	ComponentAttrs,
	ComponentOptions as _ComponentOptions,
	Computed,
	Model,
	ViewModel,
} from 'debrix';
import { destroy, detach, insert } from './document';
import { entries, FRAGMENT, Fragment, hasOwn } from './utils';

function isComputed(value: unknown): value is Computed {
	return value !== null && typeof value === 'object' && 'get' in value;
}

function unwrap<T>(value: T | Computed<T>): T {
	if (isComputed(value)) return value.get();
	else return value;
}

type SelfAttributes = Record<string, string | undefined>;
type SelfSlots = Readonly<Record<string, () => readonly ChildNode[]>>;

class Self extends Model {
	/** @internal */
	[family: symbol]: unknown;

	readonly attrs: SelfAttributes = {};

	constructor(attrs: ComponentAttrs = {}, readonly slots: SelfSlots = {}) {
		super();

		const prev: Record<string, string | undefined> = {};
		let _oldValue: Record<string, string> = {};

		const apply = (
			newValue2?: Record<string, string>,
			oldValue2?: Record<string, string>,
			changes?: Changes
		) => {
			const newValue = newValue2 ?? attrs._!.get();
			const oldValue = oldValue2 ?? _oldValue;
			_oldValue = newValue;

			console.log('changes', changes);

			if (changes) {
				for (const [key] of [...changes.additions, ...changes.modifications]) {
					if (!hasOwn(prev, key)) prev[key] = unwrap(this.attrs[key]);

					this.attrs[key] = newValue[key];
				}

				for (const [key] of changes.deletions) {
					if (hasOwn(prev, key)) {
						this.attrs[key] = prev[key];
						delete prev[key];
					} else {
						delete this.attrs[key];
					}
				}
			} else {
				for (const key of new Set([
					...Object.keys(oldValue),
					...Object.keys(newValue),
				])) {
					const inOld = hasOwn(oldValue, key);
					const inNew = hasOwn(newValue, key);

					if (inOld) {
						if (inNew) {
							this.attrs[key] = newValue[key];
						} else {
							if (hasOwn(prev, key)) {
								this.attrs[key] = prev[key];
								delete prev[key];
							} else {
								delete this.attrs[key];
							}
						}
					} else if (inNew) {
						prev[key] = this.attrs[key];
						this.attrs[key] = newValue[key];
					}
				}
			}
		};

		if (attrs._) {
			apply();
			attrs._.observe(apply);
		}

		for (const [key, value] of entries(attrs)) {
			if (key === '_') continue;

			if (typeof value === 'string') {
				this.attrs[key] = value;
				continue;
			}

			const computed = value as Computed<string>;
			const apply = (newValue?: string) => {
				this.attrs[key] = newValue ?? computed.get();
			};

			apply();
			computed.observe(apply);
		}
	}
}

export interface ComponentOptions<T extends ViewModel>
	extends _ComponentOptions<T> {
	__family?: T | false;
}

export class Component<E extends Element, T extends ViewModel>
	implements _Component<T>, Fragment
{
	/** @internal */
	readonly [FRAGMENT] = true;

	/** @internal */
	protected _element: E | undefined;

	/** @internal */
	protected _data: T | undefined;

	constructor(options?: ComponentOptions<T>) {
		const constructor = this.constructor as {
			new (): Component<E, T>;
			prototype: Component<E, T>;
			readonly render: (self: Self) => E;
			readonly model?: {
				new (): T;
				prototype: T;
			};
			readonly __family?: symbol;
		};

		if (options?.data) {
			this._data = options.data;
		} else if (constructor.model) {
			this._data = new constructor.model();

			// Should check if __family is false or undefined.
			// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
		} else if (options?.__family) {
			this._data = options.__family;
		}

		const self = new Self(options?.attrs, options?.slots);

		if (this._data)
			// eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
			(self as any).$e.pipe((this._data as any).$e);

		if (constructor.__family) self[constructor.__family] = this._data;

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
