import { Revokable } from './lifecycle';

export default class EventEmitter<EventMap extends { [key in string]: unknown[] }> {
	protected listeners = new Map<keyof EventMap, ((...args: EventMap[any]) => void)[]>();

	on<K extends keyof EventMap>(event: K, listener: (...args: EventMap[K]) => void): Revokable {
		let listeners: ((...args: EventMap[any]) => void)[];
		let index = 0;

		if (this.listeners.has(event))
			index = (listeners = this.listeners.get(event)!).push(listener) - 1;
		else
			this.listeners.set(event, listeners = [listener]);

		return {
			revoke: () => {
				// eslint-disable-next-line @typescript-eslint/no-dynamic-delete
				delete listeners[index];
			}
		};
	}

	trigger<K extends keyof EventMap>(event: K, ...args: EventMap[K]): void {
		if (!this.listeners.has(event))
			return;

		this.listeners.get(event)!.forEach(listener => listener(...args));
	}
}
