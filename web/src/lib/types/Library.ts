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
  name: string;
  // ? Abbreviation is used to index the parsers on the backend.
  abbreviation: string;
  folder: string;
  sgdbId: string;
  fullCapsulePath: string;
  thumbCapsulePath: string;
  heroPath: string;
  igdbPlatformId: string;
  tagConfig: SystemTagConfig;
  patterns: ParserPattern[];
  biosFiles: string[];
}

export type ROMMetadata = {
  title: string;
  fullCapsulePath: string;
  thumbCapsulePath: string;
  heroPath: string;
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
  dlcs: Record<string, string[]>;
  updates: Record<string, string[]>;
}

export type Library = {
  libraryPath: string;
  romDir: string;
  emulatorDir: string;
  biosDir: string;
  dlcDir: string;
  updateDir: string;
}

export type Collection = {
  name: string;
  romsIds: string[];
}