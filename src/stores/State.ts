import { DEFAULT_FILTERS } from "@models";
import type { DBFilter, Library, Palette, ROM, ROMMetadata, SGDBGame, System, SystemTagConfig } from "@types";
import { localStorageWritable } from "@utils";
import { writable } from "svelte/store";

export const requestTimeoutLength = 5000;

export const isLandscape = writable(true);

export const loadedApp = writable(false);
export const loadedSettings = writable(false);
export const loadedLibrary = writable(false);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showWarningSnackbar = writable<(data: ShowWarningOptions) => void>();

export const steamGridSearchCache = writable<{ [appid: string]: SGDBGame[] }>({});
export const hasMorePagesCache = writable<{ [steamGridId: string]: boolean }>({});

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

export const romMetadata = writable<Record<string, ROMMetadata>>({});

export const romsBySystem = writable<Record<string, string[]>>({});

export const dbFilters = localStorageWritable<DBFilter>("sgdb-filters", DEFAULT_FILTERS);

export const systemTagConfigs = writable<Record<string, SystemTagConfig>>({});