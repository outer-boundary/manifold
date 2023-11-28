<script lang="ts">
	import { goto } from "$app/navigation";
	import fetch from "../../utils/fetch";

	async function onSignup(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);

		try {
			const res = await fetch("http://localhost:8080/api/users", {
				method: "POST",
				body: {
					username: formData.get("username"),
					identity: {
						email: formData.get("email"),
						password: formData.get("password")
					}
				}
			});
			if (res.status === 201) {
				// TODO show a message telling the user to verify their account
			}
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	}
</script>

<form class="signupPage" on:submit|preventDefault={onSignup}>
	<label for="username">Username</label>
	<input name="username" type="text" />
	<label for="email">Email</label>
	<input name="email" type="email" />
	<label for="password">Password</label>
	<input name="password" type="password" />
	<button type="submit">Signup</button>
	<button class="hasAccount" on:click={() => goto("/login")}
		>Already have an account? <span class="login">Login</span></button
	>
</form>

<style lang="scss">
	@import "../../styles/globalStyles.scss";
	.signupPage {
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

		.hasAccount {
			margin-top: 6px;
			margin-left: auto;
			background: none;
			color: rgb(85, 85, 85);

			.login {
				color: $mainAccentColour;
				font-weight: 600;
				cursor: pointer;
			}
		}
	}
</style>
