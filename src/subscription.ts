import { Revokable } from "./lifecycle";

export interface SubscriptionListener {
	(): void
}

export interface Subscription extends Revokable {
}
