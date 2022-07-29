import { Accessor, Binding } from './binding';

export function input(node: HTMLInputElement, value: Accessor<string>): Binding {
	const listener = () => value.set?.(node.value);

	if (value.set)
		node.addEventListener('input', listener);

	return {
		update() {
			node.value = value.get();
		},
		destroy() {
			if (value.set)
				node.removeEventListener('input', listener);
		},
	};
}

function entries<T>(object: Record<string, T>): [string, T][] {
	return Object.getOwnPropertyNames(object).map(key => [key, object[key]!]);
}

export function event<N extends HTMLElement>(node: N, value: Accessor<{ [K in keyof HTMLElementEventMap]?: (this: N, ev: HTMLElementEventMap[K]) => void }>): Binding {
	const listeners = new Map(
		entries(value.get()).map(([key, listener]) => {
			// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
			node.addEventListener(key, listener as any);
			return [key, listener as any];
		})
	);

	return {
		update() {
			const events = value.get();

			for (const [name, listener] of listeners) {
				if (events[name as keyof typeof events] !== listener) {
					// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
					node.removeEventListener(name, listener);
					listeners.set(name, events[name as keyof typeof events]);
				}
			}
		},
		destroy() {
			for (const [name, listener] of listeners.entries())
				// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
				node.removeEventListener(name, listener);
		}
	};
}

export function click<N extends HTMLElement>(node: N, value: Accessor<(this: N, ev: MouseEvent) => void>) {
	let listener = value.get()  as EventListener;
	node.addEventListener('click', listener);
	
	return {
		update() {
			node.removeEventListener('click', listener);
			listener = value.get() as EventListener;
			node.addEventListener('click', listener);
		},
		destroy() {
			node.removeEventListener('click', listener);
		}
	};
}
