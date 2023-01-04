import { Computed } from 'debrix';

export type Not<T extends boolean> = T extends true
	? false
	: T extends false
	? true
	: boolean;

export function computedNot<T extends boolean>(
	computed: Computed<T>
): Computed<Not<T>> {
	return {
		get: () => !computed.get() as Not<T>,
		observe: (listener) =>
			computed.observe((newValue, oldValue) =>
				// eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
				listener(!newValue as Not<T>, !oldValue as Not<T>)
			),
		dispose: () => {},
	};
}

export type NodeLike<N extends Node = Node> = N | Fragment;

export const FRAGMENT = Symbol();

export interface FragmentLike {
	insert(target: ParentNode, previous: ChildNode | null): void;
	detach(target: ParentNode): void;
	destroy(): void;
}

export interface Fragment extends FragmentLike {
	[FRAGMENT]: true;
}

export function createFragment(fragment: FragmentLike): Fragment {
	return { ...fragment, [FRAGMENT]: true };
}

export function isFragment(value: unknown): value is Fragment {
	return value !== null && typeof value === 'object' && FRAGMENT in value;
}

export type FlatArray<Arr, Depth extends number> = {
	done: Arr;
	recur: Arr extends readonly (infer InnerArr)[]
		? FlatArray<
				InnerArr,
				[
					-1,
					0,
					1,
					2,
					3,
					4,
					5,
					6,
					7,
					8,
					9,
					10,
					11,
					12,
					13,
					14,
					15,
					16,
					17,
					18,
					19,
					20
				][Depth]
		  >
		: Arr;
}[Depth extends -1 ? 'done' : 'recur'];

function flatten(array: readonly unknown[], depth: number): unknown {
	if (depth < 1) {
		return array.slice();
	}

	return array.reduce(function (acc: unknown[], val) {
		return acc.concat(Array.isArray(val) ? flatten(val, depth - 1) : val);
	}, []);
}

export function flat<A extends readonly unknown[], D extends number = 1>(
	array: A,
	depth: D = 1 as D
): FlatArray<A, D>[] {
	return flatten(array, depth) as FlatArray<A, D>[];
}

export function hasOwn<T extends object, K extends string | symbol>(
	object: T,
	property: K
): boolean {
	return Object.prototype.hasOwnProperty.call(object, property);
}

export function entries<T>(object: Record<string, T>): [string, T][] {
	const keys = Object.keys(object);
	let i = keys.length;
	const entries = new Array(i) as [string, T][];

	while (i--) entries[i] = [keys[i]!, object[keys[i]!]!];

	return entries;
}
