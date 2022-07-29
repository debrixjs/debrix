export default class Modifiers<ModifierMap extends object> {
	modifiers = new WeakMap<object, Map<string | symbol, ModifierMap>>();

	set<K extends keyof ModifierMap>(target: object, key: string | symbol, modifier: K, value: ModifierMap[K]) {
		let map: Map<string | symbol, ModifierMap>;

		if (this.modifiers.has(target))
			map = this.modifiers.get(target)!;
		else
			this.modifiers.set(target, map = new Map());

		if (map.has(key))
			map.get(key)![modifier] = value as any;
		else
			map.set(key, { [modifier]: value } as ModifierMap);
	}

	get<K extends keyof ModifierMap>(target: object, key: string | symbol, modifier: K): ModifierMap[K] | undefined {
		return this.modifiers.get(Object.getPrototypeOf(target))?.get(key)?.[modifier];
	}

	has(target: object, key: string | symbol, modifier: keyof ModifierMap): boolean {
		return this.get(target, key, modifier) !== undefined;
	}
}
