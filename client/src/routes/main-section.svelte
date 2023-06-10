<script lang="ts">
	let asd = 2;
</script>

<div class="mainSection"><p>Yo, I'm the main section.</p></div>

<style lang="scss">
	.mainSection {
		height: 100%;
		width: 100%;
		outline: 2px solid #313131; // $mainBorderWidth $secondaryColour
		border-top-left-radius: 20px; // $mainBorderRadius
		border-bottom-left-radius: 20px; // $mainBorderRadius
		display: flex;
		justify-content: center;
		align-items: center;
	}

	p {
		color: white; // $mainTextColour
	}
</style>
