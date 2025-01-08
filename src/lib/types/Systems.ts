export type System = {
  name: string;
  abr: string;
  formats: string[];
}

export const SYSTEMS: Record<string, System> = {
  "Xbox": {
    name: "Microsoft Xbox",
    abr: "Xbox",
    formats: []
  },
  "Xbox 360": {
    name: "Microsoft Xbox 360",
    abr: "Xbox 360",
    formats: []
  },
  "NES": {
    name: "Nintendo Entertainment System",
    abr: "NES",
    formats: []
  },
  "SNES": {
    name: "Super Nintendo Entertainment System",
    abr: "SNES",
    formats: []
  },
  "N64": {
    name: "Nintendo 64",
    abr: "N64",
    formats: []
  },
  "GC": {
    name: "Nintendo GameCube",
    abr: "GC",
    formats: []
  },
  "GB": {
    name: "Nintendo GameBoy",
    abr: "GB",
    formats: []
  },
  "GBA": {
    name: "Nintendo GameBoy Advanced",
    abr: "GBA",
    formats: []
  },
  "GBC": {
    name: "Nintendo GameBoy Color",
    abr: "GBC",
    formats: []
  },
  "Wii": {
    name: "Nintendo Wii",
    abr: "Wii",
    formats: []
  },
  "WiiU": {
    name: "Nintendo WiiU",
    abr: "WiiU",
    formats: []
  },
  "Switch": {
    name: "Nintendo Switch",
    abr: "Switch",
    formats: []
  },
  "PS": {
    name: "Sony PlaySation",
    abr: "PS",
    formats: []
  },
  "PS2": {
    name: "Sony PlaySation 2",
    abr: "PS2",
    formats: []
  },
  "PSV": {
    name: "Sony PlaySation Vita",
    abr: "PSV",
    formats: []
  },
  "PS3": {
    name: "Sony PlaySation 3",
    abr: "PS3",
    formats: []
  },
  "PSP": {
    name: "Sony PlaySation Portable",
    abr: "PSP",
    formats: []
  },
}