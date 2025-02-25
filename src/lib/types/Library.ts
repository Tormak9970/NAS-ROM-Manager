import type { DownloadStrategy } from "./DownloadStrategy";

export type SystemTagConfig = {
  backgroundColor: string;
  borderColor: string;
}

export type System = {
  fullName: string;
  abbreviation: string;
  tagConfig: SystemTagConfig;
}

export type ROMCustomization = {
  path: string;
  title: string;
  coverPath: string;
  thumbPath: string;
  sgdbId: string;
  igdbId: string;
  metadata: Record<string, string>;
  isFavorite: boolean;
}

export type ROM = {
  title: string;
  path: string;
  size: number;
  format: string;
  system: string;
  systemFullName: string;
  addDate: string;
  downloadStrategy: DownloadStrategy;
}

export type LoadResult = {
  library: Library;
  roms: ROM[];
  romCustomizations: ROMCustomization[];
  systems: System[];
}

export type Library = {
  libraryPath: string;
  romDir: string;
  emulatorDir: string;
  biosDir: string;
}

export type Collection = {
  name: string;
  romsIds: string[];
}