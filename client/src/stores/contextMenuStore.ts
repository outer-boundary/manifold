import type { ComponentType } from "svelte";
import { writable } from "svelte/store";

interface ContextMenuBase {
  position: { x: number; y: number };
}

interface ContextMenuItems extends ContextMenuBase {
  items: ContextMenuItem[];
}

interface ContextMenuItem {
  iconName?: string;
  text: string;
  onClick: () => void;
  childActions?: ContextMenuItem[];
}

interface ContextMenuComponent extends ContextMenuBase {
  component: ComponentType
}

const contextMenuStore = writable<ContextMenuItems | ContextMenuComponent | null>(null);

export default contextMenuStore;

export function isContextMenuItem(store: ContextMenuItems | ContextMenuComponent): store is ContextMenuItems {
  return (store as ContextMenuItems).items !== undefined;
}