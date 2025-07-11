import { changeGridsId, changeGridsOnSelect, changeGridsSearchId, changeGridsType, downloadProgressInit, loadingModalMessage, replaceRomId, replaceRomSystem, romEditingId, showChangeGridsModal, showDownloadProgressModal, showEditRomModal, showLoadingModal, showReplaceRomModal } from "@stores/Modals";
import { romMetadata, roms, romsBySystem } from "@stores/State";
import type { IGDBGame } from "@types";
import { get } from "svelte/store";
import { IGDBService } from "./IGDBService";
import { DialogService } from "./utils/DialogService";
import { RestService } from "./utils/RestService";

/**
 * The ROM Service.
 */
export class RomService {
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
   * Changes the capsule of a rom.
   * @param romId The id of the rom.
   */
  static changeCapsule(romId: string) {
    let searchId = get(romMetadata)[romId].sgdbId;
    changeGridsSearchId.set(searchId);
    changeGridsType.set("Capsule");
    changeGridsOnSelect.set(async (fullCapsule?: string, thumbCapsule?: string) => {
      const metadataDict = get(romMetadata);

      const [fullCached, thumbCached] = await RestService.cacheCapsule(
        fullCapsule!,
        thumbCapsule!,
        romId.replace(/[/\\?%*:|"<> ]/g, '_')
      )

      metadataDict[romId].fullCapsulePath = fullCached!;
      metadataDict[romId].thumbCapsulePath = thumbCached!;

      romMetadata.set({ ...metadataDict });
    });
    changeGridsId.set(romId);
    showChangeGridsModal.set(true);
  }

  /**
   * Changes the capsule of a rom.
   * @param romId The id of the rom.
   */
  static changeHero(romId: string) {
    let searchId = get(romMetadata)[romId].sgdbId;
    changeGridsSearchId.set(searchId);
    changeGridsType.set("Hero");
    changeGridsOnSelect.set(async (fullCapsule?: string, thumbCapsule?: string, hero?: string) => {
      const metadataDict = get(romMetadata);

      const heroCached = await RestService.cacheHero(
        hero!,
        romId!.replace(/[/\\?%*:|"<> ]/g, '_')
      );

      metadataDict[romId].heroPath = heroCached;

      romMetadata.set({ ...metadataDict });
    });
    changeGridsId.set(romId);
    showChangeGridsModal.set(true);
  }

  /**
   * Downloads a rom.
   * @param romId The id of the rom.
   */
  static download(romId: string) {
    const rom = get(roms)[romId];
    downloadProgressInit.set((
      onStart: (fileSize: number) => void = () => {},
      onProgress: (progress: number) => void = () => {},
      onEnd: (finished: boolean) => void = () => {}
    ) => {
      RestService.downloadRom(rom, onStart, onProgress, onEnd);
    });
    showDownloadProgressModal.set(true);
  }

  /**
   * Replaces a rom's file.
   * @param romId The id of the rom.
   */
  static replaceFile(romId: string) {
    const rom = get(roms)[romId];

    replaceRomId.set(romId);
    replaceRomSystem.set(rom.system);

    showReplaceRomModal.set(true);
  }

  /**
   * Refreshes a rom's metadata.
   * @param igdbId The IGDB id of the rom.
   */
  static async refreshMetadata(igdbId: string): Promise<IGDBGame | null> {
    showLoadingModal.set(true);
    loadingModalMessage.set("Refreshing ROM Metadata...");

    return await IGDBService.getMetadata(igdbId).then((metadata: IGDBGame | null) => {
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
    await DialogService.ask(
      "This Can't be Undone!",
      "Are you sure you want to delete this rom?",
      "Yes",
      "No",
      true
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;

      const romsDict = get(roms);
      const rom = romsDict[romId];
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting ROM...");
      const success = await RestService.deleteRom(rom.path);
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