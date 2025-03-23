import { changeCoverId, downloadProgressRom, loadingModalMessage, romEditingId, showChangeCoverModal, showDownloadProgressModal, showEditRomModal, showLoadingModal } from "@stores/Modals";
import { romMetadata, roms, romsBySystem } from "@stores/State";
import type { IGDBGame } from "@types";
import { get } from "svelte/store";
import { IGDBController } from "./IGDBController";
import { DialogController } from "./utils/DialogController";
import { RestController } from "./utils/RestController";

/**
 * The rom controller.
 */
export class RomController {
  /**
   * Shows the rom edit modal.
   * @param romId The id of the rom.
   */
  static edit(romId: string) {
    romEditingId.set(romId);
    showEditRomModal.set(true);
  }

  /**
   * Toggles whether a rom is favorited.
   * @param romId The id of the rom.
   */
  static toggleFavorite(romId: string) {
    const metadata = get(romMetadata);
    metadata[romId].isFavorite = !metadata[romId].isFavorite;
    romMetadata.set({ ...metadata });
  }

  /**
   * Changes the cover of a rom.
   * @param romId The id of the rom.
   */
  static changeCover(romId: string) {
    changeCoverId.set(romId);
    showChangeCoverModal.set(true);
  }

  /**
   * Downloads a rom.
   * @param romId The id of the rom.
   */
  static download(romId: string) {
    const rom = get(roms)[romId];
    downloadProgressRom.set(rom);
    showDownloadProgressModal.set(true);
  }

  /**
   * Refreshes a rom's metadata.
   * @param igdbId The IGDB id of the rom.
   */
  static async refreshMetadata(igdbId: string): Promise<IGDBGame | null> {
    showLoadingModal.set(true);
    loadingModalMessage.set("Refreshing ROM Metadata...");

    return await IGDBController.getMetadata(igdbId).then((metadata: IGDBGame | null) => {
      loadingModalMessage.set("");
      showLoadingModal.set(false);

      return metadata;
    });
  }

  /**
   * Prompts the user to delete a rom.
   * @param romId The id of the rom.
   */
  static async delete(romId: string) {
    await DialogController.ask(
      "This Can't be Undone!",
      "Are you sure you want to delete this rom?",
      "Yes",
      "No"
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;

      const romsDict = get(roms);
      const rom = romsDict[romId];
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting ROM...");
      const success = await RestController.deleteRom(rom.path);
      loadingModalMessage.set("");
      showLoadingModal.set(false);
      if (!success) return;

      delete romsDict[romId];
      roms.set({ ...romsDict });

      const metadataDict = get(romMetadata);
      delete metadataDict[romId];
      romMetadata.set({ ...metadataDict });

      const systemsMap = get(romsBySystem);
      const system = systemsMap[rom.system];
      const romIndex = system.indexOf(romId);
      system.splice(romIndex, 1);
      romsBySystem.set({ ...systemsMap });
    });
  }
}