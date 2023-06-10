<script lang="ts">
	import Icon from "@iconify/svelte";

	let sidebarOpenState = true;
	const closedWidth = "80px";
	const openWidth = "300px";

	let selectedTab: "domains" | "friends" | "settings" = "domains";

	function toggleHoverEffect(tab: typeof selectedTab) {
		if (selectedTab !== tab) {
			document.getElementById(`${tab}Tab`)?.classList.toggle("hover");
		}
	}
</script>

<div class="sidebar" style={`width: ${sidebarOpenState ? openWidth : closedWidth}`}>
	<div class="titleContainer">
		<Icon class="logo" icon="material-symbols:layers-rounded" />
		<p class="title">Manifold</p>
	</div>
	<div class="titleDivider" />
	<button class="collapseButton" on:click={() => (sidebarOpenState = !sidebarOpenState)}>
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
			on:click={() => (selectedTab = "domains")}
		>
			<Icon class="tabIcon" icon="material-symbols:filter-none-rounded" />
			<p class="title">Domains</p>
		</button>
		<button
			id="friendsTab"
			class="tab {selectedTab === 'friends' && 'selected'}"
			on:mouseenter={() => toggleHoverEffect("friends")}
			on:mouseleave={() => toggleHoverEffect("friends")}
			on:click={() => (selectedTab = "friends")}
		>
			<Icon class="tabIcon" icon="material-symbols:group-rounded" />
			<p class="title">Friends</p>
		</button>
		<button
			id="settingsTab"
			class="tab {selectedTab === 'settings' && 'selected'}"
			on:mouseenter={() => toggleHoverEffect("settings")}
			on:mouseleave={() => toggleHoverEffect("settings")}
			on:click={() => (selectedTab = "settings")}
		>
			<Icon class="tabIcon" icon="material-symbols:settings-rounded" />
			<p class="title">Settings</p>
		</button>
	</div>
	<div class="tabsDivider" />
	<div class="actions">
		<button id="joinDomainAction" class="action">
			<Icon class="actionIcon" icon="material-symbols:search-rounded" />
			<p class="title">Join Domain</p>
		</button>
		<button id="createDomainAction" class="action">
			<Icon class="actionIcon" icon="material-symbols:add-rounded" />
			<p class="title">Create Domain</p>
		</button>
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
	}

	.titleContainer {
		display: flex;
		align-items: center;
		gap: 4px;

		& :global(.logo) {
			width: 52px;
			height: 52px;
			color: $mainAccentColour;
		}

		& .title {
			font-size: 1.4rem;
			color: $mainTextColour;
		}
	}

	.collapseButton {
		width: 40px;
		height: 40px;
		position: absolute;
		top: 10px;
		right: 10px;
		border-radius: 100%;
		background-color: transparent;

		// Can't style the component directly.
		// See: https://iconify.design/docs/icon-components/svelte/color.html
		& > :global(.collapseIcon) {
			color: white;
			width: 16px;
			height: 16px;
			transition: rotate $sidebarTransitionTime ease-in-out;
		}
	}

	.titleDivider {
		width: 80%;
		height: 4px;
		background-color: $secondaryElementColour;
		border-radius: 100px;
		margin-top: 14px;
		margin-bottom: 20px;
	}

	.tabs {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding-left: 26px;
	}

	.tab {
		display: flex;
		align-items: center;
		color: $secondaryTextColour;
		gap: 10px;
		padding: 6px 7px;
		border-top-left-radius: $mainBorderRadius;
		border-bottom-left-radius: $mainBorderRadius;
		background-color: transparent;
		font-size: 1.2rem;
		transition: background-color 120ms ease-in;

		& :global(*) {
			transition: color 120ms ease-in;
		}

		$iconSize: 34px;
		& :global(.tabIcon) {
			width: $iconSize;
			height: $iconSize;
		}

		& .title {
			color: $secondaryTextColour;
		}

		&.selected {
			background-color: $mainAccentColour;
			color: $mainTextColour;

			& .title {
				color: $mainTextColour;
			}
		}

		&.hover {
			background-color: #492683;

			& :global(*),
			.title {
				color: $mainTextColour;
			}
		}
	}

	.tabsDivider {
		width: 70%;
		height: 4px;
		background-color: $secondaryElementColour;
		border-radius: 100px;
		margin: 10px;
	}

	.actions {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding-left: 52px;
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
			width: $iconSize;
			height: $iconSize;
		}

		&:hover {
			background-color: #492683;

			& :global(*) {
				color: $mainTextColour;
			}
		}
	}
</style>
