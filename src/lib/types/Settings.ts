import type { Collection, Library, ROM } from "./Library";

export type Palette = "Auto" | "Dark" | "Light";

export type Settings = {
  FILE_SIG_DO_NOT_EDIT: "dev.travislane.nas-rom-manager";
  version: string;
  libraries: Library[];
  collections: Collection[];
  romOverrides: ROM[];
}

export const DEFAULT_SETTINGS: Settings = {
  "FILE_SIG_DO_NOT_EDIT": "dev.travislane.nas-rom-manager",
  "version": APP_VERSION,
  "libraries": [],
  "collections": [],
  "romOverrides": [],
};