export type NodeLike<N extends Node = Node> = N | Fragment;

const FRAGMENT = Symbol();

export interface FragmentLike {
	insert(target: ParentNode, previous: ChildNode | null): void
	detach(target: ParentNode): void
	destroy(): void
}

export interface Fragment extends FragmentLike {
	[FRAGMENT]: true
}

export function createFragment(fragment: FragmentLike): Fragment {
	return { ...fragment, [FRAGMENT]: true };
}

export function isFragment(value: unknown): value is Fragment {
	return value !== null && typeof value === 'object' && FRAGMENT in value;
}
