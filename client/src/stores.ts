import { writable } from "svelte/store";

export interface SidebarActions {
	iconName: string;
	text: string;
	onClick: () => any;
}

export const sidebarActions = writable<SidebarActions[]>([]);

export enum Modals {
	None,
	CreateDomain,
	JoinDomain
}

export const modalState = writable<{ name: Modals }>({ name: Modals.None });
