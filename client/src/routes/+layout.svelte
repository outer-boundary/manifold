<script lang="ts">
	import Sidebar from "./sidebar.svelte";
	import MainSection from "./main-section.svelte";
	import Overlay from "./overlay.svelte";
	import { afterUpdate, beforeUpdate } from "svelte";
	import { Configuration, FrontendApi, type Session } from "@ory/client";
	import { goto } from "$app/navigation";

	const oryKratosUrl = "http://localhost:4433";
	const ory = new FrontendApi(
		new Configuration({
			basePath: oryKratosUrl
			// baseOptions: {
			// 	withCredentials: true
			// }
		})
	);

	const nonAuthenticatedRoutes = ["/", "/login", "/signup", "/verify"];
	const routeRequiresAuthentication = (currentRoute: string) => {
		const scopedRoutes = nonAuthenticatedRoutes
			.filter((route) => route !== "/" && route.endsWith("*"))
			.map((route) => route.substring(0, route.length - 2))
			.filter((route) => route === "");
		const exactRoutes = nonAuthenticatedRoutes.filter((route) => !scopedRoutes.includes(route));

		return (
			scopedRoutes.filter((route) => currentRoute.startsWith(route)).length === 0 &&
			exactRoutes.filter((route) => currentRoute === route).length === 0
		);
	};

	let shouldRender = false;

	let showSidebar = true;
	let sessionData: Session | null = null;

	beforeUpdate(async () => {
		let isAuthRequired = routeRequiresAuthentication(window.location.pathname);

		try {
			sessionData = (await ory.toSession()).data;
			shouldRender = true;
		} catch {
			if (isAuthRequired) {
				await goto("/login");
			} else {
				shouldRender = true;
			}
		}

		showSidebar = isAuthRequired;
	});

	afterUpdate(() => {
		const mainSection = document.getElementById("mainSection");
		if (mainSection) {
			mainSection.style.borderRadius = showSidebar ? "" : "0px";
		}
	});
</script>

{#if shouldRender}
	<div id="main">
		{#if showSidebar}
			<Sidebar />
		{/if}
		<MainSection>
			<slot />
		</MainSection>
		<Overlay />
	</div>
{/if}

<style lang="scss">
	// Not actually necessary for compilation, it's just to get the IDE to shut up
	@import "../styles/globalStyles.scss";

	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
		border: none;
		font-family: sans-serif;
	}
	#main {
		height: 100vh;
		width: 100vw;
		background-color: $mainElementColour;
		display: flex;
		gap: 20px;
	}
</style>
