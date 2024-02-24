<script lang="ts">
</script>

<grid class="domain-page">
	<div class="field chat"></div>
	<div class="field textbox"></div>
	<div class="field chats"></div>
</grid>

<style lang="scss">
	@import "../../../../styles/globalStyles.scss";

	.domain-page {
		display: grid;
		width: 100%;
		height: 100%;
		grid-template-areas:
			"chat chats"
			"textbox chats";
		grid-template-rows: 85% 15%;
		grid-template-columns: 4fr 1fr;
	}

	.field {
		border: 2px solid red;
	}

	.chat {
		grid-area: chat;
	}

	.textbox {
		grid-area: textbox;
	}

	.chats {
		grid-area: chats;
	}
</style>
