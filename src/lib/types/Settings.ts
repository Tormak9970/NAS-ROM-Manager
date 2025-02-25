import type { Library } from "./Library";

export type Palette = "Auto" | "Dark" | "Light";

export type ThemeSettings = {
  primaryColor: string;
  palette: Palette;
  useOledPalette: boolean;
}

export type NavigationSettings = {
  landingPage: string;
  landscapeViews: string[];
  portraitViews: string[];
}

export type MetadataSettings = {
  saveAlongsideROMs: boolean;
}

export type AccessibilitySettings = {
  reducedMotion: boolean;
}

export type Settings = {
  FILE_SIG_DO_NOT_EDIT: "dev.travislane.nas-rom-manager";
  version: string;
  theme: ThemeSettings;
  navigation: NavigationSettings;
  metadata: MetadataSettings;
  accessibility: AccessibilitySettings;
  library: Library;
}