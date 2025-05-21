import type { ContextMenuItem } from "@directives";
import { RomService } from "@services";
import { NO_IGDB_RESULTS } from "@types";

export function getRomMenuItems(romId: string, sgdbId: string, igdbId: string): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];

  items.push({
    text: "Edit",
    action: () => RomService.edit(romId),
  });
  if (sgdbId !== "" && sgdbId !== "None") {
    items.push({
      text: "Change Cover",
      action: () => RomService.changeCapsule(romId),
    });
    items.push({
      text: "Change Banner",
      action: () => RomService.changeHero(romId),
    });
  }
  items.push({
    text: "Download",
    action: () => RomService.download(romId),
  });
  
  if (igdbId !== "" && igdbId !== NO_IGDB_RESULTS) {
    items.push({
      text: "Refresh Metadata",
      action: () => RomService.refreshMetadata(romId),
    });
  }

  items.push({
    text: "Delete",
    action: () => RomService.delete(romId),
  });

  return items;
}