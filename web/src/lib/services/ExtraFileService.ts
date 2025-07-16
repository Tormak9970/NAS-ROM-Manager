import { WebsocketService } from "@services/utils/WebsocketService";
import { downloadProgressInit, loadingModalMessage, replaceExtraFilePath, replaceExtraFileRomId, replaceExtraFileSystem, replaceExtraFileType, showDownloadProgressModal, showLoadingModal, showReplaceExtraFileModal } from "@stores/Modals";
import { library, romDLCs, romUpdates } from "@stores/State";
import { ExtraFileType } from "@types";
import { get } from "svelte/store";
import { DialogService } from "./utils/DialogService";
import { RestService } from "./utils/RestService";

/**
 * The ROM Extra File controller.
 */
export class ExtraFileService {
  /**
   * Gets the full file path for an extra file.
   * @param type The extra file type.
   * @param systemFolder The system of the extra file to get.
   * @param romId The id of the rom associated with the extra file.
   * @param filename The filename of the extra file to get.
   * @returns The full file path.
   */
  static getFilePath(type: ExtraFileType, systemFolder: string, romId: string, filename: string): string {
    const lib = get(library);

    return lib.libraryPath + "/" + (type === ExtraFileType.DLC ? lib.dlcDir : lib.updateDir) + "/" + systemFolder + "/" + romId + "/" + filename;
  }

  /**
   * Downloads the specified extra file.
   * @param type The extra file type.
   * @param systemFolder The system of the extra file to get.
   * @param romId The id of the rom associated with the extra file.
   * @param filename The extra file to download.
   */
  static async download(type: ExtraFileType, systemFolder: string, romId: string, filename: string) {
    const filePath = ExtraFileService.getFilePath(type, systemFolder, romId, filename);
    downloadProgressInit.set((
      onStart: (fileSize: number) => void = () => {},
      onProgress: (progress: number) => void = () => {},
      onEnd: (finished: boolean) => void = () => {}
    ) => {
      RestService.downloadROMExtra(filePath, onStart, onProgress, onEnd);
    });
    showDownloadProgressModal.set(true);
  }

  /**
   * Replaces the specified extra file.
   * @param type The extra file type.
   * @param systemFolder The system of the extra file to replace.
   * @param romId The id of the rom associated with the extra file.
   * @param filename The extra file to replace.
   */
  static async replaceFile(type: ExtraFileType, systemFolder: string, romId: string, filename: string) {
    const filePath = ExtraFileService.getFilePath(type, systemFolder, romId, filename);

    replaceExtraFileType.set(type);
    replaceExtraFileSystem.set(systemFolder);
    replaceExtraFileRomId.set(romId);
    replaceExtraFilePath.set(filePath);

    showReplaceExtraFileModal.set(true);
  }

  /**
   * Prompts the user to delete a extra file.
   * @param type The extra file type.
   * @param systemFolder The system of the extra file to replace.
   * @param romId The id of the rom associated with the extra file.
   * @param filename The extra file to delete.
   */
  static async delete(type: ExtraFileType, systemFolder: string, romId: string, filename: string) {
    await DialogService.ask(
      "Warning!",
      `Are you sure you want to delete this ${type === ExtraFileType.DLC ? "DLC" : "update"} file? This can't be undone.`,
      "Yes",
      "No",
      true
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;
      
      const filePath = ExtraFileService.getFilePath(type, systemFolder, romId, filename);
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting Extra File...");
      const success = await RestService.deleteROMExtra(filePath);
      await WebsocketService.removeExtraFileFromCache(type, romId, filename);
      loadingModalMessage.set("");
      showLoadingModal.set(false);
      if (!success) return;

      if (type === ExtraFileType.DLC) {
        const dlcs = get(romDLCs);
        const files = dlcs[romId];
        dlcs[romId].splice(files.indexOf(filename), 1);
        romDLCs.set({ ...dlcs });
      } else {
        const updates = get(romUpdates);
        const files = updates[romId];
        updates[romId].splice(files.indexOf(filename), 1);
        romUpdates.set({ ...updates });
      }
    });
  }
}