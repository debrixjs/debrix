import { Binding } from './component';
import { Reference, Subscription } from './model';
import { Computed } from './viewmodel';

function onUpdate<T>(
	computed: Computed<T>,
	cb: (value: T) => void,
	initial = false
): Subscription {
	if (initial) cb(computed.get());
	return computed.observe(() => cb(computed.get()));
}

export function input(
	node: HTMLInputElement,
	value: Computed<Reference<string>>
): Binding {
	const subscription = onUpdate(value, (ref) => (node.value = ref.get()), true);

	const listener = () => value.get().set(node.value);
	node.addEventListener('input', listener);

	return {
		destroy() {
			node.removeEventListener('input', listener);
			subscription.revoke();
		},
	};
}
