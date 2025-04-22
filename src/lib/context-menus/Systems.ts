import { SystemController } from "@controllers";
import type { ContextMenuItem } from "@directives";

export function getSystemMenuItems(abbreviation: string): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];

  items.push({
    text: "Edit",
    action: () => SystemController.edit(abbreviation),
  });
  items.push({
    text: "Change Cover",
    action: () => SystemController.changeCapsule(abbreviation),
  });
  items.push({
    text: "Change Banner",
    action: () => SystemController.changeHero(abbreviation),
  });

  items.push({
    text: "Delete",
    action: () => SystemController.delete(abbreviation),
  });

  return items;
}