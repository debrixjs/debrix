import { Revokable } from './lifecycle';

export default class EventEmitter<EventMap extends { [key in string]: unknown[] }> {
	protected listeners = new Map<keyof EventMap, ((...args: EventMap[any]) => void)[]>();

	on<K extends keyof EventMap>(event: K, listener: (...args: EventMap[K]) => void): Revokable {
		let listeners: ((...args: EventMap[any]) => void)[];

		if (this.listeners.has(event))
			(listeners = this.listeners.get(event)!).push(listener) - 1;
		else
			this.listeners.set(event, listeners = [listener]);

		return {
			revoke: () => {
				listeners.splice(listeners.indexOf(listener), 1);
			}
		};
	}

	trigger<K extends keyof EventMap>(event: K, ...args: EventMap[K]): void {
		if (!this.listeners.has(event))
			return;

		this.listeners.get(event)!.forEach(listener => listener(...args));
	}
}
