import { get, writable } from "svelte/store";

export interface SidebarActions {
	iconName: string;
	text: string;
	onClick: () => any;
}

export const sidebarActions = writable<SidebarActions[]>([]);

export enum Modal {
	CreateDomain,
	JoinDomain
}

export enum ModalState {
	Open,
	Closing,
	Closed
}

export interface ModalStateType {
	name?: Modal;
	state?: ModalState;
}

/**
 * Opens the modal with the specified name and sets the state to open
 */
export function openModal(name: Modal) {
	modalState.set({ name, state: ModalState.Open });
}

/**
 * Sets the modal state to closing then closed after the specified time
 */
export function startClosingModal(closeAfterMs?: number) {
	modalState.set({ name: get(modalState).name, state: ModalState.Closing });
	if (closeAfterMs) {
		setTimeout(() => {
			modalState.set({ state: ModalState.Closed });
		}, closeAfterMs);
	}
}

export function closeModal() {
	modalState.set({ state: ModalState.Closed });
}

export const modalState = writable<ModalStateType>({
	state: ModalState.Closed
});
