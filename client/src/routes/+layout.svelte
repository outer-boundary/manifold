<script>
	import Sidebar from "./sidebar.svelte";
	import MainSection from "./main-section.svelte";
</script>

<div class="main">
	<Sidebar />
	<MainSection>
		<slot />
	</MainSection>
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
	.main {
		height: 100vh;
		width: 100vw;
		background-color: $mainElementColour;
		display: flex;
		gap: 20px;
	}
</style>
