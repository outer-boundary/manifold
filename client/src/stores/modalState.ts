import type { ComponentType } from "svelte";
import { writable } from "svelte/store";

export const modalTransitionTime = 200;
export interface ModalStateType {
	component: ComponentType | null;
}

function createModalStore() {
	const store = writable<ModalStateType>({
		component: null
	});

	function open(component: ModalStateType["component"]) {
		store.set({ component });
	}

	function close() {
		store.set({ component: null });
	}

	return {
		subscribe: store.subscribe,
		update: store.update,
		open,
		close
	}
}

const modalStore = createModalStore();

export default modalStore;
