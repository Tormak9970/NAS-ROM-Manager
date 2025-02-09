import { libraries, romCustomizations, roms, romsByLibrary, romsBySystem, showErrorSnackbar, showInfoSnackbar, systems, systemTagConfigs } from "@stores/State";
import type { Library, ROM, ROMCustomization, System, SystemTagConfig } from "@types";
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
    await this.loadLibraries();
    SettingsController.registerSubs();
  }

  /**
   * Adds a new library.
   * @param library The library to add.
   */
  static async addLibrary(library: Library) {
    const loadedLibrary = await WebsocketController.addLibrary(library);

    const libraryMap = get(libraries);
    const systemMap = get(systems);
    const romMap = get(roms);
    
    const romEdits = get(romCustomizations);
    
    const romsLibraryLUT = get(romsByLibrary);
    const romsSystemLUT = get(romsBySystem);
    
    const tagConfigs = get(systemTagConfigs);


    libraryMap[library.name] = library;
    romsLibraryLUT[library.name] = [];

    for (const system of loadedLibrary.systems) {
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

    for (const customization of library.romCustomizations) {
      const id = hash64(customization.path);
      romEdits[id] = customization;
    }

    for (const rom of loadedLibrary.roms) {
      const id = hash64(rom.path);
      romMap[id] = rom;

      romsLibraryLUT[rom.library].push(id);
      romsSystemLUT[rom.system].push(id);
      systemMap[rom.system].romCount++;
    }


    libraries.set({ ...libraryMap });
    systems.set({ ...systemMap });
    roms.set({ ...romMap });

    romCustomizations.set({ ...romEdits });

    romsByLibrary.set({ ...romsLibraryLUT });
    romsBySystem.set({ ...romsSystemLUT });

    systemTagConfigs.set({ ...tagConfigs });
    

    LogController.log(`Loaded ${loadedLibrary.roms.length} new ROMs.`);

    get(showInfoSnackbar)({ message: "Library added" });
  }

  /**
   * Removes a new library.
   * @param library The library to remove.
   */
  static async removeLibrary(library: Library) {
    const success = await WebsocketController.removeLibrary(library);

    if (success) {
      const libraryMap = get(libraries);
      const systemMap = get(systems);
      const romMap = get(roms);
      
      const romEdits = get(romCustomizations);
      
      const romsLibraryLUT = get(romsByLibrary);
      const romsSystemLUT = get(romsBySystem);


      delete libraryMap[library.name];

      const romList = romsLibraryLUT[library.name];
      for (const id of romList) {
        const rom = romMap[id];

        delete romMap[id];
        delete romEdits[id];

        const system = systemMap[rom.system];
        system.romCount--;
        romsSystemLUT[rom.system].splice(romsSystemLUT[rom.system].indexOf(id), 1);
      }

      delete romsLibraryLUT[library.name];


      libraries.set({ ...libraryMap });
      systems.set({ ...systemMap });
      roms.set({ ...romMap });

      romCustomizations.set({ ...romEdits });

      romsByLibrary.set({ ...romsLibraryLUT });
      romsBySystem.set({ ...romsSystemLUT });


      get(showInfoSnackbar)({ message: "Library removed" });
    } else {
      get(showErrorSnackbar)({ message: "Failed to remove library" });
    }
  }

  /**
   * Loads the user's libraries.
   */
  static async loadLibraries() {
    const loadedLibraries = await WebsocketController.loadLibraries();

    const libraryMap: Record<string, Library> = {};
    const systemMap: Record<string, System> = {};
    const romMap: Record<string, ROM> = {};
    
    const romEdits: Record<string, ROMCustomization> = {};
    
    const romsLibraryLUT: Record<string, string[]> = {};
    const romsSystemLUT: Record<string, string[]> = {};

    const tagConfigs: Record<string, SystemTagConfig> = {};
    

    for (const loadedLibrary of loadedLibraries) {
      const library = loadedLibrary.library;

      libraryMap[library.name] = library;
      romsLibraryLUT[library.name] = [];

      for (const system of loadedLibrary.systems) {
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

      for (const customization of library.romCustomizations) {
        const id = hash64(customization.path);
        romEdits[id] = customization;
      }

      for (const rom of loadedLibrary.roms) {
        const id = hash64(rom.path);
        romMap[id] = rom;

        romsLibraryLUT[rom.library].push(id);
        romsSystemLUT[rom.system].push(id);
        systemMap[rom.system].romCount++;
      }
    }


    libraries.set(libraryMap);
    systems.set(systemMap);
    roms.set(romMap);

    romCustomizations.set(romEdits);

    romsByLibrary.set(romsLibraryLUT);
    romsBySystem.set(romsSystemLUT);

    systemTagConfigs.set(tagConfigs);

    LogController.log(`Loaded ${Object.keys(libraryMap).length} libraries.`);
    LogController.log(`Loaded ${Object.keys(systemMap).length} systems.`);
    LogController.log(`Loaded ${Object.keys(romMap).length} ROMs.`);
  }

  /**
   * Unloads the app's state and performs any cleanup.
   */
  static unload() {
    SettingsController.destroy();
  }
}