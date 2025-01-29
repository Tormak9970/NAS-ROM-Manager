export type SystemTagConfig = {
  backgroundColor: string;
  borderColor: string;
}

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

/**
 * Color config for the different systems.
 */
export const SYSTEM_TAGS_CONFIG: Record<string, SystemTagConfig> = {
  nes: {
    backgroundColor: "165 165 165",
    borderColor: "165 165 165",
  },
  snes: {
    backgroundColor: "255 51 36",
    borderColor: "255 51 36",
  },
  n64: {
    backgroundColor: "160 125 156",
    borderColor: "160 125 156",
  },
  gb: {
    backgroundColor: "86 44 6",
    borderColor: "86 44 6",
  },
  gba: {
    backgroundColor: "112 87 255",
    borderColor: "112 87 255",
  },
  gbc: {
    backgroundColor: "217 126 229",
    borderColor: "217 126 229",
  },
  nds: {
    backgroundColor: "205 216 53",
    borderColor: "205 216 53",
  },
  n3ds: {
    backgroundColor: "240 143 57",
    borderColor: "240 143 57",
  },
  gc: {
    backgroundColor: "123 67 226",
    borderColor: "123 67 226",
  },
  wii: {
    backgroundColor: "207 207 207",
    borderColor: "207 207 207",
  },
  wiiu: {
    backgroundColor: "0 228 212",
    borderColor: "0 228 212",
  },
  switch: {
    backgroundColor: "228 0 15",
    borderColor: "228 0 15",
  },
  ps: {
    backgroundColor: "165 165 165",
    borderColor: "165 165 165",
  },
  ps2: {
    backgroundColor: "0 194 165",
    borderColor: "0 194 165",
  },
  ps3: {
    backgroundColor: "0 117 202",
    borderColor: "0 117 202",
  },
  psp: {
    backgroundColor: "15 15 15",
    borderColor: "15 15 15",
  },
  psvita: {
    backgroundColor: "1 30 98",
    borderColor: "1 30 98",
  },
  xbox: {
    backgroundColor: "50 104 18",
    borderColor: "50 104 18",
  },
  xbox360: {
    backgroundColor: "105 217 38",
    borderColor: "105 217 38",
  },
};