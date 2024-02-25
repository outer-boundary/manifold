<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import ContextMenu from "../components/context-menus/context-menu.svelte";
	import contextMenuStore from "../stores/contextMenuStore";
	import modalStore from "../stores/modalStore";

	function deactiveOverlay() {
		(document.getElementById("overlay") as HTMLElement)?.classList.remove("active");
		modalStore.close();
		$contextMenuStore = null;
	}

	function listenForEscape(e: KeyboardEvent) {
		if (e.key === "Escape") {
			deactiveOverlay();
		}
	}

	onMount(() => window.addEventListener("keyup", listenForEscape));
	onDestroy(() => window.removeEventListener("keyup", listenForEscape));
</script>

<div
	id="overlay"
	class:active={$modalStore.component || $contextMenuStore}
	class:active-modal={$modalStore.component}
	on:click={deactiveOverlay}
	on:contextmenu|preventDefault
	role="none"
>
	{#if $modalStore.component}
		<svelte:component this={$modalStore.component} />
	{/if}

	{#if $contextMenuStore}
		<ContextMenu />
	{/if}
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#overlay {
		width: 100vw;
		height: 100vh;
		position: absolute;
		background-color: transparent;
		transition:
			background-color calc($modalTransitionTime / 2) ease-out,
			backdrop-filter calc($modalTransitionTime / 2) ease-out;
		z-index: 100;
		pointer-events: none;
		display: flex;

		&.active {
			pointer-events: all;
		}

		&.active-modal {
			background-color: rgba(0, 0, 0, 0.4);
			backdrop-filter: blur(4px);
		}
	}
</style>
