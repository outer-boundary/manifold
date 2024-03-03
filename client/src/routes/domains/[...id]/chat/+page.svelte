<script lang="ts">
	import Icon from "@iconify/svelte";
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

	let chatGroupCount = 0;
	let chatCount = 0;

	let dragInfo: {
		draggedItemInfo: { chatGroupIndex: number; chatIndex: number };
		newPosition: { chatGroupIndex: number; chatIndex: number };
	} = {
		draggedItemInfo: { chatGroupIndex: -1, chatIndex: -1 },
		newPosition: { chatGroupIndex: -1, chatIndex: -1 }
	};

	function sendMessage(e: ElementEvent<KeyboardEvent, HTMLTextAreaElement>) {
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
				e.currentTarget.value = "";
			}
		}
	}

	function sortChatGroups(chatGroups: ChatGroup[]) {
		const index = chatGroups.findIndex((x) => x.name === null);

		if (index >= 0) {
			const [item] = chatGroups.splice(index, 1);
			chatGroups.unshift(item);
		}

		return chatGroups;
	}

	function handleChatContainerContextMenu(e: ElementEvent<MouseEvent, HTMLDivElement>) {
		e.preventDefault();
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Create Chat Group",
					onClick: () => {
						chatGroups = [
							...chatGroups,
							{
								id: Date.now().toString(),
								name: "New Chat Group " + ++chatGroupCount,
								hiddenChats: false,
								chats: []
							}
						];
					}
				},
				{
					text: "Create Chat",
					onClick: () => {
						const chatID = Date.now().toString();
						const newChat = { id: chatID, name: "New Chat " + ++chatCount, messages: [] };
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

	function handleChatGroupContextMenu(e: ElementEvent<MouseEvent, HTMLDivElement>, id: string) {
		const chatGroupIndex = chatGroups.findIndex((x) => x.id === id)!;
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Create New Chat",
					onClick: () => {
						const chatID = Date.now().toString();
						chatGroups[chatGroupIndex].chats.push({
							id: chatID,
							name: "New Chat " + ++chatCount,
							messages: []
						});
						chatGroups = [...chatGroups];
						selectedChatID = chatID;
					}
				},
				{
					text: "Delete Chat Group",
					onClick: () => {
						chatGroups = chatGroups.filter((x) => x.id === id);
					},
					theme: "red"
				}
			]
		});
	}

	function handleChatContextMenu(e: ElementEvent<MouseEvent, HTMLButtonElement>, id: string) {
		contextMenuStore.set({
			position: { x: e.x, y: e.y },
			items: [
				{
					text: "Delete Chat",
					theme: "red",
					onClick: () => {
						// delete chat group (gonna wait till i do the actual implementation cause it'll be easier anyway)
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

	function onDragStart(chatGroupIndex: number, chatIndex: number) {
		dragInfo.draggedItemInfo = { chatGroupIndex, chatIndex };
	}

	function onDragOver(
		e: ElementEvent<DragEvent, HTMLElement>,
		isOverChatGroup: boolean,
		chatGroupIndex: number,
		chatIndex: number
	) {
		const targetRect = e.currentTarget.getBoundingClientRect();
		let pos: "top" | "bottom" = "top";
		// If the user is mousing nearing the bottom half of the element
		if (e.pageY >= targetRect.top + targetRect.height / 2) {
			pos = "bottom";
		}

		// Add half the container's gap value so that the indicator is in the same spot regardless of if it's from the bottom or top of an element
		// TODO need to change the drag indicator to be relative to the component as it won't adjust its position/size if the window size changes
		const offset = 1 * (pos === "top" ? -1 : 1);
		const style = `left: ${targetRect.left}px; top: ${
			(pos === "top" ? targetRect.top : targetRect.bottom) + offset
		}px; width: ${targetRect.width}px; display: unset`;
		document.getElementById("drag-indicator")!.style.cssText = style;

		// If the user is dragging over a chat then get the new index based on where their cursor is
		// Also need to account for the dragged element's index since it needs to essentially be ignored while it's being dragged
		if (!isOverChatGroup) {
			const newIndex = pos === "top" ? chatIndex : chatIndex + 1;
			const shouldIgnoreCurIndex =
				dragInfo.draggedItemInfo.chatIndex < newIndex &&
				dragInfo.draggedItemInfo.chatGroupIndex === chatGroupIndex;
			dragInfo.newPosition = {
				chatGroupIndex,
				chatIndex: shouldIgnoreCurIndex ? newIndex - 1 : newIndex
			};
		} else {
			// If hovering over the top of a chat group, place it at the end of the previous group.
			// This is always valid as the ungrouped chat group cannot be dragged over.
			// Otherwise, add it to the start of the chat group being dragged over
			if (pos === "top") {
				const newChatGroupIndex = chatGroupIndex - 1;
				dragInfo.newPosition = {
					chatGroupIndex: newChatGroupIndex,
					chatIndex: chatGroups[newChatGroupIndex].chats.length
				};
			} else {
				dragInfo.newPosition = {
					chatGroupIndex,
					chatIndex: 0
				};
			}
		}
	}

	function onDragEnd() {
		const curChatGroup = chatGroups[dragInfo.draggedItemInfo.chatGroupIndex];
		const [item] = curChatGroup.chats.splice(dragInfo.draggedItemInfo.chatIndex, 1);

		const newChatGroup = chatGroups[dragInfo.newPosition.chatGroupIndex];
		newChatGroup.chats.splice(dragInfo.newPosition.chatIndex, 0, item);
		// Make sure the the chat group the element is being added to is opened up
		newChatGroup.hiddenChats = false;

		chatGroups = [...chatGroups];

		document.getElementById("drag-indicator")!.style.cssText = "";
	}

	function getChatGroup(id: string, type: "chatGroup" | "chat") {
		return chatGroups.find((x) => x.id === id);
	}

	function getChat(id: string) {
		return chatGroups.flatMap((x) => x.chats).find((x) => x.id === id);
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
			<textarea class="chat-textbox" on:keyup={(e) => sendMessage(e)} />
		</div>
	</div>
	<div
		class="section chat-groups-container"
		role="none"
		on:contextmenu={handleChatContainerContextMenu}
	>
		{#each sortChatGroups(chatGroups) as chatGroup, chatGroupIndex}
			<div
				class="chat-group"
				class:padding={chatGroup.name !== null}
				role="none"
				on:click={() => toggleChatsVisibility(chatGroup.id)}
				on:contextmenu|stopPropagation|preventDefault={(e) =>
					handleChatGroupContextMenu(e, chatGroup.id)}
			>
				<button
					class="name-container"
					on:dragstart={() => {
						onDragStart(chatGroupIndex, -1);
					}}
					on:dragover={(e) => onDragOver(e, true, chatGroupIndex, 0)}
					draggable={true}
				>
					{#if chatGroup.name !== null}
						<p>{chatGroup.name}</p>
						<div class="drop-down-icon-container" class:closed={chatGroup.hiddenChats}>
							<Icon class="drop-down-icon" icon="material-symbols:arrow-drop-down-rounded" />
						</div>
					{/if}
				</button>

				<div
					class="chats-container"
					class:hidden={chatGroup.hiddenChats}
					class:margin={chatGroup.name !== null}
					role="none"
					on:click|stopPropagation
					on:contextmenu|stopPropagation
				>
					{#each chatGroup.chats as chat, chatIndex}
						<button
							class="chat"
							class:selected={selectedChatID === chat.id}
							on:click={() => (selectedChatID = chat.id)}
							on:dragstart={() => onDragStart(chatGroupIndex, chatIndex)}
							on:dragover={(e) => onDragOver(e, false, chatGroupIndex, chatIndex)}
							on:dragend={() => onDragEnd()}
							on:contextmenu|preventDefault|stopPropagation={(e) =>
								handleChatContextMenu(e, chat.id)}
							draggable={true}
						>
							<p>{chat.name}</p>
						</button>
					{/each}
				</div>
			</div>
		{/each}

		<div id="drag-indicator" />
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
		gap: 4px;
	}

	.chat-group {
		text-align: left;

		&.padding {
			padding: 4px;
		}

		& :global(.drop-down-icon) {
			width: 20px;
			height: 20px;
			color: white;
		}
	}

	.drop-down-icon-container {
		display: flex;
		align-items: center;
		justify-content: center;

		&.closed {
			rotate: 90deg;
		}
	}

	.name-container {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.chats-container {
		display: flex;
		flex-direction: column;
		gap: 2px;

		&.hidden {
			display: none;
		}

		&.margin {
			margin-left: 4px;
			margin-top: 2px;
		}
	}

	.chat {
		padding: 4px;
		text-align: left;

		&.selected {
			background-color: $mainAccentColour;
		}
	}

	#drag-indicator {
		height: 4px;
		background-color: $mainAccentColour;
		position: absolute;
		outline: $mainElementColour solid $mainBorderWidth;
		display: none;
	}
</style>
