<script lang="ts">
	import Sidebar from "./sidebar.svelte";
	import MainSection from "./main-section.svelte";
	import Overlay from "./overlay.svelte";
	import { afterUpdate, beforeUpdate } from "svelte";

	let showSidebar = true;
	beforeUpdate(() => {
		showSidebar = !["login", "signup", "verify"].includes(
			window.location.pathname.replace(/\//g, "")
		);
	});

	afterUpdate(() => {
		document.getElementById("mainSection")!.style.borderRadius = showSidebar ? "" : "0px";
	});
</script>

<div id="main">
	{#if showSidebar}
		<Sidebar />
	{/if}
	<MainSection>
		<slot />
	</MainSection>
	<Overlay />
</div>

<style lang="scss">
	// Not actually necessary for compilation, it's just to get the IDE to shut up
	@import "../styles/globalStyles.scss";

	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
		border: none;
		font-family: sans-serif;
	}
	#main {
		height: 100vh;
		width: 100vw;
		background-color: $mainElementColour;
		display: flex;
		gap: 20px;
	}
</style>
