import { writable } from "svelte/store";

export interface SidebarActions {
	iconName: string;
	text: string;
	onClick: () => any;
}

export const sidebarActions = writable<SidebarActions[]>([]);
