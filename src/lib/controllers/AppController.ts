import { isFirstSetup, loadingModalMessage, showEditLibraryModal, showLoadingModal } from "@stores/Modals";
import { library, loadedLibrary, romMetadata, roms, romsBySystem, systems, systemTagConfigs } from "@stores/State";
import type { Library, LoadResult, ROMMetadata } from "@types";
import { hash64 } from "@utils";
import { get } from "svelte/store";
import { IGDBController } from "./IGDBController";
import { SGDBController } from "./SGDBController";
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
    await SGDBController.init();
    await IGDBController.init();
    
    const lib = get(library);
    if (lib.libraryPath === "") {
      showEditLibraryModal.set(true);
    } else {
      isFirstSetup.set(false);
      await this.loadLibrary();
    }

    SettingsController.registerSubs();
  }

  private static async setStateFromLoadRes(loadRes: LoadResult, metadata: Record<string, ROMMetadata>) {
    const systemMap = get(systems);
    const romMap = get(roms);
    
    const romEdits = metadata;
    const romsSystemLUT = get(romsBySystem);
    const tagConfigs = get(systemTagConfigs);


    library.set(loadRes.library);

    for (const system of loadRes.systems) {
      if (system.sgdbId === "") {
        system.sgdbId = await SGDBController.chooseSteamGridGameId(system.abbreviation, system.name);
      }

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

    for (const rom of loadRes.roms) {
      const id = hash64(rom.path);
      romMap[id] = rom;

      if (!romEdits[id]) {
        romEdits[id] = {
          title: rom.title,
          fullCapsulePath: "",
          thumbCapsulePath: "",
          sgdbId: "",
          igdbId: "",
          heroPath: "",
          metadata: null,
          isFavorite: false,
        }
      }

      if (romEdits[id].sgdbId === "") {
        romEdits[id].sgdbId = await SGDBController.chooseSteamGridGameId(id, romEdits[id].title || rom.title);
      }

      romsSystemLUT[rom.system].push(id);
    }


    systems.set({ ...systemMap });
    roms.set({ ...romMap });

    romMetadata.set({ ...romEdits });

    romsBySystem.set({ ...romsSystemLUT });

    systemTagConfigs.set({ ...tagConfigs });
    
    
    LogController.log(`Loaded ${Object.keys(systemMap).length} systems.`);
    LogController.log(`Loaded ${Object.keys(romMap).length} ROMs.`);
  }

  /**
   * Refreshes the frontend's metadata.
   */
  static async refreshMetadata() {
    loadingModalMessage.set("Refreshing Metadata...");
    showLoadingModal.set(true);

    WebsocketController.refreshMetadata().then((metadata) => {
      romMetadata.set({ ...metadata });

      showLoadingModal.set(false);
      
      LogController.log(`Refreshed ROM Metadata.`);
    });
  }
  
  /**
   * Refreshes the frontend's metadata.
   */
  static async refreshLibrary() {
    loadingModalMessage.set("Refreshing Library...");
    showLoadingModal.set(true);
    loadedLibrary.set(false);

    WebsocketController.updateLibrary(get(library)).then(async (loadRes) => {
      const metadata = await WebsocketController.refreshMetadata();

      await AppController.setStateFromLoadRes(loadRes, metadata);
      loadedLibrary.set(true);
      showLoadingModal.set(false);
    });
  }

  /**
   * Updates the app's library.
   * @param library The new library info.
   */
  static async updateLibrary(library: Library) {
    const metadata = await WebsocketController.refreshMetadata();
    const loadRes = await WebsocketController.updateLibrary(library);

    await AppController.setStateFromLoadRes(loadRes, metadata);
    loadedLibrary.set(true);
  }

  /**
   * Loads the user's libraries.
   */
  static async loadLibrary() {
    const metadata = await WebsocketController.getMetadata();
    const loadRes = await WebsocketController.loadLibrary();

    await AppController.setStateFromLoadRes(loadRes, metadata);
    loadedLibrary.set(true);
  }

  /**
   * Unloads the app's state and performs any cleanup.
   */
  static unload() {
    SettingsController.destroy();
  }
}