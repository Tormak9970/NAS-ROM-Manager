import { writable } from "svelte/store";
import type { Palette } from "@types";

export const isLandscape = writable(true);

// * Settings stores
export const palette = writable<Palette>("Auto");
export const useOledPalette = writable(false);
export const themePrimaryColor = writable("#a74bf2");