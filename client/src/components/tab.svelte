<script lang="ts">
	import Icon from "@iconify/svelte";
	import type { TabType, TabInfo } from "../types/tabInfo";

	export let id: string;
	export let className: string;
	export let tabInfo: TabInfo;
	export let onClick: () => void;
</script>

<button {id} class={className} on:click={onClick}>
	<Icon class="tabIcon" icon={`material-symbols:${tabInfo.icon}`} />
	<p class="tabText">{tabInfo.name.replace(/[A-Za-z]/, (x) => x.toUpperCase())}</p>
</button>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	.tab {
		display: flex;
		align-items: center;
		color: $secondaryTextColour;
		padding: 6px 7px;
		border-top-left-radius: $mainBorderRadius;
		border-bottom-left-radius: $mainBorderRadius;
		background-color: transparent;
		font-size: 1.1rem;
		transition: background-color 120ms ease-in;
		position: relative;

		&.selected {
			background-color: $mainAccentColour;
			color: $mainTextColour;

			& .tabText {
				color: $mainTextColour;
			}
		}

		& :global(*) {
			transition: color 120ms ease-in;
		}

		$iconSize: 34px;
		& :global(.tabIcon) {
			min-width: $iconSize;
			min-height: $iconSize;
		}

		& .tabText {
			color: $secondaryTextColour;
			position: absolute;
			left: calc($iconSize + 14px);
		}

		&.hoverable:hover {
			background-color: #492683;

			& :global(*),
			.tabText {
				color: $mainTextColour;
			}
		}
	}
</style>
