import type { Binder, Computed } from 'debrix';

interface Lifecycle {
	destroy(): void
}

export function bind<T, N extends ChildNode>(node: N, binder: Binder<T, N>, accessor: Computed<T>): Lifecycle {
	const binding = binder(node, accessor);
	return {
		destroy() {
			binding.destroy?.();
		},
	};
}

export function bind_text(node: Text, accessor: Computed<string>): Lifecycle {
	const subscription = accessor.observe(() => node.textContent = accessor.get());
	node.textContent = accessor.get();
	return {
		destroy() {
			subscription.revoke();
		},
	};
}

export function bind_attr<N extends Element>(node: N, attr: string, accessor: Computed<string | undefined>): Lifecycle {
	const render = () => {
		const value = accessor.get();
		return value === undefined ? node.removeAttribute(attr) : node.setAttribute(attr, value);
	};
	const subscription = accessor.observe(render);
	render();
	return {
		destroy() {
			subscription.revoke();
		},
	};
}
