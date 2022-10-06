import type { Accessor, Binder, Lifecycle } from '@debrixjs/debrix';

export function bind<T, N extends ChildNode>(node: N, binder: Binder<T, N>, accessor: Accessor<T>): Lifecycle {
	const binding = binder(node, accessor);
	return {
		destroy() {
			binding.destroy?.();
		},
	};
}

export function bind_text(node: Text, accessor: Accessor<string>): Lifecycle {
	const subscription = accessor.observe(() => node.textContent = accessor.value);
	node.textContent = accessor.value;
	return {
		destroy() {
			subscription.revoke();
		},
	};
}

export function bind_attr<N extends Element>(node: N, attr: string, accessor: Accessor<string | undefined>): Lifecycle {
	const render = () => {
		const value = accessor.value;
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
