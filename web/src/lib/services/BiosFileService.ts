import { downloadProgressInit, loadingModalMessage, replaceBiosFilePath, replaceBiosFileSystem, showDownloadProgressModal, showLoadingModal, showReplaceBiosFileModal } from "@stores/Modals";
import { library, systems } from "@stores/State";
import { get } from "svelte/store";
import { DialogService } from "./utils/DialogService";
import { RestService } from "./utils/RestService";

/**
 * The BIOS File controller.
 */
export class BiosFileService {
  /**
   * Gets the full file path for a bios file.
   * @param systemFolder The system of the bios file to get.
   * @param filename The filename of the bios file to get.
   * @returns The full file path.
   */
  static getFilePath(systemFolder: string, filename: string): string {
    const lib = get(library);

    return lib.libraryPath + "/" + lib.biosDir + "/" + systemFolder + "/" + filename;
  }

  /**
   * Downloads the specified bios file.
   * @param systemFolder The system of the bios file to download.
   * @param filename The bios file to download.
   */
  static async download(systemFolder: string, filename: string) {
    const filePath = BiosFileService.getFilePath(systemFolder, filename);
    downloadProgressInit.set((
      onStart: (fileSize: number) => void = () => {},
      onProgress: (progress: number) => void = () => {},
      onEnd: (finished: boolean) => void = () => {}
    ) => {
      RestService.downloadBIOS(filePath, onStart, onProgress, onEnd);
    });
    showDownloadProgressModal.set(true);
  }

  /**
   * Replaces the specified bios file.
   * @param systemKey The system of the bios file to replace.
   * @param filename The bios file to replace.
   */
  static async replaceFile(systemKey: string, filename: string) {
    const system = get(systems)[systemKey];
    const filePath = BiosFileService.getFilePath(system.folder, filename);

    replaceBiosFileSystem.set(systemKey);
    replaceBiosFilePath.set(filePath);

    showReplaceBiosFileModal.set(true);
  }

  /**
   * Prompts the user to delete a bios file.
   * @param system The system of the bios file to delete.
   * @param filename The bios file to delete.
   */
  static async delete(systemKey: string, filename: string) {
    await DialogService.ask(
      "Warning!",
      "Are you sure you want to delete this bios file? This can't be undone.",
      "Yes",
      "No",
      true
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;
      
      const filePath = BiosFileService.getFilePath(systemKey, filename);

      const systemsDict = get(systems);
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting BIOS File...");
      const success = await RestService.deleteBIOS(filePath);
      loadingModalMessage.set("");
      showLoadingModal.set(false);
      if (!success) return;

      const files = systemsDict[systemKey].biosFiles;
      systemsDict[systemKey].biosFiles.splice(files.indexOf(filename), 1);
      systems.set({ ...systemsDict });
    });
  }
}