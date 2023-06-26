<script lang="ts">
	import { fadeScale } from "../utils/transitions";
	import { modalState, modalTransitionTime } from "../stores/modalState";
	import Icon from "@iconify/svelte";
</script>

<div
	id="modal"
	transition:fadeScale={{ duration: modalTransitionTime }}
	on:click|stopPropagation={() => {}}
	on:keyup|stopPropagation={() => {}}
	role="none"
>
	<button
		class="close-icon"
		on:click={() => {
			modalState.set({ component: null });
		}}
	>
		<Icon icon="material-symbols:close-rounded" />
	</button>
	<slot />
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#modal {
		background-color: $mainElementColour;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		padding: 16px;
		position: absolute;
		left: 50%;
		top: 50%;
		translate: -50% -50%;
	}

	$iconSize: 24px;
	.close-icon {
		width: $iconSize;
		height: $iconSize;
		position: absolute;
		top: 16px;
		right: 16px;
		background-color: transparent;
		color: $secondaryElementColour;
		transition: filter 100ms ease-out;

		& :global(*) {
			width: $iconSize;
			height: $iconSize;
		}

		&:hover {
			filter: brightness(1.5);
			cursor: pointer;
		}
	}
</style>
