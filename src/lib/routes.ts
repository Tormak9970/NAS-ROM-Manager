import type { IconifyIcon } from "@iconify/types";
import { Assignment, AssignmentOutline, Dashboard, DashboardOutline, GameAsset, GameAssetOutline, Library, LibraryOutline, Search, Settings, SettingsOutline } from "./icons";

export type View = {
  icon: IconifyIcon;
  selectedIcon: IconifyIcon;
  label: string;
  path: string;
}

export const desktopViews: View[] = [
  { label: "Dashboard", path: "/dashboard", selectedIcon: Dashboard, icon: DashboardOutline },
  { label: "Library", path: "/library", selectedIcon: Library, icon: LibraryOutline },
  { label: "Emulators", path: "/emulators", selectedIcon: GameAsset, icon: GameAssetOutline },
  { label: "Docs", path: "/docs", selectedIcon: Assignment, icon: AssignmentOutline },
  { label: "Settings", path: "/settings", selectedIcon: Settings, icon: SettingsOutline },
]

export const mobileViews: View[] = [
  { label: "Dashboard", path: "/dashboard", selectedIcon: Dashboard, icon: DashboardOutline },
  { label: "Library", path: "/library", selectedIcon: Library, icon: LibraryOutline },
  { label: "Search", path: "/search", selectedIcon: Search, icon: Search },
  { label: "Emulators", path: "/emulators", selectedIcon: GameAsset, icon: GameAssetOutline },
  { label: "Settings", path: "/settings", selectedIcon: Settings, icon: SettingsOutline },
]