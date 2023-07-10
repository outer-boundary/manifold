<script lang="ts">
	export let style = "";
</script>

<div class="modalPage" {style}>
	<slot />
</div>

<style>
	.modalPage {
		position: relative;
		min-width: 100%;
		height: fit-content;
	}
</style>
