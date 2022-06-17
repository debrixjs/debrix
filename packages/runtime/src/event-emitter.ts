import { Revokable } from './lifecycle';

export default class EventEmitter<EventMap extends { [key in string]: unknown[] }> {
	protected listeners = new Map<keyof EventMap, ((...args: EventMap[any]) => void)[]>();

	on<K extends keyof EventMap>(event: K, listener: (...args: EventMap[K]) => void): Revokable {
		let index = 0;

		if (this.listeners.has(event))
			index = this.listeners.get(event)!.push(listener) - 1;
		else
			this.listeners.set(event, [listener]);

		return {
			revoke: () => {
				this.listeners.get(event)!.splice(index, 0);
			}
		};
	}

	trigger<K extends keyof EventMap>(event: K, ...args: EventMap[K]): void {
		if (!this.listeners.has(event))
			return;

		for (const listener of this.listeners.get(event)!)
			listener(...args);
	}
}
