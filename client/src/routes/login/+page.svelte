<script lang="ts">
	import { goto } from "$app/navigation";
	import fetch from "../../utils/fetch";

	async function onLogin(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		try {
			const res = await fetch("http://localhost:8080/api/auth/login", {
				method: "POST",
				body: {
					email: formData.get("identity"),
					password: formData.get("password")
				}
			});
			if (res.status === 204) {
				goto("/");
			}
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	}
</script>

<form class="loginPage" on:submit|preventDefault={onLogin}>
	<label for="identity">Identity</label>
	<input name="identity" type="text" />
	<label for="password">Password</label>
	<input name="password" type="password" />
	<button type="submit">Login</button>
</form>

<style lang="scss">
	@import "../../styles/globalStyles.scss";
	.loginPage {
		display: flex;
		flex-direction: column;
		color: $mainTextColour;
		width: 25%;

		input {
			padding: 6px;
			margin-bottom: 12px;
		}

		button {
			padding: 4px;
		}
	}
</style>
