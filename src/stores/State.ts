import { DEFAULT_FILTERS } from "@models";
import type { DBFilters, Library, Palette, ROM, ROMCustomization, System } from "@types";
import { localStorageWritable } from "@utils";
import { writable } from "svelte/store";

export const requestTimeoutLength = 5000;

export const isLandscape = writable(true);
export const loadedSettings = writable(false);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showErrorSnackbar = writable<(data: ShowErrorOptions) => void>();

// * Settings stores
export const palette = writable<Palette>("Auto");
export const useOledPalette = writable(false);
export const themePrimaryColor = writable("#a74bf2");

export const landingPage = writable("library");
export const libraryGridType = writable("portrait");

// * App State
export const libraries = writable<Record<string, Library>>({});
export const systems = writable<Record<string, System>>({});
export const roms = writable<Record<string, ROM>>({});

export const romCustomizations = writable<Record<string, ROMCustomization>>({});

export const romsByLibrary = writable<Record<string, string[]>>({});
export const romsBySystem = writable<Record<string, string[]>>({});

export const dbFilters = localStorageWritable<DBFilters>("sgdb-filters", DEFAULT_FILTERS);