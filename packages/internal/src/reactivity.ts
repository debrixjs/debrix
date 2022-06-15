import type { Binder, Lifecycle, Revokable } from 'debrix';

export interface SubscribableProperty<T> {
	subscribe(cb: (v: T) => void): Revokable
	get(): T
	set?(v: T): void
}

export function bind<T, N extends ChildNode>(node: N, binder: Binder<T, N>, { subscribe, get, set }: SubscribableProperty<T>): Lifecycle {
	const binding = binder(node, { get, set });
	// Create a subscription even if the update method doesn't exists (yet).
	// The binding might add the method later.
	const subscription = subscribe(() => binding.update?.());
	return {
		destroy() {
			subscription.revoke();
			binding.destroy?.();
		},
	};
}

export function bind_text(node: Text, { subscribe, get }: SubscribableProperty<string>): Lifecycle {
	const subscription = subscribe(v => node.textContent = v);
	node.textContent = get();
	return {
		destroy() {
			subscription.revoke();
		},
	};
}

export function bind_attr<N extends Element>(node: N, attr: string, { subscribe, get }: SubscribableProperty<string | undefined>): Lifecycle {
	const render = (value: string | undefined) =>
		value === undefined ? node.removeAttribute(attr) : node.setAttribute(attr, value);
	const subscription = subscribe(render);
	render(get());
	return {
		destroy() {
			subscription.revoke();
		},
	};
}