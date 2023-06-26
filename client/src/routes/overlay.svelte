<script lang="ts">
	import { modalState } from "../stores/modalState";

	function deactiveOverlay() {
		(document.getElementById("overlay") as HTMLElement)?.classList.remove("active");
		modalState.set({ component: null });
	}
</script>

<div
	id="overlay"
	class:active={$modalState.component}
	on:click={deactiveOverlay}
	on:keyup={deactiveOverlay}
	role="none"
>
	{#if $modalState.component}
		<svelte:component this={$modalState.component} />
	{/if}
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#overlay {
		width: 100vw;
		height: 100vh;
		position: absolute;
		background-color: transparent;
		transition: background-color calc($modalTransitionTime / 2) ease-out,
			backdrop-filter calc($modalTransitionTime / 2) ease-out;
		z-index: 100;
		pointer-events: none;

		&.active {
			background-color: rgba(0, 0, 0, 0.4);
			backdrop-filter: blur(4px);
			pointer-events: all;
		}
	}
</style>
