export type GRID_LAYOUT = {
  width: number;
  height: number;
  gap: number;
}

export const IMAGE_FADE_OPTIONS = { delay: 0, duration: 500 };

export const GRID_LAYOUTS: Record<string, GRID_LAYOUT> = {
  portrait: {
    width: 162,
    height: 242,
    gap: 30,
  },
  sgdbGrid: {
    width: 132,
    height: 197,
    gap: 15,
  },
  landscape: {
    width: 370,
    height: 174,
    gap: 30,
  },
  sgdbHero: {
    width: 353,
    height: 114,
    gap: 30,
  }
  // hero: {
  //   width: 353,
  //   height: 114,
  //   gap: 30,
  // }
}

export const DETAILS_ASPECT_RATIO = 1.5;