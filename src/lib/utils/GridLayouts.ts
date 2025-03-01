export type GRID_LAYOUT = {
  width: number;
  height: number;
  gap: number;
}

export const GRID_LAYOUTS: Record<string, GRID_LAYOUT> = {
  portrait: {
    width: 162,
    height: 242,
    gap: 30,
  },
  landscape: {
    width: 370,
    height: 174,
    gap: 30,
  }
}

export const DETAILS_ASPECT_RATIO = 1.5;