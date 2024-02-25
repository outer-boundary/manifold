<script lang="ts">
	import contextMenuStore from "../../../../stores/contextMenuStore";

	let messages: string[] = [];
	let chatGroups: {
		name: string | null;
		chats: {
			name: string;
			messages: string[];
		}[];
	}[] = [];

	function handleContextMenu(
		e: MouseEvent & {
			currentTarget: EventTarget & HTMLDivElement;
		}
	) {
		e.preventDefault();
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Create Chat Group",
					onClick: () => {
						chatGroups = [...chatGroups, { name: "New Chat Group", chats: [] }];
					}
				},
				{
					text: "Create Chat",
					onClick: () => {
						const newChat = { name: "New Chat", messages: [] };
						const existingNonGroupedChats = chatGroups.find((x) => x.name === null);
						if (existingNonGroupedChats) {
							existingNonGroupedChats.chats = [...existingNonGroupedChats.chats, newChat];
							chatGroups = [...chatGroups];
						} else {
							chatGroups = [...chatGroups, { name: null, chats: [newChat] }];
						}
					}
				}
			]
		});
	}

	function toggleChatsVisibility(
		e: MouseEvent & {
			currentTarget: EventTarget & HTMLButtonElement;
		}
	) {
		const chatsContainer = e.currentTarget.children[
			e.currentTarget.children.length - 1
		] as HTMLElement;
		chatsContainer.classList.toggle("hidden");
	}
</script>

<div class="domain-page">
	<div class="left-container">
		<div class="section chat-container-outer">
			<div class="chat-container-inner">
				{#each messages as message}
					<div class="message-box">
						<p>{message}</p>
					</div>
				{/each}
			</div>
		</div>
		<div class="section textbox-container">
			<textarea
				class="chat-textbox"
				on:keyup={(e) => {
					// currently it'll always have the new line character so we need to check for that
					if (e.key === "Enter" && e.currentTarget.value.length > 1) {
						messages = [...messages, e.currentTarget.value.trim()];
						e.currentTarget.value = "";
					}
				}}
			/>
		</div>
	</div>
	<div class="section chat-groups-container" role="none" on:contextmenu={handleContextMenu}>
		{#each chatGroups as chatGroup}
			<button class="chat-group" on:click={(e) => {}}>
				{#if chatGroup.name !== null}
					<p>{chatGroup.name}</p>
				{/if}
				<div class="chats-container">
					{#each chatGroup.chats as chat}
						<button class="chat">
							<p>{chat.name}</p>
						</button>
					{/each}
				</div>
			</button>
		{/each}
	</div>
</div>

<style lang="scss">
	@import "../../../../styles/globalStyles.scss";

	.domain-page {
		display: flex;
		width: 100%;
		height: 100%;
	}

	.message-box {
		background-color: $secondaryElementColour;
	}

	p {
		color: white;
	}

	button {
		background-color: transparent;
	}

	.section {
		padding: 16px;
	}

	.left-container {
		width: 80%;
		height: 100%;
	}

	.chat-container-outer {
		width: 100%;
		height: 90%;
		display: flex;
		flex-direction: column-reverse;
		overflow: scroll;
	}

	.chat-container-inner {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.message-box {
		padding: 8px;
	}

	.textbox-container {
		height: 10%;
	}

	.chat-textbox {
		width: 100%;
		height: 100%;
		resize: none;
		padding: 8px;
	}

	.chat-groups-container {
		width: 20%;
		border-left: $mainBorderWidth solid $secondaryElementColour;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.chat-group {
		text-align: left;
	}

	.chats-container {
		display: flex;
		flex-direction: column;
		gap: 4px;

		&:hidden {
			display: none;
		}
	}

	.chat {
		text-align: left;
	}
</style>
