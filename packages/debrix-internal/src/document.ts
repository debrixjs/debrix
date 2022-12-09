import { isFragment, NodeLike } from './utils';

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

export function attr(element: Element, name: string, value?: string): void {
	element.setAttribute(name, value ?? '');
}

/**
 * Inserts nodes into element.
 * 
 * @param target The target/parent element.
 * @param previous The node previous to the new nodes.
 * @param nodes The nodes to be inserted.
 */
export function insert(
	target: ParentNode,
	previous: ChildNode | null,
	...nodes: readonly NodeLike<ChildNode>[]
) {
	for (const node of nodes) {
		if (isFragment(node)) {
			node.insert(target, previous);
		} else {
			if (previous) {
				previous.before(node);
			} else {
				target.append(node);
			}
		}
	}
}

/**
 * Detaches, without deleting, nodes from element.
 * 
 * @param target The target/parent element.
 * @param nodes The nodes to be detached.
 */
export function detach(
	target: ParentNode,
	...nodes: readonly NodeLike<ChildNode>[]
) {
	if (!nodes.length)
		return null;

	for (const node of nodes) {
		if (isFragment(node)) {
			node.detach(target);
		} else {
			target.removeChild(node);
		}
	}
}

export function destroy(...nodes: readonly NodeLike<ChildNode>[]) {
	for (const node of nodes) {
		if (isFragment(node)) {
			node.destroy();
		} else {
			node.remove();
		}
	}
}
