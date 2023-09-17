<script lang="ts">
	import Icon from "@iconify/svelte";
	import { goto } from "$app/navigation";
	import { sidebarActions } from "../stores/sidebarActions";
	import type { TabType, TabInfo } from "../types/tabInfo";
	import Tab from "../components/tab.svelte";

	function manageSidebarState() {
		sidebarOpenState = !sidebarOpenState;

		const sidebar = document.getElementById("sidebar")!;
		const elements = sidebar.querySelectorAll(
			".tabText, .tabs, .title, .actions, .titleContainer, .collapseButton, .titleDivider, .tabsDivider"
		) as NodeListOf<HTMLElement>;
		for (const element of elements) {
			element.classList.toggle("closed");
		}
	}

	let sidebarOpenState = true;
	const closedWidth = "80px";
	const openWidth = "300px";

	let selectedTab: TabType = "domains";

	const tabs: TabInfo[] = [
		{
			name: "domains",
			icon: "filter-none-rounded"
		},
		{
			name: "friends",
			icon: "group-rounded"
		},
		{
			name: "settings",
			icon: "settings-rounded"
		}
	];
</script>

<div
	id="sidebar"
	style={`width: ${sidebarOpenState ? openWidth : closedWidth}; min-width: ${closedWidth}`}
>
	<button class="collapseButton" on:click={() => manageSidebarState()}>
		<Icon
			class="collapseIcon"
			icon="material-symbols:arrow-left-rounded"
			style={`rotate: ${sidebarOpenState ? "0deg" : "180deg"}`}
		/>
	</button>
	<div class="sidebarContainer">
		<div class="titleContainer">
			<Icon class="logo" icon="material-symbols:layers-rounded" />
			<p class="title">Manifold</p>
		</div>
		<div class="titleDivider" />
		<div class="tabs">
			{#each tabs as tab}
				<Tab
					id="{tab.name}Tab"
					className="tab {selectedTab === tab.name ? 'selected' : 'hoverable'}"
					onClick={() => {
						selectedTab = tab.name;
						goto(`/${tab.name}`);
					}}
					tabInfo={tab}
				/>
			{/each}
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
		<div class="tabs signoutContainer">
			<Tab
				id="logoutTab"
				className="tab hoverable"
				onClick={() => {
					selectedTab = "logout";
					// call logout endpoint
				}}
				tabInfo={{ name: "logout", icon: "logout-rounded" }}
			/>
		</div>
	</div>
</div>

<style global lang="scss">
	@import "../styles/globalStyles.scss";

	$sidebarTransitionTime: 300ms;

	#sidebar {
		outline: $mainBorderWidth solid $secondaryElementColour;
		border-top-right-radius: $mainBorderRadius;
		border-bottom-right-radius: $mainBorderRadius;
		position: relative;
		transition: width $sidebarTransitionTime ease-in-out;
		padding: 20px 0;

		.collapseButton {
			width: 26px;
			height: 26px;
			position: absolute;
			top: 33px;
			right: 18px;
			z-index: 10;
			border-radius: 100%;
			background-color: black;
			display: flex;
			align-items: center;
			justify-content: center;
			border: $mainBorderWidth solid $mainElementColour;
			transition:
				right $sidebarTransitionTime ease-in-out,
				border $sidebarTransitionTime ease-in-out;

			// Can't style the component directly.
			// See: https://iconify.design/docs/icon-components/svelte/color.html
			> :global(.collapseIcon) {
				color: $secondaryElementColour;
				filter: brightness(
					1.45
				); // because of the thickness of the icon, it appears much darker than the actual colour
				min-width: 30px;
				min-height: 30px;
				transition: rotate $sidebarTransitionTime ease-in-out;
				position: relative;
			}

			&:global(.closed) {
				right: -14px;
				border: $mainBorderWidth solid $secondaryElementColour;
			}
		}
	}

	.sidebarContainer {
		display: flex;
		flex-direction: column;
		align-items: center;
		position: relative;
		overflow: hidden;
		height: 100%;

		.titleContainer {
			display: flex;
			align-items: center;
			align-self: normal;
			margin-left: 25px;
			gap: 4px;
			transition: margin-left $sidebarTransitionTime ease-in-out;

			&:global(.closed) {
				margin-left: 14px;
			}

			$iconSize: 52px;
			:global(.logo) {
				min-width: $iconSize;
				min-height: $iconSize;
				color: $mainAccentColour;
			}

			.title {
				font-size: 1.4rem;
				color: $mainTextColour;
				transition: opacity $sidebarTransitionTime ease-in-out;
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

			&:global(.closed) {
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

			&:global(.closed) {
				padding-left: 16px;
			}
		}

		:global(.tabText) {
			opacity: 1;
			transition:
				opacity $sidebarTransitionTime ease-in-out,
				color 120ms ease-in;
			white-space: nowrap;
			overflow: hidden;
			color: $secondaryTextColour;
		}

		// not sure why but this only works when at the top-level, I can't get it to work with &:global(.closed)
		:global(.tabText.closed),
		:global(.title.closed) {
			opacity: 0;
		}

		.tabsDivider {
			width: 70%;
			height: 4px;
			background-color: $secondaryElementColour;
			border-radius: 100px;
			margin: 10px;
			transition: width $sidebarTransitionTime ease-in-out;

			&:global(.closed) {
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

			&:global(.closed) {
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

			$iconSize: 32px;
			:global(.actionIcon) {
				min-width: $iconSize;
				min-height: $iconSize;
				color: $secondaryTextColour;
				transition: color 120ms ease-in;
			}

			&:hover {
				background-color: #492683;

				:global(*) {
					color: $mainTextColour;
				}
			}
		}

		.signoutContainer {
			margin-top: auto;
		}
	}
</style>
