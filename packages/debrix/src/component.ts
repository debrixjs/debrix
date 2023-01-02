import { Computed, ViewModel } from './viewmodel';

export type ComponentAttrs = Record<string, string | Computed<string>> & {
	_?: Computed<Record<string, string>>
};

export interface ComponentOptions<T extends ViewModel> {
	[key: string]: unknown;
	data?: T
	props?: Record<string, unknown>
	slots?: Record<string, () => ChildNode[]>
	attrs?: ComponentAttrs
}

export declare class Component<T extends ViewModel> {
	constructor(options?: ComponentOptions<T>);
	insert(target: ParentNode, previous?: ChildNode): void;
	destroy(): void;
}

export interface Binding {
	destroy?(): void;
}

export type Binder<T = unknown, N extends ChildNode = ChildNode> = (node: N, value: Computed<T>) => Binding;
