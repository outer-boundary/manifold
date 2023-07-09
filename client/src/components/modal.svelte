<script lang="ts">
	import { fadeScale } from "../utils/transitions";
	import { modalState, modalTransitionTime } from "../stores/modalState";
	import Icon from "@iconify/svelte";
	import { onMount } from "svelte";

	// The width needs to be explicitly defined because of how the 'pagination' works
	export let width: number;
	export let style: string = "";

	let pagesContainer: HTMLElement;
	let pageCount = 0;
	let currentPage = 0;

	onMount(() => {
		const pages = pagesContainer.querySelectorAll(".page") as NodeListOf<HTMLElement>;
		pageCount = pages.length - 1;
		for (let i = 0; i < pages.length; i++) {
			pages[i].style.minWidth = "100%";
		}
	});

	function changePage(action: "next" | "previous") {
		if (action === "previous" && currentPage - 1 >= 0) {
			currentPage -= 1;
		} else if (action === "next" && currentPage + 1 <= pageCount) {
			currentPage += 1;
		}
		pagesContainer.style.left = currentPage * -width + "px";
	}
</script>

<div
	id="modal"
	transition:fadeScale={{ duration: modalTransitionTime }}
	on:click|stopPropagation={() => {}}
	on:keyup|stopPropagation={() => {}}
	role="none"
	style="width:{width}px;{style}"
>
	<button
		class="close-icon"
		on:click={() => {
			modalState.set({ component: null });
		}}
	>
		<Icon icon="material-symbols:close-rounded" />
	</button>
	{#if pageCount > 0}
		{#if currentPage !== 0}
			<button class="arrow-icon left" on:click={() => changePage("previous")}>
				<Icon icon="material-symbols:arrow-left-rounded" />
			</button>
		{/if}
		{#if currentPage !== pageCount}
			<button class="arrow-icon right" on:click={() => changePage("next")}>
				<Icon icon="material-symbols:arrow-right-rounded" />
			</button>
		{/if}
	{/if}

	<!-- Minus the padding and border from the width -->
	<div class="pagesContainer" style="width: {width - 32 - 4 + 'px'};" bind:this={pagesContainer}>
		<slot />
	</div>
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	$padding: 16px;

	#modal {
		background-color: $mainElementColour;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		padding: $padding;
		position: absolute;
		left: 50%;
		top: 50%;
		translate: -50% -50%;
		overflow: hidden;
	}

	.pagesContainer {
		display: flex;
		flex-direction: row;
		gap: calc($padding * 2);
		position: relative;
		transition: left 200ms ease-in-out;
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
		z-index: 100;

		& :global(*) {
			width: $iconSize;
			height: $iconSize;
		}

		&:hover {
			filter: brightness(1.5);
			cursor: pointer;
		}
	}

	.arrow-icon {
		position: absolute;
		bottom: 50%;
		transform: translateY(50%);
		background-color: transparent;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;

		$iconSize: 34px;
		& :global(*) {
			color: $mainTextColour;
			width: $iconSize;
			height: $iconSize;
		}

		$spacing: -6px;
		&.left {
			left: $spacing;
		}

		&.right {
			right: $spacing;
		}
	}
</style>
