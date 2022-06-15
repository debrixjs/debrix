function bind(node, binder, { subscribe, get, set }) {
    const binding = binder(node, { get, set });
    // Create a subscription even if the update method doesn't exists (yet).
    // The binding might add the method later.
    const subscription = subscribe(() => { var _a; return (_a = binding.update) === null || _a === void 0 ? void 0 : _a.call(binding); });
    return {
        destroy() {
            var _a;
            subscription.revoke();
            (_a = binding.destroy) === null || _a === void 0 ? void 0 : _a.call(binding);
        },
    };
}
function bind_text(node, { subscribe, get }) {
    const subscription = subscribe(v => node.textContent = v);
    node.textContent = get();
    return {
        destroy() {
            subscription.revoke();
        },
    };
}
function bind_attr(node, attr, { subscribe, get }) {
    const render = (value) => value === undefined ? node.removeAttribute(attr) : node.setAttribute(attr, value);
    const subscription = subscribe(render);
    render(get());
    return {
        destroy() {
            subscription.revoke();
        },
    };
}

export { bind, bind_attr, bind_text };
