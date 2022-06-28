import { Accessor } from "./binding";
import { Revokable } from "./lifecycle";

export interface SubscriptionListener {
	(value: unknown): void
}

export interface UnknownSubscriptionListener {
	(): void
}

export interface Subscription extends Revokable {
}

export interface Subscribable {
	$subscribe(target: unknown, listener: SubscriptionListener): Subscription
	$subscribeWhile(inner: () => void, listener: UnknownSubscriptionListener): Subscription
	$accessor<T>(target: T): Accessor<T>
}
