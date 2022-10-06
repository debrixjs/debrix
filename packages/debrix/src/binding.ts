import { Computed } from './model';

export interface Binding {
	destroy?(): void;
}

export type Binder<T = unknown, N extends ChildNode = ChildNode> = (node: N, value: Computed<T>) => Binding;
