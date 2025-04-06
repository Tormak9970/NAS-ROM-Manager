import { changeCoverId, changeCoverOnSelect, changeCoverSearchId, showChangeCoverModal } from "@stores/Modals";
import { systems } from "@stores/State";
import { get } from "svelte/store";
import { DialogController } from "./utils/DialogController";

/**
 * The system controller.
 */
export class SystemController {
  /**
   * Shows the rom edit modal.
   * @param abbreviation The abbreviation of the system.
   */
  static edit(abbreviation: string) {
    // romEditingId.set(romId);
    // showEditRomModal.set(true);
  }

  /**
   * Changes the cover of a rom.
   * @param abbreviation The abbreviation of the system.
   */
  static changeCover(abbreviation: string) {
    let searchId = get(systems)[abbreviation].sgdbId;
    changeCoverSearchId.set(searchId)
    changeCoverOnSelect.set((cover: string, thumb: string) => {
      const systemsDict = get(systems);
      
      let system = systemsDict[abbreviation];
      system.coverPath = cover;
      system.thumbPath = thumb;

      systems.set({ ...systemsDict });
    });
    changeCoverId.set(abbreviation);
    showChangeCoverModal.set(true);
  }

  /**
   * Prompts the user to delete a rom.
   * @param abbreviation The abbreviation of the system.
   */
  static async delete(abbreviation: string) {
    await DialogController.ask(
      "This Can't be Undone!",
      "Are you sure you want to delete this system?",
      "Yes",
      "No",
      true
    ).then(async (shouldDelete: boolean) => {
      if (!shouldDelete) return;

      // const romsDict = get(roms);
      // const rom = romsDict[romId];
    
      // showLoadingModal.set(true);
      // loadingModalMessage.set("Deleting ROM...");
      // const success = await RestController.deleteRom(rom.path);
      // loadingModalMessage.set("");
      // showLoadingModal.set(false);
      // if (!success) return;

      // delete romsDict[romId];
      // roms.set({ ...romsDict });

      // const metadataDict = get(romMetadata);
      // delete metadataDict[romId];
      // romMetadata.set({ ...metadataDict });

      // const systemsMap = get(romsBySystem);
      // const system = systemsMap[rom.system];
      // const romIndex = system.indexOf(romId);
      // system.splice(romIndex, 1);
      // romsBySystem.set({ ...systemsMap });
    });
  }
}