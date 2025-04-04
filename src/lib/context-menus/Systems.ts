import type { ContextMenuItem } from "@directives";

export function getSystemMenuItems(systemName: string): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];

  items.push({
    text: "Edit",
    action: () => {}// RomController.edit(romId),
  });
  items.push({
    text: "Change Cover",
    action: () => {}// RomController.changeCover(romId),
  });

  items.push({
    text: "Delete",
    action: () => {}// RomController.delete(romId),
  });

  return items;
}