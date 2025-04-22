import { RomController } from "@controllers";
import type { ContextMenuItem } from "@directives";
import { NO_IGDB_RESULTS } from "@types";

export function getRomMenuItems(romId: string, igdbId: string): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];

  items.push({
    text: "Edit",
    action: () => RomController.edit(romId),
  });
  items.push({
    text: "Change Cover",
    action: () => RomController.changeCapsule(romId),
  });
  items.push({
    text: "Change Banner",
    action: () => RomController.changeHero(romId),
  });
  items.push({
    text: "Download",
    action: () => RomController.download(romId),
  });
  
  if (igdbId !== "" && igdbId !== NO_IGDB_RESULTS) {
    items.push({
      text: "Refresh Metadata",
      action: () => RomController.refreshMetadata(romId),
    });
  }

  items.push({
    text: "Delete",
    action: () => RomController.delete(romId),
  });

  return items;
}