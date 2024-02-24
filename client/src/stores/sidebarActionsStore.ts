import { writable } from "svelte/store";

export interface SidebarAction {
	iconName: string;
	text: string;
	onClick: () => any;
}

function createModalStore() {
	const store = writable<SidebarAction[]>([]);

	function add(actions: SidebarAction[]) {
		store.update((val) => [...val, ...actions]);
	}

	return {
		subscribe: store.subscribe,
		set: store.set,
		add,
	}
}

const sidebarActionsStore = createModalStore();

export default sidebarActionsStore;