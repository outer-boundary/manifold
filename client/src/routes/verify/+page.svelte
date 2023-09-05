<script lang="ts">
	import { error } from "@sveltejs/kit";
	import { onMount } from "svelte";

	onMount(() => {
		const token = new URLSearchParams(window.location.search).get("token");
		if (!token) {
			throw error(307, "Could not get the token from the URL search parameters");
		}
		fetch("http://localhost:8080/api/auth/verify", {
			method: "POST",
			headers: {
				"Access-Control-Allow-Origin": "http://localhost:5173",
				Origin: "http://localhost:5173"
			},
			body: JSON.stringify({
				token: token
			})
		});
	});
</script>

<div><p style={"color: white"}>Verifying...</p></div>
