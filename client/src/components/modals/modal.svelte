<script lang="ts">
	import Icon from "@iconify/svelte";
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";
	import { cubicOut } from "svelte/easing";
	import { fadeScale } from "../../utils/transitions";
	import { modalState, modalTransitionTime } from "../../stores/modalState";

	// The width needs to be explicitly defined because of how the 'pagination' works
	export let width: number;
	export let style: string = "";

	let modal: HTMLElement;
	let pagesContainer: HTMLElement;
	let pages: NodeListOf<HTMLElement> | null = null;
	let currentPage = 0;

	const paddingX = 80;
	const paddingY = 70;
	const borderWidth = 4;

	onMount(() => {
		// Only get the direct children with a modalPage class
		pages = pagesContainer.querySelectorAll(
			"#pagesContainer > .modalPage"
		) as NodeListOf<HTMLElement>;

		updateModalHeight();
	});

	function updateModalHeight() {
		if (pages && pages?.length > 0) {
			modal.style.height = pages[currentPage].clientHeight + paddingY + borderWidth + "px";
			pagesContainer.style.height = pages[currentPage].clientHeight + "px";
		}
	}

	function changePage(action: "next" | "previous") {
		if (action === "previous") {
			currentPage -= 1;
		} else if (action === "next") {
			currentPage += 1;
		}
		pagesContainer.style.right = currentPage * width - currentPage * borderWidth + "px";
		updateModalHeight();
	}
</script>

<div
	id="modal"
	transition:fadeScale={{ duration: modalTransitionTime }}
	on:click|stopPropagation={() => {}}
	on:keyup|stopPropagation={() => {}}
	role="none"
	style="width:{width}px;{style}"
	bind:this={modal}
>
	<button
		class="close-icon"
		on:click={() => {
			modalState.set({ component: null });
		}}
	>
		<Icon icon="material-symbols:close-rounded" />
	</button>
	{#if pages && pages.length > 0}
		{#if currentPage !== 0}
			<button
				class="arrow-icon left"
				on:click={() => changePage("previous")}
				transition:fade={{ duration: 200, easing: cubicOut }}
			>
				<Icon icon="material-symbols:arrow-left-rounded" />
			</button>
		{/if}
		{#if currentPage !== pages.length - 1}
			<button
				class="arrow-icon right"
				on:click={() => changePage("next")}
				transition:fade={{ duration: 200, easing: cubicOut }}
			>
				<Icon icon="material-symbols:arrow-right-rounded" />
			</button>
		{/if}
	{/if}

	<!-- Minus the padding and border from the width -->
	<div
		id="pagesContainer"
		style="width: {width - paddingX - borderWidth + 'px'};"
		bind:this={pagesContainer}
	>
		<slot />
	</div>
</div>

<style lang="scss">
	@import "../../styles/globalStyles.scss";

	$padding: 40px;

	#modal {
		background-color: $mainElementColour;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		padding: $padding;
		padding-top: 30px;
		position: absolute;
		align-self: center;
		left: 50%;
		transform: translateX(-50%);
		overflow: hidden;
		transition: height 200ms ease-in-out;
		display: flex;
		align-items: center;
		// Because it's using translateX(-50%), I have to move the origin -%50 as well
		transform-origin: 0% 50%;
	}

	#pagesContainer {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: calc($padding * 2);
		position: relative;
		transition: right 200ms ease-in-out;
		right: 0;
	}

	$iconSize: 24px;
	.close-icon {
		width: $iconSize;
		height: $iconSize;
		position: absolute;
		top: 16px;
		right: 16px;
		background-color: transparent;
		color: #555555;
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

		$spacing: 3px;
		&.left {
			left: $spacing;
		}

		&.right {
			right: $spacing;
		}
	}
</style>
