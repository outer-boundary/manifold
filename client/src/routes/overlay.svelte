<script lang="ts">
	import { onDestroy } from "svelte";
	import { Modal, ModalState, type ModalStateType, startClosingModal } from "../stores";
	import { modalState } from "../stores";
	import CreateDomainModal from "./domains/modals/create-domain-modal.svelte";
	import JoinDomainModal from "./domains/modals/join-domain-modal.svelte";

	export const modalTransitionTime = 200;

	let curModalState: ModalStateType = { state: ModalState.Closed };
	const unsubscribe = modalState.subscribe((value) => (curModalState = value));
	onDestroy(unsubscribe);

	function deactiveOverlay() {
		(document.getElementById("overlay") as HTMLElement)?.classList.remove("active");
		startClosingModal(modalTransitionTime);
	}
</script>

<div
	id="overlay"
	class:active={curModalState.state === ModalState.Open}
	on:click={deactiveOverlay}
	on:keyup={deactiveOverlay}
>
	{#if curModalState.name === Modal.CreateDomain}
		<CreateDomainModal />
	{:else if curModalState.name === Modal.JoinDomain}
		<JoinDomainModal />
	{/if}
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#overlay {
		width: 100vw;
		height: 100vh;
		position: absolute;
		background-color: transparent;
		transition: background-color $modalTransitionTime ease-in-out,
			backdrop-filter $modalTransitionTime ease-in-out;
		z-index: 100;
		pointer-events: none;

		&.active {
			background-color: rgba(0, 0, 0, 0.4);
			backdrop-filter: blur(4px);
			pointer-events: all;
		}
	}
</style>
