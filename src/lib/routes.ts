import type { IconifyIcon } from "@iconify/types";
import { Dashboard, DashboardOutline, GameAsset, GameAssetOutline, Host, HostOutline, Library, LibraryOutline, Search, Settings, SettingsOutline } from "./icons";

export type Route = {
  icon: IconifyIcon;
  selectedIcon: IconifyIcon;
  label: string;
  path: string;
}

export const routes: Record<string, Route> = {
  "Dashboard": {
    label: "Dashboard",
    path: "/dashboard",
    selectedIcon: Dashboard,
    icon: DashboardOutline
  },
  "Library": {
    label: "Library",
    path: "/library",
    selectedIcon: Library,
    icon: LibraryOutline
  },
  "Systems": {
    label: "Systems",
    path: "/systems",
    selectedIcon: GameAsset,
    icon: GameAssetOutline
  },
  "Emulators": {
    label: "Emulators",
    path: "/emulators",
    selectedIcon: Host,
    icon: HostOutline
  },
  "Settings": {
    label: "Settings",
    path: "/settings",
    selectedIcon: Settings,
    icon: SettingsOutline 
  },
  "Search": {
    label: "Search",
    path: "/search",
    selectedIcon: Search,
    icon: Search
  },
}