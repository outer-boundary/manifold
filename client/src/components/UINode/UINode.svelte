<script lang="ts">
	import Icon from "@iconify/svelte";
	import type { UiNode } from "@ory/client";

	export let data: UiNode;
	let nodeType = input;
	let type = () => {
		switch (data.type) {
			case "a":
				return HTMLAnchorElement;
			case "img":
				return HTMLImageElement;
			case "text":
				return HTMLParagraphElement;
			case "input":
				return HTMLInputElement;
			case "script":
				return HTMLScriptElement;
		}
	};
</script>

<svelte:component this={type()} {...data.attributes} />

<style lang="scss">
	@import "../styles/globalStyles.scss";
</style>
