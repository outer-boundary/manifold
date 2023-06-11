<script lang="ts">
	let asd = 2;
</script>

<div class="mainSection">
	<slot />
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	.mainSection {
		height: 100%;
		width: 100%;
		outline: $mainBorderWidth solid $secondaryElementColour;
		border-top-left-radius: $mainBorderRadius;
		border-bottom-left-radius: $mainBorderRadius;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	p {
		color: white; // $mainTextColour
	}
</style>
