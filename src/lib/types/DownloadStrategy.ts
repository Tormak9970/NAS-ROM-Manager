
export type SingleFileStrategy = {
  type: "single-file";
}

export type FolderStrategy = {
  type: "folder";
  parent: string;
}

export type DownloadStrategy = SingleFileStrategy | FolderStrategy;