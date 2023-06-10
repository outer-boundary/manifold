<script>
	import Sidebar from "./sidebar.svelte";
	import MainSection from "./main-section.svelte";
</script>

<div class="main">
	<Sidebar />
	<MainSection />
</div>

<style lang="scss">
	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
		border: none;
	}
	.main {
		height: 100vh;
		width: 100vw;
		background-color: black; //$mainElementColour
		display: flex;
		gap: 20px;
	}
</style>
