import type { DownloadStrategy } from "./DownloadStrategy";
import type { IGDBGame } from "./IGDB";

export type SystemTagConfig = {
  backgroundColor: string;
  borderColor: string;
}

export type ParserPattern = {
  glob: string;
  regex: string;
  downloadStrategy: DownloadStrategy;
}

export type System = {
  fullName: string;
  // ? Abbreviation is used to index the parsers on the backend.
  abbreviation: string;
  folder: string;
  sgdbId: string;
  coverPath: string;
  thumbPath: string;
  igdbPlatformId: string;
  tagConfig: SystemTagConfig;
  patterns: ParserPattern[];
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