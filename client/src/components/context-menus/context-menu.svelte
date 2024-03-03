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
						class="context-menu-item {item.theme ?? 'default'}"
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
		padding: 6px 7px;
		cursor: pointer;
		background-color: transparent;
		color: $mainTextColour;
		text-align: left;
		transition-property: background-color, color;
		transition-duration: 120ms;
		transition-timing-function: ease-in;
		border-radius: $mainBorderRadius;

		&.default:hover {
			background-color: $mainAccentColour;
		}

		&.red {
			color: rgb(231, 55, 55);
			&:hover {
				color: white;
				background-color: rgb(209, 41, 41);
			}
		}
	}
</style>
