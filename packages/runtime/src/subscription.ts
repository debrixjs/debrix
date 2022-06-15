import { Accessor } from './binding';
import { Revokable } from './lifecycle';

export interface SubscriptionListener {
	(value: unknown): void
}

export interface Subscription extends Revokable {
}

export interface Subscribable {
	$subscribe(target: unknown, listener: SubscriptionListener): Subscription
	$resubscribe(listener: SubscriptionListener): () => Subscription
	$reference<T>(target: T): Accessor<T>
}