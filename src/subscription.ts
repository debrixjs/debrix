import { Revokable } from './lifecycle';

export type SubscriptionListener = () => void;

export interface Subscription extends Revokable {
}
