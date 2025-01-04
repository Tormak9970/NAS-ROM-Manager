import type { IconifyIcon } from "@iconify/types"
import { History, Home, Monitoring, Settings, VideoLibrary } from "./icons";

export type View = {
  icon: IconifyIcon;
  label: string;
  path: string;
}

export const views: View[] = [
  { label: "Home", path: "/", icon: Home },
  { label: "Library", path: "/library", icon: VideoLibrary },
  { label: "Stats", path: "/stats", icon: Monitoring },
  { label: "Logs", path: "/logs", icon: History },
  { label: "Settings", path: "/settings", icon: Settings },
]