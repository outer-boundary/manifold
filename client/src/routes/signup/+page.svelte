<script lang="ts">
	function onSignup(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);

		fetch("http://localhost:8080/api/users/add", {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				username: formData.get("username"),
				identity: {
					email: formData.get("email"),
					password: formData.get("password")
				}
			})
		});
	}
</script>

<form class="loginPage" on:submit|preventDefault={onSignup}>
	<label for="username">Username</label>
	<input name="username" type="text" />
	<label for="email">Email</label>
	<input name="email" type="email" />
	<label for="password">Password</label>
	<input name="password" type="password" />
	<button type="submit">Signup</button>
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
