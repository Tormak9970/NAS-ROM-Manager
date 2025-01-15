export const SYSTEMS: string[] = [
  "Xbox",
  "Xbox 360",
  "NES",
  "SNES",
  "N64",
  "GC",
  "GB",
  "GBA",
  "GBC",
  "NDS",
  "N3DS",
  "Wii",
  "WiiU",
  "Switch",
  "PS",
  "PS2",
  "PSVita",
  "PS3",
  "PSP",
]

export type ROMCustomization = {
  /**
   * Path will act like the ID of the ROM.
   */
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
}

export type LoadedLibrary = {
  library: Library;
  ROMs: ROM[];
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
  roms: ROM[];
}