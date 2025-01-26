export type GRID_LAYOUT = {
  width: number;
  height: number;
  gap: number;
}

const TITLE_SPACE = 48;

export const GRID_LAYOUTS: Record<string, GRID_LAYOUT> = {
  portrait: {
    width: 160,
    height: 225 + TITLE_SPACE,
    gap: 30,
  },
  landscape: {
    width: 368,
    height: 172 + TITLE_SPACE,
    gap: 30,
  }
}