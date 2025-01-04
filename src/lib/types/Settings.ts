
export type Palette = "Auto" | "Dark" | "Light";

export type Settings = {
  FILE_SIG_DO_NOT_EDIT: "dev.travislane.media-scale";
  version: string;
  
}

export const DEFAULT_SETTINGS: Settings = {
  "FILE_SIG_DO_NOT_EDIT": "dev.travislane.media-scale",
  "version": "",
};