import type { IconifyIcon } from "@iconify/types";
import { Dashboard, DashboardOutline, GameAsset, GameAssetOutline, Host, HostOutline, Library, LibraryOutline, Search, Settings, SettingsOutline } from "./icons";

export type View = {
  icon: IconifyIcon;
  selectedIcon: IconifyIcon;
  label: string;
  path: string;
}

export const desktopViews: View[] = [
  { label: "Dashboard", path: "/dashboard", selectedIcon: Dashboard, icon: DashboardOutline },
  { label: "Library", path: "/library", selectedIcon: Library, icon: LibraryOutline },
  { label: "Systems", path: "/systems", selectedIcon: GameAsset, icon: GameAssetOutline },
  { label: "Emulators", path: "/emulators", selectedIcon: Host, icon: HostOutline },
  { label: "Settings", path: "/settings", selectedIcon: Settings, icon: SettingsOutline },
]

export const mobileViews: View[] = [
  { label: "Dashboard", path: "/dashboard", selectedIcon: Dashboard, icon: DashboardOutline },
  { label: "Library", path: "/library", selectedIcon: Library, icon: LibraryOutline },
  { label: "Search", path: "/search", selectedIcon: Search, icon: Search },
  { label: "Systems", path: "/systems", selectedIcon: GameAsset, icon: GameAssetOutline },
  { label: "Settings", path: "/settings", selectedIcon: Settings, icon: SettingsOutline },
]