import { UpdateService } from "@services/UpdateService";
import { isFirstSetup, loadingModalMessage, showEditLibraryModal, showLoadingModal } from "@stores/Modals";
import { library, loadedLibrary, romDLCs, romMetadata, roms, romsBySystem, romUpdates, systems, systemTagConfigs } from "@stores/State";
import type { Library, LoadResult, ROMMetadata } from "@types";
import { hash64 } from "@utils";
import { get } from "svelte/store";
import { IGDBService } from "./IGDBService";
import { SGDBService } from "./SGDBService";
import { LogService } from "./utils/LogService";
import { SettingsService } from "./utils/SettingsService";
import { WebsocketService } from "./utils/WebsocketService";

/**
 * The Core App Service.
 */
export class AppService {
  /**
   * Loads the app's state.
   */
  static async load() {
    UpdateService.checkForUpdate();
    await SettingsService.init();
    await SGDBService.init();
    await IGDBService.init();
    
    const lib = get(library);
    if (lib.libraryPath === "") {
      showEditLibraryModal.set(true);
    } else {
      isFirstSetup.set(false);
      await this.loadLibrary();
    }

    SettingsService.registerSubs();
  }

  private static async setStateFromLoadRes(loadRes: LoadResult, metadata: Record<string, ROMMetadata>) {
    const systemMap = get(systems);
    const romMap = get(roms);
    
    const romEdits = metadata;
    const romsSystemLUT = get(romsBySystem);
    const tagConfigs = get(systemTagConfigs);


    library.set(loadRes.library);

    romDLCs.set(loadRes.dlcs);
    romUpdates.set(loadRes.updates);

    for (const system of loadRes.systems) {
      if (system.sgdbId === "") {
        system.sgdbId = await SGDBService.chooseSteamGridGameId(system.abbreviation, system.name);
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
        romEdits[id].sgdbId = await SGDBService.chooseSteamGridGameId(id, romEdits[id].title || rom.title);
      }

      romsSystemLUT[rom.system].push(id);
    }


    systems.set({ ...systemMap });
    roms.set({ ...romMap });

    romMetadata.set({ ...romEdits });

    romsBySystem.set({ ...romsSystemLUT });

    systemTagConfigs.set({ ...tagConfigs });
    
    
    LogService.log(`Loaded ${Object.keys(systemMap).length} systems.`);
    LogService.log(`Loaded ${Object.keys(romMap).length} ROMs.`);
  }

  /**
   * Refreshes the frontend's metadata.
   */
  static async refreshMetadata() {
    loadingModalMessage.set("Refreshing Metadata...");
    showLoadingModal.set(true);

    WebsocketService.refreshMetadata().then((metadata) => {
      romMetadata.set({ ...metadata });

      showLoadingModal.set(false);
      
      LogService.log(`Refreshed ROM Metadata.`);
    });
  }
  
  /**
   * Refreshes the frontend's metadata.
   */
  static async refreshLibrary() {
    loadingModalMessage.set("Refreshing Library...");
    showLoadingModal.set(true);
    loadedLibrary.set(false);

    WebsocketService.updateLibrary(get(library)).then(async (loadRes) => {
      const metadata = await WebsocketService.refreshMetadata();

      await AppService.setStateFromLoadRes(loadRes, metadata);
      loadedLibrary.set(true);
      showLoadingModal.set(false);
    });
  }

  /**
   * Updates the app's library.
   * @param library The new library info.
   */
  static async updateLibrary(library: Library) {
    const metadata = await WebsocketService.refreshMetadata();
    const loadRes = await WebsocketService.updateLibrary(library);

    await AppService.setStateFromLoadRes(loadRes, metadata);
    loadedLibrary.set(true);
  }

  /**
   * Loads the user's libraries.
   */
  static async loadLibrary() {
    const metadata = await WebsocketService.getMetadata();
    const loadRes = await WebsocketService.loadLibrary();

    await AppService.setStateFromLoadRes(loadRes, metadata);
    loadedLibrary.set(true);
  }

  /**
   * Unloads the app's state and performs any cleanup.
   */
  static unload() {
    SettingsService.destroy();
  }
}