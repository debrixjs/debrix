import { Lifecycle } from './lifecycle';

export interface ComponentOptions {
	[key: string]: unknown;
	props?: Record<string, unknown>
}

export declare class Component implements Lifecycle {
	constructor(options?: ComponentOptions);
	attach(target: ParentNode, anchor?: Node): void;
	destroy(): void;
}
