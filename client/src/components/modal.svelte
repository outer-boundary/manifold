<script lang="ts">
	import { fadeScale } from "../utils/transitions";
	import { modalTransitionTime } from "../stores/modalState";
</script>

<div
	id="modal"
	transition:fadeScale={{ duration: modalTransitionTime }}
	on:click|stopPropagation={() => {}}
	on:keyup|stopPropagation={() => {}}
	role="none"
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
	}
</style>
