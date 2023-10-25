<script lang="ts">
	import Icon from "@iconify/svelte";
	import ModalPage from "../../../components/modals/modal-page.svelte";
	import Modal from "../../../components/modals/modal.svelte";
	import fetch from "../../../utils/fetch";

	const pageStyle = "display: flex; flex-direction: column;";

	let files: FileList | undefined = undefined;
	let iconUrl: string | undefined = undefined;
	$: if (files) {
		const fileReader = new FileReader();
		fileReader.readAsDataURL(files[0]);
		fileReader.addEventListener("load", () => {
			iconUrl = fileReader.result?.toString();
			console.log(iconUrl);
		});
	}

	async function createDomain() {
		const displayName = (document.getElementById("nameInput") as HTMLInputElement).value;
		const descriptionText = (document.getElementById("descriptionTextArea") as HTMLInputElement)
			.value;
		try {
			const res = await fetch("http://localhost:8080/api/domains", {
				method: "POST",
				body: {
					display_name: displayName,
					description_text: descriptionText,
					icon_url: "",
					banner_url: "",
					public: false
				}
			});
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	}

	async function getDomains() {
		try {
			const res = await fetch(`http://localhost:8080/api/domains`);
		} catch (err) {
			console.log("Error:", (err as Error).message);
		}
	}
</script>

<Modal width={500}>
	<ModalPage style={pageStyle}>
		<p class="title">Create Domain</p>
		<p class="description">
			This is where you can create a new domain. Make sure to pick a memorable name!
		</p>
		<div class="outerInputContainer">
			<div class="iconContainer inputContainer">
				<label id="iconLabel" for="iconInput">
					<input class="hidden" type="file" name="" id="iconInput" bind:files />
					{#if !files}
						<Icon class="addPhotoIcon" icon="material-symbols:add-a-photo-rounded" />
					{:else}
						<img class="imageIcon" src={iconUrl} alt="Domain icon" />
					{/if}
				</label>
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
		<div class="outerInputContainer">
			<div class="inputContainer">
				<p>Banner</p>
				<label id="bannerLabel" for="bannerInput">
					<input class="hidden" type="file" name="" id="bannerInput" />
					<Icon class="addBannerIcon" icon="material-symbols:image-outline-rounded" />
				</label>
			</div>
			<div class="inputContainer">
				<label for="descriptionTextArea">Description</label>
				<textarea name="" id="descriptionTextArea" cols="20" rows="3" />
			</div>
		</div>
		<button class="create-domain-button" on:click={() => createDomain()}>
			<p>Create Domain</p>
		</button>
	</ModalPage>
</Modal>

<style lang="scss">
	@import "../../../styles/globalStyles.scss";

	.title {
		font-size: 24px;
		align-self: center;
		margin-bottom: 12px;
	}

	.description {
		font-size: 18px;
		text-align: center;
		margin-bottom: 20px;
	}

	.outerInputContainer {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.iconContainer {
		align-items: center;
	}

	.inputContainer {
		display: flex;
		flex-direction: column;
		align-self: center;
		width: 100%;
	}

	$iconSize: 60px;
	#iconLabel {
		width: $iconSize;
		height: $iconSize;
		border: 3px dashed $secondaryElementColour;
		border-radius: 100%;
		position: relative;
		overflow: hidden;

		$iconSize: 22px;
		& > :global(.addPhotoIcon) {
			width: $iconSize;
			height: $iconSize;
			position: absolute;
			top: calc(50% - 2%);
			left: calc(50% + 2.5%);
			transform: translate(-50%, -50%);
		}
	}

	input.hidden {
		visibility: hidden;
	}

	#bannerLabel {
		height: 120px;
		border: 3px dashed $secondaryElementColour;
		position: relative;

		& > :global(.addBannerIcon) {
			width: 30px;
			height: 30px;
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
		}
	}

	.imageIcon {
		position: absolute;
		width: 100%;
		height: 100%;
	}

	textarea {
		resize: none;
	}

	.create-domain-button {
		width: fit-content;
		padding: 8px 12px;
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
