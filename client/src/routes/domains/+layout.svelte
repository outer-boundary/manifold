<script lang="ts">
	import { onMount } from "svelte";
	import sidebarActionsStore from "../../stores/sidebarActionsStore";
	import modalStore from "../../stores/modalStore";
	import JoinDomainModal from "../../components/modals/domains/join-domain-modal.svelte";
	import CreateDomainModal from "../../components/modals/domains/create-domain-modal.svelte";

	sidebarActionsStore.set([
		{
			iconName: "material-symbols:search-rounded",
			text: "Join Domain",
			onClick: () => {
				modalStore.open(JoinDomainModal);
			}
		},
		{
			iconName: "material-symbols:add-rounded",
			text: "Create Domain",
			onClick: () => {
				modalStore.open(CreateDomainModal);
			}
		}
	]);
</script>

<slot />
