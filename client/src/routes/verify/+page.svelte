<script lang="ts">
	import { error } from "@sveltejs/kit";
	import { onMount } from "svelte";
	import fetch from "../../utils/fetch";

	onMount(async () => {
		const token = new URLSearchParams(window.location.search).get("token");
		if (!token) {
			throw error(307, "Could not get the token from the URL search parameters");
		}
		try {
			await fetch("http://localhost:8080/api/auth/verify", {
				method: "POST",
				body: token
			});
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	});
</script>

<div><p style={"color: white"}>Verifying...</p></div>
