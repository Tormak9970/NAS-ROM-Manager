import { WebsocketController } from "@controllers/utils/WebsocketController";
import { changeGridsId, changeGridsOnSelect, changeGridsSearchId, changeGridsType, loadingModalMessage, showChangeGridsModal, showEditSystemModal, showLoadingModal, systemEditingId } from "@stores/Modals";
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
   * Changes the capsule of a rom.
   * @param abbreviation The abbreviation of the system.
   */
  static changeCapsule(abbreviation: string) {
    let searchId = get(systems)[abbreviation].sgdbId;
    changeGridsSearchId.set(searchId);
    changeGridsType.set("Capsule");
    changeGridsOnSelect.set((fullCapsule?: string, thumbCapsule?: string) => {
      const systemsDict = get(systems);
      
      let system = systemsDict[abbreviation];
      system.fullCapsulePath = fullCapsule!;
      system.thumbCapsulePath = thumbCapsule!;

      systems.set({ ...systemsDict });
    });
    changeGridsId.set(abbreviation);
    showChangeGridsModal.set(true);
  }

  /**
   * Changes the hero of a rom.
   * @param abbreviation The abbreviation of the system.
   */
  static changeHero(abbreviation: string) {
    let searchId = get(systems)[abbreviation].sgdbId;
    changeGridsSearchId.set(searchId);
    changeGridsType.set("Hero");
    changeGridsOnSelect.set((fullCapsule?: string, thumbCapsule?: string, hero?: string) => {
      const systemsDict = get(systems);
      
      let system = systemsDict[abbreviation];
      system.heroPath = hero!;

      systems.set({ ...systemsDict });
    });
    changeGridsId.set(abbreviation);
    showChangeGridsModal.set(true);
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