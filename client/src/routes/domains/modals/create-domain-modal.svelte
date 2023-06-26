<script>
	import Modal from "../../../components/modal.svelte";
</script>

<Modal>
	<div class="modal">
		<p class="title">Create a Domain</p>
		<p class="description">
			This is where you can create a new domain. Make sure to pick a memorable name!
		</p>
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
		font-size: 20px;
		align-self: center;
		margin-bottom: 8px;
	}

	.description {
		text-align: center;
	}

	p {
		color: $mainTextColour;
	}
</style>
