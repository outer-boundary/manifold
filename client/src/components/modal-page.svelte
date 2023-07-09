<script lang="ts">
	export let style = "";
</script>

<div class="page" {style}>
	<slot />
</div>

<style>
	.page {
		position: relative;
	}
</style>
