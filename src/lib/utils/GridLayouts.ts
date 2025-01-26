export type GRID_LAYOUT = {
  width: number;
  height: number;
  gap: number;
}

export const GRID_LAYOUTS: Record<string, GRID_LAYOUT> = {
  portrait: {
    width: 150,
    height: 250,
    gap: 30,
  },
  landscape: {
    width: 400,
    height: 300,
    gap: 30,
  }
}