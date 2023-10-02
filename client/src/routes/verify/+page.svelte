<script lang="ts">
	import { error } from "@sveltejs/kit";
	import { onMount } from "svelte";
	import fetch from "../../utils/fetch";
	import { goto } from "$app/navigation";

	onMount(async () => {
		const token = new URLSearchParams(window.location.search).get("token");
		if (!token) {
			throw error(404, "Could not get the token from the URL search parameters");
		}
		try {
			const res = await fetch("http://localhost:8080/api/auth/verify", {
				method: "POST",
				body: token
			});
			if (res.status === 204) {
				goto("/login");
			}
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	});
</script>

<div><p style={"color: white"}>Verifying...</p></div>
