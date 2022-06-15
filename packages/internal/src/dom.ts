export function comment(data: string) {
	return document.createComment(data);
}

export function element(tag_name: string, options?: ElementCreationOptions) {
	return document.createElement(tag_name, options)
}

export function text(data: string) {
	return document.createTextNode(data);
}

export function insert(parent: ParentNode, target: ChildNode, anchor?: Node) {
	parent.insertBefore(target, anchor ?? null);
}