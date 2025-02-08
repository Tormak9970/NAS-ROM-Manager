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