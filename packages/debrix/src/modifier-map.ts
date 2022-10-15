export interface ModifierMap<M extends object> {
	set<K extends keyof M>(target: object, key: string | symbol, modifier: K, value: M[K]): void
	get<K extends keyof M>(target: object, key: string | symbol, modifier: K): M[K] | undefined
}

export function createModifierMap<M extends object>(): ModifierMap<M> {
	const modifiers = new WeakMap<object, Map<string | symbol, M>>();

	return {
		set(target, key, modifier, value) {
			let map: Map<string | symbol, M>;

			if (modifiers.has(target))
				map = modifiers.get(target)!;
			else
				modifiers.set(target, map = new Map<string | symbol, M>());

			if (map.has(key))
				map.get(key)![modifier] = value;
			else
				map.set(key, { [modifier]: value } as M);
		},

		get(target, key, modifier) {
			// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
			return modifiers.get(Object.getPrototypeOf(target))?.get(key)?.[modifier];
		}
	};
}
