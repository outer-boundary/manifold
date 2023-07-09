<script lang="ts">
	import Icon from "@iconify/svelte";
	import { goto } from "$app/navigation";
	import { sidebarActions } from "../stores/sidebarActions";

	function toggleHoverEffect(tab: typeof selectedTab) {
		if (selectedTab !== tab) {
			document.getElementById(`${tab}Tab`)?.classList.toggle("hover");
		}
	}

	function manageSidebarState() {
		sidebarOpenState = !sidebarOpenState;

		const elements = document.querySelectorAll(
			".tabText, .tabs, .title, .actions, .titleContainer, .collapseButton, .titleDivider, .tabsDivider"
		) as NodeListOf<HTMLElement>;
		for (const element of elements) {
			element.classList.toggle("closed");
		}
	}

	let sidebarOpenState = true;
	const closedWidth = "80px";
	const openWidth = "300px";

	let selectedTab: "domains" | "friends" | "settings" = "domains";
</script>

<div
	class="sidebar"
	style={`width: ${sidebarOpenState ? openWidth : closedWidth}; min-width: ${closedWidth}`}
>
	<div class="titleContainer">
		<Icon class="logo" icon="material-symbols:layers-rounded" />
		<p class="title">Manifold</p>
	</div>
	<div class="titleDivider" />
	<button class="collapseButton" on:click={() => manageSidebarState()}>
		<Icon
			class="collapseIcon"
			icon="material-symbols:arrow-back-ios-new-rounded"
			style={`rotate: ${sidebarOpenState ? "0deg" : "180deg"}`}
		/>
	</button>
	<div class="tabs">
		<button
			id="domainsTab"
			class="tab {selectedTab === 'domains' && 'selected'}"
			on:mouseenter={() => toggleHoverEffect("domains")}
			on:mouseleave={() => toggleHoverEffect("domains")}
			on:click={() => {
				selectedTab = "domains";
				goto("/domains");
			}}
		>
			<Icon class="tabIcon" icon="material-symbols:filter-none-rounded" />
			<p class="tabText">Domains</p>
		</button>
		<button
			id="friendsTab"
			class="tab {selectedTab === 'friends' && 'selected'}"
			on:mouseenter={() => toggleHoverEffect("friends")}
			on:mouseleave={() => toggleHoverEffect("friends")}
			on:click={() => {
				selectedTab = "friends";
				goto("/friends");
			}}
		>
			<Icon class="tabIcon" icon="material-symbols:group-rounded" />
			<p class="tabText">Friends</p>
		</button>
		<button
			id="settingsTab"
			class="tab {selectedTab === 'settings' && 'selected'}"
			on:mouseenter={() => toggleHoverEffect("settings")}
			on:mouseleave={() => toggleHoverEffect("settings")}
			on:click={() => {
				selectedTab = "settings";
				goto("/settings");
			}}
		>
			<Icon class="tabIcon" icon="material-symbols:settings-rounded" />
			<p class="tabText">Settings</p>
		</button>
	</div>
	<div class="tabsDivider" />
	<div class="actions">
		{#each $sidebarActions as action}
			<button class="action" on:click={action.onClick}>
				<Icon class="actionIcon" icon={action.iconName} />
				<p class="tabText">{action.text}</p>
			</button>
		{/each}
	</div>
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	$sidebarTransitionTime: 300ms;

	.sidebar {
		outline: $mainBorderWidth solid $secondaryElementColour;
		border-top-right-radius: $mainBorderRadius;
		border-bottom-right-radius: $mainBorderRadius;
		display: flex;
		flex-direction: column;
		align-items: center;
		position: relative;
		transition: width $sidebarTransitionTime ease-in-out;
		padding: 20px 0;
		// overflow: hidden;
	}

	.titleContainer {
		display: flex;
		align-items: center;
		align-self: normal;
		margin-left: 25px;
		gap: 4px;
		transition: margin-left $sidebarTransitionTime ease-in-out;

		&.closed {
			margin-left: 14px;
		}

		$iconSize: 52px;
		& :global(.logo) {
			min-width: $iconSize;
			min-height: $iconSize;
			color: $mainAccentColour;
		}

		& .title {
			font-size: 1.4rem;
			color: $mainTextColour;
			transition: opacity $sidebarTransitionTime ease-in-out;
		}
	}

	.collapseButton {
		width: 26px;
		height: 26px;
		position: absolute;
		top: 33px;
		right: 18px;
		border-radius: 100%;
		background-color: black;
		display: flex;
		align-items: center;
		justify-content: center;
		border: $mainBorderWidth solid $mainElementColour;
		transition: right $sidebarTransitionTime ease-in-out, border $sidebarTransitionTime ease-in-out;

		// Can't style the component directly.
		// See: https://iconify.design/docs/icon-components/svelte/color.html
		& > :global(.collapseIcon) {
			color: $secondaryElementColour;
			filter: brightness(
				1.45
			); // because of the thickness of the icon, it appears much darker than the actual colour
			width: 16px;
			height: 16px;
			transition: rotate $sidebarTransitionTime ease-in-out;
			position: relative;
		}

		&.closed {
			right: -14px;
			border: $mainBorderWidth solid $secondaryElementColour;
		}
	}

	.titleDivider {
		width: 80%;
		height: 4px;
		background-color: $secondaryElementColour;
		border-radius: 100px;
		margin-top: 14px;
		margin-bottom: 20px;
		transition: width $sidebarTransitionTime ease-in-out;

		&.closed {
			width: 65%;
		}
	}

	.tabs {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding-left: 26px;
		transition: padding-left $sidebarTransitionTime ease-in-out;

		&.closed {
			padding-left: 16px;
		}
	}

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

		&.hover {
			background-color: #492683;

			& :global(*),
			.tabText {
				color: $mainTextColour;
			}
		}
	}

	.tabText {
		opacity: 1;
		transition: opacity $sidebarTransitionTime ease-in-out;
		white-space: nowrap;
		overflow: hidden;
	}

	// not sure why but this only works when at the top-level, I can't get it to work with &.closed
	.tabText.closed,
	.title.closed {
		opacity: 0;
	}

	.tabsDivider {
		width: 70%;
		height: 4px;
		background-color: $secondaryElementColour;
		border-radius: 100px;
		margin: 10px;
		transition: width $sidebarTransitionTime ease-in-out;

		&.closed {
			width: 50%;
		}
	}

	.actions {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding-left: 52px;
		transition: padding-left $sidebarTransitionTime ease-in-out;

		&.closed {
			padding-left: 21px;
		}
	}

	.action {
		display: flex;
		align-items: center;
		gap: 4px;
		background-color: transparent;
		font-size: 1rem;
		padding: 4px 5px;
		border-top-left-radius: $mainBorderRadius;
		border-bottom-left-radius: $mainBorderRadius;
		transition: background-color 120ms ease-in;

		& :global(*) {
			color: $secondaryTextColour;
			transition: color 120ms ease-in;
		}

		$iconSize: 32px;
		& :global(.actionIcon) {
			min-width: $iconSize;
			min-height: $iconSize;
		}

		&:hover {
			background-color: #492683;

			& :global(*) {
				color: $mainTextColour;
			}
		}
	}
</style>