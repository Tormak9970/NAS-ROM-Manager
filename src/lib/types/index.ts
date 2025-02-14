export * from "./DownloadStrategy";
export * from "./Library";
export * from "./Settings";
export * from "./SGDB";

export type RomUploadConfig = {
  library: string;
  system: string;
  file: File;
  needsUnzip: boolean;
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