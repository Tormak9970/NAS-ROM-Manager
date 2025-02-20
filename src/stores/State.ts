import { DEFAULT_FILTERS } from "@models";
import type { DBFilters, Library, Palette, ROM, ROMCustomization, System, SystemTagConfig } from "@types";
import { localStorageWritable } from "@utils";
import { writable } from "svelte/store";

export const requestTimeoutLength = 5000;

export const isLandscape = writable(true);
export const loadedSettings = writable(false);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showWarningSnackbar = writable<(data: ShowWarningOptions) => void>();

// * Settings stores
export const palette = writable<Palette>("Auto");
export const useOledPalette = writable(false);
export const themePrimaryColor = writable("#a74bf2");

export const landingPage = writable("library");
export const landscapeViews = writable(["Dashboard", "Library", "Systems", "Emulators", "Settings"]);
export const portraitViews = writable(["Dashboard", "Library", "Search", "Systems", "Settings"]);
export const libraryGridType = writable("portrait");

export const saveMetadataAlongside = writable(false);

export const reducedMotion = writable(false);

// * App State
export const library = writable<Library>({
  libraryPath: "",
  romDir: "roms",
  emulatorDir: "emulators",
  biosDir: "bios"
});
export const systems = writable<Record<string, System>>({});
export const roms = writable<Record<string, ROM>>({});
export const emulators = writable<Record<string, string>>({});

export const romCustomizations = writable<Record<string, ROMCustomization>>({});

export const romsBySystem = writable<Record<string, string[]>>({});

export const dbFilters = localStorageWritable<DBFilters>("sgdb-filters", DEFAULT_FILTERS);

export const systemTagConfigs = writable<Record<string, SystemTagConfig>>({});