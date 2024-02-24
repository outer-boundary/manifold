<script lang="ts">
	import { goto } from "$app/navigation";
	import sidebarActionsStore from "../../../stores/sidebarActionsStore";

	sidebarActionsStore.set([
		{
			iconName: "material-symbols:chat-rounded",
			text: "Chat",
			onClick: () => goto(`${window.location.pathname}/chat`)
		}
	]);
</script>

<slot />
