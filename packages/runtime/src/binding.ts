import { Lifecycle } from "./lifecycle";
import { Subscription, SubscriptionListener } from "./subscription";

export interface Accessor<T> {
	get(): T
	set?(v: T): void
	observe(listener: SubscriptionListener): Subscription
}

export interface Binding extends Lifecycle {
	update?(): void
}

export interface Binder<T, N extends ChildNode> {
	(node: N, value: Accessor<T>): Binding
}
