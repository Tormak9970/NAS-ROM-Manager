import type { IconifyIcon } from "@iconify/types"
import { Dashboard, Settings, Library, Assignment, DashboardOutline, LibraryOutline, AssignmentOutline, SettingsOutline, GameAsset, GameAssetOutline } from "./icons";

export type View = {
  icon: IconifyIcon;
  selectedIcon: IconifyIcon;
  label: string;
  path: string;
}

export const views: View[] = [
  { label: "Dashboard", path: "/dashboard", selectedIcon: Dashboard, icon: DashboardOutline },
  { label: "Library", path: "/library", selectedIcon: Library, icon: LibraryOutline },
  { label: "Systems", path: "/systems", selectedIcon: GameAsset, icon: GameAssetOutline },
  { label: "Logs", path: "/logs", selectedIcon: Assignment, icon: AssignmentOutline },
  { label: "Settings", path: "/settings", selectedIcon: Settings, icon: SettingsOutline },
]