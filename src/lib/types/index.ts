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

export type FilePickerFilter = RegExp | ((file: File) => boolean);

export enum FileSelectionType {
  FILE,
  FOLDER,
}

export type FilePickerConfig = {
  select: FileSelectionType;
  startPath: string;
  showFiles?: boolean;
  filter?: FilePickerFilter;
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