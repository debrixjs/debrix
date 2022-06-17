import { Lifecycle } from "./lifecycle";

export interface Accessor<T> {
	get(): T | undefined
	set?(v: T): void
}

export interface Binding extends Lifecycle {
	update?(): void
}

export interface Binder<T, N extends ChildNode> {
	(node: N, value: Accessor<T>): Binding
}
