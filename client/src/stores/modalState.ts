import type { ComponentType } from "svelte";
import { writable } from "svelte/store";

export interface ModalStateType {
	component: ComponentType | null;
}

export const modalState = writable<ModalStateType>({
	component: null
});

export const modalTransitionTime = 200;
