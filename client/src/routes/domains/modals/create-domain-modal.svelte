<script>
	import ModalPage from "../../../components/modal-page.svelte";
	import Modal from "../../../components/modal.svelte";

	const pageStyle = "display: flex; flex-direction: column;";
</script>

<Modal width={500}>
	<ModalPage style={pageStyle}>
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
	</ModalPage>
	<ModalPage style={pageStyle}>
		<p class="description">
			How about customizing it a bit further? <br />You can change these later if you just want to
			get started!
		</p>
		<div class="inputContainer">
			<label for="bannerInput">Banner</label>
			<input type="file" name="" id="bannerInput" />
		</div>
		<div class="inputContainer">
			<label for="descriptionTextArea">Description</label>
			<textarea name="" id="descriptionTextArea" cols="20" rows="3" />
		</div>
		<button class="create-domain-button">
			<p>Create Domain!</p>
		</button>
	</ModalPage>
</Modal>

<style lang="scss">
	@import "../../../styles/globalStyles.scss";

	.title {
		font-size: 24px;
		align-self: center;
		margin-bottom: 8px;
	}

	.description {
		font-size: 18px;
		text-align: center;
		margin-bottom: 8px;
	}

	.nameIconContainer {
		display: flex;
		flex-direction: column;
	}

	.inputContainer {
		display: flex;
		flex-direction: column;
	}

	textarea {
		resize: none;
	}

	.create-domain-button {
		width: fit-content;
		padding: 4px 6px;
		border: $mainBorderWidth solid $secondaryElementColour;
		border-radius: $mainBorderRadius;
		background-color: $mainElementColour;
		align-self: center;
		margin-top: 12px;
		cursor: pointer;
		transition: background-color 200ms ease-in-out;

		& > p {
			font-size: 16px;
		}

		&:hover {
			background-color: #202020;
		}
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
