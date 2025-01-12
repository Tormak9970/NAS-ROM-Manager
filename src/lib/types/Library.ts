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

export type ROM = {
  name: string;
  path: string;
  heroPath: string;
  gridPath: string;
  format: string;
  library: string;
  system: string;
  isFavorite: boolean;
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
  romCustomizations: ROM[];
}

export type Collection = {
  name: string;
  roms: ROM[];
}