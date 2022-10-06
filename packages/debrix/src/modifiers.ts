export default class Modifiers<ModifierMap extends object> {
	modifiers = new WeakMap<object, Map<string | symbol, ModifierMap>>();

	set<K extends keyof ModifierMap>(target: object, key: string | symbol, modifier: K, value: ModifierMap[K]) {
		let map: Map<string | symbol, ModifierMap>;

		if (this.modifiers.has(target))
			map = this.modifiers.get(target)!;
		else
			// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
			this.modifiers.set(target, map = new Map());

		if (map.has(key))
			// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
			map.get(key)![modifier] = value as any;
		else
			map.set(key, { [modifier]: value } as ModifierMap);
	}

	get<K extends keyof ModifierMap>(target: object, key: string | symbol, modifier: K): ModifierMap[K] | undefined {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
		return this.modifiers.get(Object.getPrototypeOf(target))?.get(key)?.[modifier];
	}

	has(target: object, key: string | symbol, modifier: keyof ModifierMap): boolean {
		return this.get(target, key, modifier) !== undefined;
	}
}
