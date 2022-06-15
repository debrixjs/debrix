export interface Revokable {
	revoke(): void
}

export interface Lifecycle {
	destroy(): void
}