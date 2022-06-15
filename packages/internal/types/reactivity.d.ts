import type { Binder, Lifecycle, Revokable } from 'debrix';
export interface SubscribableProperty<T> {
    subscribe(cb: (v: T) => void): Revokable;
    get(): T;
    set?(v: T): void;
}
export declare function bind<T, N extends ChildNode>(node: N, binder: Binder<T, N>, { subscribe, get, set }: SubscribableProperty<T>): Lifecycle;
export declare function bind_text(node: Text, { subscribe, get }: SubscribableProperty<string>): Lifecycle;
export declare function bind_attr<N extends Element>(node: N, attr: string, { subscribe, get }: SubscribableProperty<string | undefined>): Lifecycle;