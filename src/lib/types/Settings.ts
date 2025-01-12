import type { Collection, Library } from "./Library";

export type Palette = "Auto" | "Dark" | "Light";

export type ThemeSettings = {
  primaryColor: string;
  palette: Palette;
  useOledPalette: boolean;
}

export type Settings = {
  FILE_SIG_DO_NOT_EDIT: "dev.travislane.nas-rom-manager";
  version: string;
  theme: ThemeSettings;
  libraries: Library[];
  collections: Collection[];
}