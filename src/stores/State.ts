import { writable } from "svelte/store";
import type { Collection, Library, ROM, Palette } from "@types";

export const isLandscape = writable(true);

export const showInfoSnackbar = writable<(data: ShowInfoOptions) => void>();
export const showErrorSnackbar = writable<(data: ShowErrorOptions) => void>();

// * Settings stores
export const palette = writable<Palette>("Auto");
export const useOledPalette = writable(false);
export const themePrimaryColor = writable("#a74bf2");

// * App State
export const libraries = writable<Library[]>([]);
export const libraryROMLUT = writable<Record<string, ROM[]>>({});
export const ROMs = writable<ROM[]>([]);
export const collections = writable<Collection[]>([]);
export const tags = writable<string[]>([]);