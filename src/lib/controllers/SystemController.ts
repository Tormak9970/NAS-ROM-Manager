import { WebsocketController } from "@controllers/utils/WebsocketController";
import { changeCoverId, changeCoverOnSelect, changeCoverSearchId, loadingModalMessage, showChangeCoverModal, showEditSystemModal, showLoadingModal, systemEditingId } from "@stores/Modals";
import { roms, romsBySystem, systems } from "@stores/State";
import type { ParserPattern } from "@types";
import { isValidRegex } from "@utils";
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
    systemEditingId.set(abbreviation);
    showEditSystemModal.set(true);
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

      const systemsDict = get(systems);
    
      showLoadingModal.set(true);
      loadingModalMessage.set("Deleting System...");
      const success = await WebsocketController.deleteParser(abbreviation);
      loadingModalMessage.set("");
      showLoadingModal.set(false);
      if (!success) return;

      delete systemsDict[abbreviation];
      systems.set({ ...systemsDict });

      const systemsMap = get(romsBySystem);
      const romsDict = get(roms);

      for (const id of systemsMap[abbreviation]) {
        delete romsDict[id];
      }

      roms.set({ ...romsDict });

      delete systemsMap[abbreviation];
      romsBySystem.set({ ...systemsMap });
    });
  }
  
  /**
   * Checks if a ParserPattern is valid.
   * @param pattern The pattern to check.
   * @returns True if the pattern is valid.
   */
  static async validateParserPattern(pattern: ParserPattern): Promise<boolean> {
    return pattern.glob !== "" &&
      pattern.regex !== "" &&
      (pattern.downloadStrategy.type === "single-file" || pattern.downloadStrategy.parent !== "") &&
      isValidRegex(pattern.regex) &&
      await WebsocketController.isValidGlob(pattern.glob);
  }
}