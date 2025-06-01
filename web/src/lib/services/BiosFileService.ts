import { downloadProgressInit, loadingModalMessage, showDownloadProgressModal, showLoadingModal } from "@stores/Modals";
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
   * @param system The system of the bios file to get.
   * @param filename The filename of the bios file to get.
   * @returns The full file path.
   */
  static getFilePath(system: string, filename: string): string {
    const lib = get(library);

    return lib.libraryPath + "/" + lib.biosDir + "/" + system + "/" + filename;
  }

  /**
   * Downloads the specified bios file.
   * @param system The system of the bios file to download.
   * @param filename The bios file to download.
   */
  static async download(system: string, filename: string) {
    const filePath = BiosFileService.getFilePath(system, filename);
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
   * Prompts the user to delete a bios file.
   * @param system The system of the bios file to delete.
   * @param filename The bios file to delete.
   */
  static async delete(system: string, filename: string) {
    await DialogService.ask(
      "This Can't be Undone!",
      "Are you sure you want to delete this bios file?",
      "Yes",
      "No",
      true
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;
      
      const filePath = BiosFileService.getFilePath(system, filename);

      const systemsDict = get(systems);
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting BIOS File...");
      const success = await RestService.deleteBIOS(filePath);
      loadingModalMessage.set("");
      showLoadingModal.set(false);
      if (!success) return;

      const files = systemsDict[system].biosFiles;
      systemsDict[system].biosFiles.splice(files.indexOf(filename), 1);
      systems.set({ ...systemsDict });
    });
  }
}