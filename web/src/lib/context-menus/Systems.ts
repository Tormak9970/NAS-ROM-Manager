import type { ContextMenuItem } from "@directives";
import { SystemService } from "@services";

export function getSystemMenuItems(abbreviation: string, sgdbId: string): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];

  items.push({
    text: "Edit",
    action: () => SystemService.edit(abbreviation),
  });
  if (sgdbId !== "" && sgdbId !== "None") {
    items.push({
      text: "Change Cover",
      action: () => SystemService.changeCapsule(abbreviation),
    });
    items.push({
      text: "Change Banner",
      action: () => SystemService.changeHero(abbreviation),
    });
  }

  items.push({
    text: "Delete",
    action: () => SystemService.delete(abbreviation),
  });

  return items;
}