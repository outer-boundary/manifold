<script lang="ts">
	import modalStore from "../stores/modalStore";

	function deactiveOverlay() {
		(document.getElementById("overlay") as HTMLElement)?.classList.remove("active");
		modalStore.close();
	}
</script>

<div
	id="overlay"
	class:active={$modalStore.component}
	on:click={deactiveOverlay}
	on:keyup={deactiveOverlay}
	role="none"
>
	{#if $modalStore.component}
		<svelte:component this={$modalStore.component} />
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
			background-color: rgba(0, 0, 0, 0.4);
			backdrop-filter: blur(4px);
			pointer-events: all;
		}
	}
</style>
