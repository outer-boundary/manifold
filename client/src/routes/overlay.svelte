<script lang="ts">
	import { onDestroy } from "svelte";
	import { Modals } from "../stores";
	import { modalState } from "../stores";
	import CreateDomainModal from "./domains/modals/create-domain-modal.svelte";
	import JoinDomainModal from "./domains/modals/join-domain-modal.svelte";

	$: curModalState = { name: Modals.None };
	const unsubscribe = modalState.subscribe((value) => (curModalState = value));
	onDestroy(unsubscribe);

	function deactiveOverlay() {
		(document.getElementById("overlay") as HTMLElement)?.classList.remove("active");
		modalState.set({ name: Modals.None });
	}
</script>

<div
	id="overlay"
	class={curModalState.name !== Modals.None ? "active" : ""}
	on:click={deactiveOverlay}
	on:keyup={deactiveOverlay}
>
	{#if curModalState.name === Modals.CreateDomain}
		<CreateDomainModal />
	{:else if curModalState.name === Modals.JoinDomain}
		<JoinDomainModal />
	{/if}
</div>

<style lang="scss">
	#overlay {
		width: 100vw;
		height: 100vh;
		position: absolute;
		background-color: transparent;
		transition: background-color 250ms ease-in-out, backdrop-filter 250ms ease-in-out;
		z-index: 100;
		pointer-events: none;

		&.active {
			background-color: rgba(0, 0, 0, 0.4);
			backdrop-filter: blur(4px);
			pointer-events: all;
		}
	}
</style>
