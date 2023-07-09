<script>
	import Modal from "../../../components/modal.svelte";
</script>

<Modal>
	<div class="modal">
		<p class="title">Create Domain</p>
		<p class="description">
			This is where you can create a new domain. Make sure to pick a memorable name!
		</p>
		<div class="nameIconContainer">
			<div class="inputContainer">
				<label for="fileInput">Icon</label>
				<input type="file" name="" id="iconInput" />
			</div>
			<div class="inputContainer">
				<label for="nameInput">Name</label>
				<input type="text" id="nameInput" />
			</div>
		</div>
		<div class="inputContainer">
			<label for="bannerInput">Banner</label>
			<input type="file" name="" id="bannerInput" />
		</div>
		<div class="inputContainer">
			<label for="descriptionTextArea">Description</label>
			<textarea name="" id="descriptionTextArea" cols="20" rows="3" />
		</div>
	</div>
</Modal>

<style lang="scss">
	@import "../../../styles/globalStyles.scss";

	.modal {
		max-width: 500px;
		display: flex;
		flex-direction: column;
	}

	.title {
		font-size: 24px;
		align-self: center;
		margin-bottom: 8px;
	}

	.description {
		font-size: 18px;
		text-align: center;
	}

	.nameIconContainer {
		display: flex;
		flex-direction: row;
	}

	.inputContainer {
		display: flex;
		flex-direction: column;
	}

	textarea {
		resize: none;
	}

	p,
	label {
		color: $mainTextColour;
	}

	input:not([type="file"]),
	textarea {
		border: none;
		outline: none;
		padding: 4px;
	}
</style>
