<script lang="ts">
	import { goto } from "$app/navigation";
	import fetch from "../../utils/fetch";

	async function onLogin(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		try {
			// Attempt to log the user in
			const loginRes = await fetch("http://localhost:8080/api/auth/login", {
				method: "POST",
				body: {
					email: formData.get("identity"),
					password: formData.get("password")
				}
			});
			if (loginRes.status === 200) {
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
	<button class="noAccount" on:click={() => goto("/signup")}
		>Don't have an account? <span class="signup">Signup</span></button
	>
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

		.noAccount {
			margin-top: 6px;
			margin-left: auto;
			background: none;
			color: rgb(85, 85, 85);

			.signup {
				color: $mainAccentColour;
				font-weight: 600;
				cursor: pointer;
			}
		}
	}
</style>
