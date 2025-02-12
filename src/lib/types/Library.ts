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
  heroPath: string;
  gridPath: string;
  isFavorite: boolean;
}

export type ROM = {
  title: string;
  path: string;
  size: number;
  format: string;
  library: string;
  system: string;
  systemFullName: string;
  addDate: string;
  downloadStrategy: DownloadStrategy;
}

export type LoadedLibrary = {
  library: Library;
  roms: ROM[];
  systems: System[];
}

/**
 * TODO: eventually add support for emulators, tools, and bios files
 */
export type Library = {
  name: string;
  path: string;
  useProvidedParsers: boolean;
  parsersPath: string;
  romCustomizations: ROMCustomization[];
}

export type Collection = {
  name: string;
  romsIds: string[];
}