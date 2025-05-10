import { DEFAULT_FILTERS } from "@models";
import type { DBFilters, Library, Palette, ROM, ROMMetadata, SGDBGame, System, SystemTagConfig } from "@types";
import { localStorageWritable } from "@utils";
import { derived, writable } from "svelte/store";

export const requestTimeoutLength = 5000;

export const isLandscape = writable(true);

export const loadedApp = writable(false);
export const loadedSettings = writable(false);
export const loadedLibrary = writable(false);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showWarningSnackbar = writable<(data: ShowWarningOptions) => void>();

export const steamGridSearchCache = writable<{ [appid: string]: SGDBGame[] }>({});
export const hasMorePagesCache = writable<{ [steamGridId: string]: Record<string, boolean> }>({});

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

export const fileFormatsBySystem = derived([ roms ], ([$roms]: [Record<string, ROM>]) => {
  const formats = Object.values($roms).reduce((formats: Record<string, Set<string>>, rom: ROM) => {
    if (!formats[rom.system]) {
      formats[rom.system] = new Set<string>();
    }

    formats[rom.system].add(rom.format);
    
    return formats;
  }, {});

  return Object.fromEntries(Object.entries(formats).map(([key, value]) => {
    return [key, Array.from(value.values())]
  }));
});

export const romMetadata = writable<Record<string, ROMMetadata>>({});

export const metadataSearchFilters = derived([ romMetadata ], ([$romMetadata]: [Record<string, ROMMetadata>]) => {
  const filters = Object.values($romMetadata).reduce((filters: Record<string, Set<string>>, metadata: ROMMetadata) => {
    if (metadata) {
      if (metadata.metadata?.metadata?.genres && metadata.metadata?.metadata?.genres?.length > 0) {
        metadata.metadata?.metadata?.genres.forEach((genre: string) => filters.genres.add(genre));
      }
      if (metadata.metadata?.metadata?.developers && metadata.metadata?.metadata?.developers?.length > 0) {
        metadata.metadata?.metadata?.developers.forEach((dev: string) => filters.developers.add(dev));
      }
      if (metadata.metadata?.metadata?.publishers && metadata.metadata?.metadata?.publishers?.length > 0) {
        metadata.metadata?.metadata?.publishers.forEach((pub: string) => filters.publishers.add(pub));
      }
    }
    
    return filters;
  }, {
    genres: new Set<string>(),
    developers: new Set<string>(),
    publishers: new Set<string>()
  });

  return filters;
});

export const romsBySystem = writable<Record<string, string[]>>({});

export const dbFilters = localStorageWritable<DBFilters>("sgdb-filters", DEFAULT_FILTERS);

export const systemTagConfigs = writable<Record<string, SystemTagConfig>>({});