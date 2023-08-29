<script lang="ts">
	import Icon from "@iconify/svelte";

	export let wallpaperUrl: string | null = null;
	export let name: string;
	export let memberCount: number;
	export let cardType: "favourite" | "all";
</script>

<!-- When the user hovers over the card, do the transform skew thing based on their mouse pos -->
<div class="card">
	{#if cardType === "favourite"}
		<Icon class="favouriteIcon" icon="material-symbols:star-rounded" />
	{/if}
	<img class="wallpaper" src={wallpaperUrl} alt="Domain wallpaper" />
	<div class="info">
		<p class="name">{name}</p>
		<p class="memberCount">{memberCount}</p>
	</div>
</div>

<style lang="scss">
	@import "../../styles/globalStyles.scss";

	.card {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		border-radius: $mainBorderRadius;
		border: $mainBorderWidth solid $secondaryElementColour;
		color: $mainTextColour;
		position: relative;
	}

	:global(.favouriteIcon) {
		width: 24px;
		height: 24px;
		position: absolute;
		top: 10px;
		right: 10px;
		color: #dbbc58;
		stroke: black;
		stroke-width: 1.25px;
		stroke-dasharray: 0; // Makes it one solid line rather than dashed
		stroke-linejoin: round;
	}

	.wallpaper {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.info {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 14px;
	}
</style>
