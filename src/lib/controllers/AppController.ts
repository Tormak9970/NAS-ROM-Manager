import type { Collection, Library, ROM, ROMCustomization, System } from "@types";
import { SettingsController } from "./utils/SettingsController";
import { LogController } from "./utils/LogController";
import { RustInterop } from "./utils/RustInterop";
import { collections, libraries, romCustomizations, roms, romsByLibrary, romsBySystem, showErrorSnackbar, showInfoSnackbar, systems } from "@stores/State";
import { hash64 } from "@utils";
import { get } from "svelte/store";

/**
 * The core app controller.
 */
export class AppController {
  /**
   * Loads the app's state.
   */
  static async load() {
    await SettingsController.init();
    await this.loadLibraries();
    SettingsController.registerSubs();
  }

  /**
   * Adds a new library.
   * @param library The library to add.
   */
  static async addLibrary(library: Library) {
    const loadedLibrary = await RustInterop.addLibrary(library);

    const libraryMap = get(libraries);
    const systemMap = get(systems);
    const romMap = get(roms);
    const collectionList = get(collections);
    
    const romEdits = get(romCustomizations);
    
    const romsLibraryLUT = get(romsByLibrary);
    const romsSystemLUT = get(romsBySystem);


    libraryMap[library.name] = library;
    romsLibraryLUT[library.name] = [];

    for (const customization of library.romCustomizations) {
      const id = hash64(customization.path);
      romEdits[id] = customization;
    }

    for (const rom of loadedLibrary.roms) {
      const id = hash64(rom.path);
      romMap[id] = rom;

      romsLibraryLUT[rom.library].push(id);

      if (!romsSystemLUT[rom.system]) romsSystemLUT[rom.system] = [];
      romsSystemLUT[rom.system].push(id);

      let system = systemMap[rom.system];
      if (!system) {
        system = {
          fullName: rom.systemFullName,
          abbreviation: rom.system,
          romCount: 0,
        }

        systemMap[rom.system] = system;
      }
      system.romCount++;
    }


    libraries.set({ ...libraryMap });
    systems.set({ ...systemMap });
    roms.set({ ...romMap });
    collections.set([ ...collectionList ]);

    romCustomizations.set({ ...romEdits });

    romsByLibrary.set({ ...romsLibraryLUT });
    romsBySystem.set({ ...romsSystemLUT });


    LogController.log(`Loaded ${loadedLibrary.roms.length} new ROMs.`);

    get(showInfoSnackbar)({ message: "Library added" });
  }

  /**
   * Removes a new library.
   * @param library The library to remove.
   */
  static async removeLibrary(library: Library) {
    const success = await RustInterop.removeLibrary(library);

    if (success) {
      const libraryMap = get(libraries);
      const systemMap = get(systems);
      const romMap = get(roms);
      const collectionList = get(collections);
      
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

        if (system.romCount === 0) {
          delete systemMap[rom.system];
        }

        for (const collection of collectionList) {
          const idIndex = collection.romsIds.indexOf(id);

          if (idIndex !== -1) {
            collection.romsIds.splice(idIndex, 1);
          }
        }
      }

      delete romsLibraryLUT[library.name];


      libraries.set({ ...libraryMap });
      systems.set({ ...systemMap });
      roms.set({ ...romMap });
      collections.set([ ...collectionList ]);

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
    const loadedLibraries = await RustInterop.loadLibraries();

    const libraryMap: Record<string, Library> = {};
    const systemMap: Record<string, System> = {};
    const romMap: Record<string, ROM> = {};
    
    const romEdits: Record<string, ROMCustomization> = {};
    
    const romsLibraryLUT: Record<string, string[]> = {};
    const romsSystemLUT: Record<string, string[]> = {};
    

    for (const loadedLibrary of loadedLibraries) {
      const library = loadedLibrary.library;

      libraryMap[library.name] = library;
      romsLibraryLUT[library.name] = [];

      for (const customization of library.romCustomizations) {
        const id = hash64(customization.path);
        romEdits[id] = customization;
      }

      for (const rom of loadedLibrary.roms) {
        const id = hash64(rom.path);
        romMap[id] = rom;

        romsLibraryLUT[rom.library].push(id);

        if (!romsSystemLUT[rom.system]) romsSystemLUT[rom.system] = [];
        romsSystemLUT[rom.system].push(id);

        let system = systemMap[rom.system];
        if (!system) {
          system = {
            fullName: rom.systemFullName,
            abbreviation: rom.system,
            romCount: 0,
          }

          systemMap[rom.system] = system;
        }
        system.romCount++;
      }
    }


    libraries.set(libraryMap);
    systems.set(systemMap);
    roms.set(romMap);

    romCustomizations.set(romEdits);

    romsByLibrary.set(romsLibraryLUT);
    romsBySystem.set(romsSystemLUT);

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