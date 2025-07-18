import type { Snippet } from "svelte";

export * from "./DownloadStrategy";
export * from "./IGDB";
export * from "./Library";
export * from "./Settings";
export * from "./SGDB";

export enum ExtraFileType {
  DLC = "dlc",
  UPDATE = "update"
}

export type UploadConfig = {
  uploadFolder: string;
  system: string;
  file: File;
  path?: string;
  needsUnzip: boolean;
  romId?: string;
}

export type CompletedUploadData = {
  uploadId: string;
  path: string;
  libraryPath: string;
  system: string;
  unzip: boolean;
}

export type UploadSettings = {
  config: UploadConfig;
  complete: (data: CompletedUploadData) => Promise<string>;
  process: (
    filePath: string,
    closeModal: () => void
  ) => Promise<void>;
  isReplace?: boolean;
}

export type AvailableStorage = {
  usedSpace: number;
  totalSpace: number;
}

export type FilePickerFilter = RegExp | ((file: FilePickerEntry) => boolean);

/**
 * Checks if a filePickerFilter is a RegEx.
 * @param filter The filter to check.
 * @returns True if the filter is a RegEx.
 */
export function isRegEx(filter: FilePickerFilter): filter is RegExp {
  return (filter as RegExp).test !== undefined;
}

export type FilePickerEntry = {
  path: string;
  name: string;
  isDir: boolean;
}

export enum FileSelectionType {
  FILE,
  FOLDER,
}

export type FilePickerConfig = {
  select: FileSelectionType;
  startPath: string;
  showFiles?: boolean;
  filter?: FilePickerFilter;
  /**
   * Extensions must be formatted like "svelte" instead of ".svelte".
   */
  extensions?: string[];
  showHiddenFiles?: boolean;
  max?: number;
}

export type BackendError = {
  message: string;
  fix: string;
  type: BackendErrorType;
}

export enum BackendErrorType {
  WARN,
  PANIC,
}

export type LoadGuardProps = {
  message?: string;
  onLoad?: () => Promise<void>;
  children: Snippet<[]>;
}

export type SearchQuery = {
  textQuery?: string;
  system?: string;
  startReleaseDate?: string;
  endReleaseDate?: string;
  genre?: string;
  publisher?: string;
  developer?: string;
  format?: string;
}