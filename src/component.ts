import { Lifecycle } from "./lifecycle";

export interface ComponentOptions {
	[key: string]: unknown;
	props?: Record<string, unknown>
}

export declare class Component implements Lifecycle {
	constructor(options?: ComponentOptions);
	insert(target: ParentNode, anchor?: Node): void;
	destroy(): void;
}
