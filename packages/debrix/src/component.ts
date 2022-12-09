import { Computed } from './viewmodel';

export interface ComponentOptions {
	[key: string]: unknown;
	props?: Record<string, unknown>
}

export declare class Component {
	constructor(options?: ComponentOptions);
	insert(target: ParentNode, anchor?: Node): void;
	destroy(): void;
}

export interface Binding {
	destroy?(): void;
}

export type Binder<T = unknown, N extends ChildNode = ChildNode> = (node: N, value: Computed<T>) => Binding;
