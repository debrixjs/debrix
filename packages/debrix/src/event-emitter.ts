import { Revokable } from './lifecycle';

export interface EventEmitter<EventMap extends { [key in string]: unknown[] }> {
	on<K extends keyof EventMap>(event: K, listener: (...args: EventMap[K]) => void): Revokable
	trigger<K extends keyof EventMap>(event: K, ...args: EventMap[K]): void
}

export function createEventEmitter<EventMap extends { [key in string]: unknown[] }>(): EventEmitter<EventMap> {
	const listeners = new Map<keyof EventMap, Set<(...args: EventMap[any]) => void>>();

	return {
		on(event, listener) {
			if (!listeners.has(event))
				listeners.set(event, new Set());

			const set = listeners.get(event)!.add(listener);

			return {
				revoke: () => {
					set.delete(listener);
				}
			};
		},

		trigger(event, ...args) {
			if (!listeners.has(event))
				return;

			listeners.get(event)!.forEach(listener => listener(...args));
		}
	};
}
