import type { Component } from 'debrix';

export function comment(data: string) {
	return document.createComment(data);
}

export function element(tag_name: string, options?: ElementCreationOptions) {
	return document.createElement(tag_name, options);
}

export function text(data: string) {
	return document.createTextNode(data);
}

export function space() {
	return text(' ');
}

export function append(target: ParentNode, nodes: (ChildNode | Component)[], anchor?: ChildNode) {
	for (const node of nodes) {
		if (node instanceof Node)
			target.insertBefore(node, anchor ?? null);
		else
			node.attach(target, anchor);
	}
}

export function attr(element: Element, name: string, value?: string): void {
	element.setAttribute(name, value ?? '');
}
