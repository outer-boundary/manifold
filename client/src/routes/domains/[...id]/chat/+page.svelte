<script lang="ts">
	import contextMenuStore from "../../../../stores/contextMenuStore";
	import type { ElementEvent } from "../../../../utils/types";

	interface Chat {
		id: string;
		name: string;
		messages: string[];
	}

	interface ChatGroup {
		id: string;
		name: string | null;
		hiddenChats: boolean;
		chats: Chat[];
	}

	let chatGroups: ChatGroup[] = [];
	let selectedChatID = "";
	$: selectedChatMessages =
		chatGroups.flatMap((x) => x.chats).find((x) => x.id === selectedChatID)?.messages ?? [];

	function handleChatContainerContextMenu(e: ElementEvent<HTMLDivElement>) {
		e.preventDefault();
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Create Chat Group",
					onClick: () => {
						chatGroups = [
							...chatGroups,
							{ id: Date.now().toString(), name: "New Chat Group", hiddenChats: false, chats: [] }
						];
					}
				},
				{
					text: "Create Chat",
					onClick: () => {
						const chatID = Date.now().toString();
						const newChat = { id: chatID, name: "New Chat", messages: [] };
						const existingNonGroupedChats = chatGroups.find((x) => x.name === null);
						if (existingNonGroupedChats) {
							existingNonGroupedChats.chats = [...existingNonGroupedChats.chats, newChat];
							chatGroups = [...chatGroups];
						} else {
							chatGroups = [
								...chatGroups,
								{ id: Date.now().toString(), name: null, hiddenChats: false, chats: [newChat] }
							];
						}
						selectedChatID = chatID;
					}
				}
			]
		});
	}

	function handleChatGroupContextMenu(e: ElementEvent<HTMLButtonElement>, id: string) {
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Create New Chat",
					onClick: () => {
						const chatGroup = chatGroups.find((x) => x.id === id)!;
						const chatID = Date.now().toString();
						chatGroup.chats.push({ id: chatID, name: "New Chat", messages: [] });
						chatGroups = [...chatGroups];
						selectedChatID = chatID;
					}
				}
			]
		});
	}

	function toggleChatsVisibility(id: string) {
		const chatGroup = chatGroups.find((x) => x.id === id)!;
		// Only toggle if there are any chats in the group
		if (chatGroup.chats.length > 0) {
			chatGroup.hiddenChats = !chatGroup.hiddenChats;
			chatGroups = [...chatGroups];
		}
	}

	function getSelectedChat() {
		return chatGroups.flatMap((x) => x.chats).find((x) => x.id === selectedChatID);
	}
</script>

<div class="domain-page">
	<div class="left-container">
		<div class="section chat-container-outer">
			<div class="chat-container-inner">
				{#each selectedChatMessages as message}
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
						const selectedChat = getSelectedChat();
						if (selectedChat) {
							selectedChat.messages.push(e.currentTarget.value.trim());
							chatGroups = chatGroups.map((group) => ({
								...group,
								chats: group.chats.map((chat) =>
									chat.id === selectedChatID
										? {
												...chat,
												messages: [...chat.messages]
										  }
										: chat
								)
							}));
							console.log(selectedChatMessages);
							e.currentTarget.value = "";
						}
					}
				}}
			/>
		</div>
	</div>
	<div
		class="section chat-groups-container"
		role="none"
		on:contextmenu={handleChatContainerContextMenu}
	>
		{#each chatGroups as chatGroup}
			<button
				class="chat-group"
				on:click={() => toggleChatsVisibility(chatGroup.id)}
				on:contextmenu|stopPropagation|preventDefault={(e) =>
					handleChatGroupContextMenu(e, chatGroup.id)}
			>
				{#if chatGroup.name !== null}
					<p>{chatGroup.name}</p>
				{/if}
				<div
					class="chats-container"
					class:hidden={chatGroup.hiddenChats}
					role="none"
					on:click|stopPropagation
				>
					{#each chatGroup.chats as chat}
						<button
							class="chat"
							class:selected={selectedChatID === chat.id}
							on:click={() => (selectedChatID = chat.id)}
						>
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

		&.hidden {
			display: none;
		}
	}

	.chat {
		text-align: left;

		&.selected {
			background-color: $mainAccentColour;
		}
	}
</style>
