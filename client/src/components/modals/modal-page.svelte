<script lang="ts">
	export let style = "";
</script>

<div class="modalPage" {style}>
	<slot />
</div>

<style>
	.modalPage {
		min-width: 100%;
	}
</style>
