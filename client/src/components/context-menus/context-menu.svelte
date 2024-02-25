<script>
	import Icon from "@iconify/svelte";
	import contextMenuStore, { isContextMenuItem } from "../../stores/contextMenuStore";
	import { fadeScale } from "../../utils/transitions";
</script>

{#if $contextMenuStore}
	<div
		class="context-menu"
		style="left: {$contextMenuStore.position.x}px; top: {$contextMenuStore.position.y}px;"
		role="none"
		transition:fadeScale={{ duration: 200 }}
		on:contextmenu|preventDefault
		on:click|stopPropagation
		on:keyup|stopPropagation
	>
		<!-- Check that it's a list of context items and not a component -->
		{#if isContextMenuItem($contextMenuStore)}
			<div class="context-menu-item-container">
				{#each $contextMenuStore.items as item}
					<button
						class="context-menu-item"
						on:click={() => {
							item.onClick();
							contextMenuStore.set(null);
						}}
					>
						{#if item.iconName}
							<Icon class="icon" icon={item.iconName} />
						{/if}
						<p>{item.text}</p>
					</button>
				{/each}
			</div>
		{:else}
			{$contextMenuStore.component}
		{/if}
	</div>
{/if}

<style lang="scss">
	@import "../../styles/globalStyles.scss";

	.context-menu {
		padding: 8px;
		background-color: $mainElementColour;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		position: absolute;
	}

	.context-menu-item-container {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.context-menu-item {
		padding: 4px;
		cursor: pointer;
		background-color: transparent;
		color: $mainTextColour;
		text-align: left;
		transition: background-color 120ms ease-in;
		border-radius: $mainBorderRadius;

		&:hover {
			background-color: $mainAccentColour;
		}
	}
</style>
