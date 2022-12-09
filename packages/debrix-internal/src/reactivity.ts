import type { Binder, Computed, Subscription } from 'debrix';
import { insert, detach, destroy } from './document';
import { createFragment, Fragment, NodeLike } from './utils';

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

export function bind_when(
	nodes: NodeLike<ChildNode>[],
	accessor: Computed<boolean>
): Fragment {
	let subscription: Subscription | undefined;
	let attached = false;

	return createFragment({
		insert(target, previous) {

			const rerender = (value: boolean) => {
				if (value) {
					if (!attached) {
						insert(target, previous, ...nodes);
						attached = true;
					}
				} else {
					if (attached) {
						detach(target, ...nodes);
						attached = false;
					}
				}
			};

			rerender(accessor.get());
			subscription = accessor.observe(() => rerender(accessor.get()));
		},

		detach(target) {
			detach(target, ...nodes);
			attached = false;
		},

		destroy() {
			subscription!.revoke();
			destroy(...nodes);
		},
	});
}

export function bind_each<T = unknown>(
	render: (item: T) => readonly ChildNode[],
	accessor: Computed<ArrayLike<T>>,
): Lifecycle {
	let subscription: Subscription | undefined;
	let prevNodes: readonly ChildNode[] = [];

	return createFragment({
		insert(target, previous) {
			const rerender = (array: ArrayLike<T>) => {
				const newNodes: ChildNode[] = [];
		
				const length = array.length;
				for (let i = 0; i < length; i++)
					newNodes.push(...render(array[i]!));
		
				const newLength = newNodes.length;
				const prevLength = prevNodes.length;
				let prevNode = previous;
		
				for (let i = 0; i < prevLength; i++) {
					if (i < newLength)
						target.replaceChild(prevNode = newNodes[i]!, prevNodes[i]!);
					else
						target.removeChild(prevNodes[i]!);
				}
		
				if (newLength > prevLength) {
					const nodes = new Array<ChildNode>(newLength - prevLength);
		
					for (let i = 0; i < newLength; i++)
						nodes[i] = newNodes[i]!;
		
					insert(target, prevNode, ...nodes);
				}
		
				prevNodes = newNodes;
			};
		
			rerender(accessor.get());
			subscription = accessor.observe(() => rerender(accessor.get()));
		},

		detach(target) {
			detach(target, ...prevNodes);
		},

		destroy() {
			subscription!.revoke();
			destroy(...prevNodes);
		},
	});
}
