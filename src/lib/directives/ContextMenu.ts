import type { IconifyIcon } from "@iconify/types";
import { contextMenuItems, contextMenuPosition, showContextMenu } from "@stores/ContextMenu";
import type { Action } from "svelte/action";

export type ActionMenuItem = {
  text: string;
  action: () => void;
}

export type IconMenuItem = {
  icon: IconifyIcon;
  text: string;
  action: () => void;
}

export type SeperatorMenuItem = {
  isSeparator: boolean;
}

export type ContextMenuItem = ActionMenuItem | IconMenuItem | SeperatorMenuItem;

type ContextMenuParams = {
  items: ContextMenuItem[];
  disabled?: boolean;
}

/**
 * A Svelte directive for custom context menus.
 */
export const contextMenu: Action<HTMLElement, ContextMenuParams> = (node: HTMLElement, { items, disabled }) => {
  const config = {
    items: items,
    disabled: disabled ?? false,
  }
  
  async function handleContextMenu(e: MouseEvent) {
    if (config.disabled) return;
    e.preventDefault();

    contextMenuItems.set(config.items);

    contextMenuPosition.set({
      x: e.clientX,
      y: e.clientY,
    });

    showContextMenu.set(true);
  }

  node.addEventListener("contextmenu", handleContextMenu);

  return {
    update({ items, disabled }) {
      config.items = items;
      config.disabled = disabled ?? false;
    },
    destroy() {
      node.removeEventListener("contextmenu", handleContextMenu);
    }
  }
}