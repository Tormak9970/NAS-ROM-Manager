import type { IconifyIcon } from "@iconify/types";
import { Dashboard, DashboardOutline, GameAsset, GameAssetOutline, Library, LibraryOutline, Search, Settings, SettingsOutline } from "./icons";

export type Route = {
  icon: IconifyIcon;
  selectedIcon: IconifyIcon;
  label: string;
  path: string;
}

export const routes: Record<string, Route> = {
  "Dashboard": {
    label: "Dashboard",
    path: "/app/dashboard",
    selectedIcon: Dashboard,
    icon: DashboardOutline
  },
  "Library": {
    label: "Library",
    path: "/app/library",
    selectedIcon: Library,
    icon: LibraryOutline
  },
  "Systems": {
    label: "Systems",
    path: "/app/systems",
    selectedIcon: GameAsset,
    icon: GameAssetOutline
  },
  // "Emulators": {
  //   label: "Emulators",
  //   path: "/app/emulators",
  //   selectedIcon: Host,
  //   icon: HostOutline
  // },
  "Settings": {
    label: "Settings",
    path: "/app/settings",
    selectedIcon: Settings,
    icon: SettingsOutline 
  },
  "Search": {
    label: "Search",
    path: "/app/search",
    selectedIcon: Search,
    icon: Search
  },
}