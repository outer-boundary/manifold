<script lang="ts">
	import { goto } from "$app/navigation";
	import sidebarActionsStore from "../../../stores/sidebarActionsStore";

	sidebarActionsStore.set([
		{
			iconName: "material-symbols:chat-rounded",
			text: "Chat",
			onClick: () => {
				if (window.location.pathname.split("/").at(-1) !== "chat") {
					goto(`${window.location.pathname}/chat`);
				}
			}
		}
	]);
</script>

<slot />
