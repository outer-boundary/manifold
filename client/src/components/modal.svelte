<script lang="ts">
	import { afterUpdate, onDestroy, onMount } from "svelte";
	import { modalState, ModalState, type ModalStateType } from "../stores";

	let curModalState: ModalStateType;
	$: curModalState = { state: ModalState.Closed };
	const unsubscribe = modalState.subscribe((value) => (curModalState = value));
	onDestroy(unsubscribe);

	let active = false;
	onMount(() => {
		setTimeout(() => {
			active = true;
		}, 0);
	});

	$: if (curModalState.state === ModalState.Closing) {
		active = false;
	}
</script>

<div
	id="modal"
	class:active
	on:click|stopPropagation={() => {}}
	on:keyup|stopPropagation={() => {}}
>
	<slot />
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#modal {
		background-color: $mainElementColour;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		padding: 20px;
		position: absolute;
		left: 50%;
		top: 50%;
		translate: -50% -50%;
		opacity: 0;
		transition: opacity $modalTransitionTime ease-in-out, scale $modalTransitionTime ease-in-out;
		scale: 0.25;

		&.active {
			opacity: 1;
			scale: 1;

			animation: openModal $modalTransitionTime ease-out alternate;
		}
	}

	@keyframes openModal {
		0% {
			scale: 0.25;
		}
		60% {
			scale: 1.05;
		}
		70% {
			scale: 1.05;
		}
		100% {
			scale: 1;
		}
	}
</style>
