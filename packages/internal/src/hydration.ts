export function claim(parent: Element, index: number): Node {
	return parent.childNodes.item(index);
}

export function split_text(node: Text, offset: number): Text {
	return node.splitText(offset);
}
