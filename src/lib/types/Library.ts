export type ROM = {
  name: string;
  path: string;
  heroPath: string;
  gridPath: string;
  format: string;
  system: string;
  isFavorite: boolean;
}

export type Library = {
  name: string;
  path: string;
  roms: ROM[];
}

export type Collection = {
  name: string;
  roms: ROM[];
}