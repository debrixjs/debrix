import { Reference } from './model';
import { Subscription, SubscriptionListener } from './subscription';

type AccessorValue<T> = {
	readonly [P in keyof T]: T[P] extends Reference<infer U> ? U : T[P]
};

export interface Accessor<T = unknown> {
	readonly value: AccessorValue<T>
	reference(): Reference<AccessorValue<T>> | undefined
	reference<T>(target: T): Reference<T> | undefined
	observe(listener: SubscriptionListener): Subscription
	dispose(): void
}

export interface Binding {
	destroy?(): void;
}

export type Binder<T = unknown, N extends ChildNode = ChildNode> = (node: N, accessor: Accessor<T>) => Binding;
