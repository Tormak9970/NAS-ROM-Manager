import { writable } from "svelte/store";
import type { Collection, Library, ROM, Palette, System, ROMCustomization } from "@types";

export const isLandscape = writable(true);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showErrorSnackbar = writable<(data: ShowErrorOptions) => void>();

// * Settings stores
export const palette = writable<Palette>("Auto");
export const useOledPalette = writable(false);
export const themePrimaryColor = writable("#a74bf2");

// * App State
export const libraries = writable<Record<string, Library>>({});
export const systems = writable<Record<string, System>>({});
export const roms = writable<Record<string, ROM>>({});
export const collections = writable<Collection[]>([]);

export const romCustomizations = writable<Record<string, ROMCustomization>>({});

export const romsByLibrary = writable<Record<string, string[]>>({});
export const romsBySystem = writable<Record<string, string[]>>({});