<script lang="ts">
</script>

<div id="mainSection">
	<slot />
</div>

<style lang="scss">
	@import "../styles/globalStyles.scss";

	#mainSection {
		height: 100%;
		width: 100%;
		outline: $mainBorderWidth solid $secondaryElementColour;
		border-top-left-radius: $mainBorderRadius;
		border-bottom-left-radius: $mainBorderRadius;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
