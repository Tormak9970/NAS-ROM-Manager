import type { DownloadStrategy } from "./DownloadStrategy";
import type { IGDBGame } from "./IGDB";

export type SystemTagConfig = {
  backgroundColor: string;
  borderColor: string;
}

export type System = {
  fullName: string;
  abbreviation: string;
  folder: string;
  sgdbId: string;
  coverPath: string;
  thumbPath: string;
  igdbPlatformId: string;
  tagConfig: SystemTagConfig;
}

export type ROMMetadata = {
  title: string;
  coverPath: string;
  thumbPath: string;
  sgdbId: string;
  igdbId: string;
  metadata: IGDBGame | null;
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