import { isFirstSetup, showEditLibraryModal } from "@stores/Modals";
import { library, romCustomizations, roms, romsBySystem, showInfoSnackbar, systems, systemTagConfigs } from "@stores/State";
import type { Library, LoadResult } from "@types";
import { hash64 } from "@utils";
import { get } from "svelte/store";
import { ApiController } from "./ApiController";
import { LogController } from "./utils/LogController";
import { SettingsController } from "./utils/SettingsController";
import { WebsocketController } from "./utils/WebsocketController";

/**
 * The core app controller.
 */
export class AppController {
  /**
   * Loads the app's state.
   */
  static async load() {
    await SettingsController.init();
    await ApiController.init();
    
    const lib = get(library);
    if (lib.libraryPath === "") {
      showEditLibraryModal.set(true);
    } else {
      isFirstSetup.set(false);
      await this.loadLibrary();
    }

    SettingsController.registerSubs();
  }

  private static setStateFromLoadRes(loadRes: LoadResult) {
    const systemMap = get(systems);
    const romMap = get(roms);
    
    const romEdits = get(romCustomizations);
    
    const romsSystemLUT = get(romsBySystem);
    
    const tagConfigs = get(systemTagConfigs);


    library.set(loadRes.library);

    for (const system of loadRes.systems) {
      if (!systemMap[system.abbreviation]) {
        systemMap[system.abbreviation] = system;
      }

      if (!romsSystemLUT[system.abbreviation]) {
        romsSystemLUT[system.abbreviation] = [];
      }

      if (!tagConfigs[system.abbreviation]) {
        tagConfigs[system.abbreviation] = system.tagConfig;
      }
    }

    for (const customization of loadRes.romCustomizations) {
      const id = hash64(customization.path);
      romEdits[id] = customization;
    }

    for (const rom of loadRes.roms) {
      const id = hash64(rom.path);
      romMap[id] = rom;

      romsSystemLUT[rom.system].push(id);
    }


    systems.set({ ...systemMap });
    roms.set({ ...romMap });

    romCustomizations.set({ ...romEdits });

    romsBySystem.set({ ...romsSystemLUT });

    systemTagConfigs.set({ ...tagConfigs });
    
    
    LogController.log(`Loaded ${Object.keys(systemMap).length} systems.`);
    LogController.log(`Loaded ${Object.keys(romMap).length} ROMs.`);
    
    get(showInfoSnackbar)({ message: "Library loaded" });
  }

  /**
   * Updates the app's library.
   * @param library The new library info.
   */
  static async updateLibrary(library: Library) {
    const loadRes = await WebsocketController.updateLibrary(library);

    AppController.setStateFromLoadRes(loadRes);
  }

  /**
   * Loads the user's libraries.
   */
  static async loadLibrary() {
    const loadRes = await WebsocketController.loadLibrary();

    AppController.setStateFromLoadRes(loadRes);
  }

  /**
   * Unloads the app's state and performs any cleanup.
   */
  static unload() {
    SettingsController.destroy();
  }
}