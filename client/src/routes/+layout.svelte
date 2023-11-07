<script lang="ts">
	import Sidebar from "./sidebar.svelte";
	import MainSection from "./main-section.svelte";
	import Overlay from "./overlay.svelte";
	import { afterUpdate, beforeUpdate } from "svelte";
	import { redirect } from "@sveltejs/kit";
  import {Configuration, FrontendApi} from '@ory/client';

	let showSidebar = true;
	beforeUpdate(() => {
		showSidebar = !["login", "signup", "verify"].includes(
			window.location.pathname.replace(/\//g, "")
		);
	});

	afterUpdate(() => {
		document.getElementById("mainSection")!.style.borderRadius = showSidebar ? "" : "0px";
	});

  const basePath = process.env.REACT_APP_ORY_URL || "http://localhost:4000";
  const ory = new FrontendApi(
    new Configuration({
      basePath,
      baseOptions: {
        withCredentials: true,
      },
    }),
  );

  // const authenticated = cookies.get("id");

  // If an unauthenticated user tries to go to any page besides the below, redirect them to the login page
  // if (!authenticated && !["/login", "/signup", "/verify"].some(x => x === url.pathname)) {
  //   throw redirect(307, "/login");
  // }
</script>

<div id="main">
	{#if showSidebar}
		<Sidebar />
	{/if}
	<MainSection>
		<slot />
	</MainSection>
	<Overlay />
</div>

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
