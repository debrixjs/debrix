import { Lifecycle } from "./lifecycle";

export interface ComponentOptions {
	props?: Record<string, unknown>
}

export declare class Component implements Lifecycle {
	constructor(options: ComponentOptions);
	insert(target: ParentNode, anchor?: Node): void;
	destroy(): void;
}
