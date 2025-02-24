import { library, showWarningSnackbar } from "@stores/State";
import { BackendErrorType, type GridResults, type ROM, type RomUploadConfig, type SGDBGame } from "@types";
import { hash64, showError } from "@utils";
import streamSaver from "streamsaver";
import { get } from "svelte/store";
import { LogController } from "./LogController";

type ROMDownload = {
  path: string,
  parent: string,
}

type ROMUploadComplete = {
  uploadId: string;
  path: string;
  libraryPath: string;
  system: string;
  unzip: boolean;
}

/**
 * The Rest api controller.
 */
export class RestController {
  private static readonly STREAM_CHUNK_SIZE = 10 * 1024 * 1024;

  private static restURL = "http://127.0.0.1:1500/rest";


  /**
   * Deletes the cover for a title.
   * @param url The url of the cover to delete.
   * @param id The id of the title whose cover is being deleted.
   * @returns Whether the cover was successfully deleted.
   */
  static async deleteCover(url: string, id: string): Promise<boolean> {
    const res = await fetch(this.restURL + `/covers/${id}`, {
      method: "DELETE",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json",
        "Cover-Extension": url.substring(url.lastIndexOf(".") + 1)
      },
    });

    if (res.ok) {
      return true;
    } else {
      LogController.error(`Failed to delete cover ${url}:`, res.statusText);
      return false;
    }
  }

  /**
   * Caches the cover for a title.
   * @param url The url of the cover to cache.
   * @param id The id of the title whose cover is being cached.
   * @returns The path to the cached cover.
   */
  static async cacheCover(url: string, id: string): Promise<string> {
    const res = await fetch(this.restURL + `/covers/${id}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        url: url,
        ext: url.substring(url.lastIndexOf(".") + 1),
        timeout: 5000,
      })
    });

    if (res.ok) {
      return await res.text();
    } else {
      LogController.error(`Failed to cache cover ${url}:`, res.statusText);
      return "";
    }
  }


  private static async getMetadata(data: ROMDownload): Promise<{ size: number, path: string }> {
    const res = await fetch(this.restURL + `/roms/download/metadata?romPath=${encodeURIComponent(data.path)}&romParent=${encodeURIComponent(data.parent)}`, {
      method: "GET",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
      }
    });

    if (res.ok) {
      return await res.json();
    } else {
      LogController.error(`Failed to get metadata for ${data.path}:`, res.statusText);
      return { size: 0, path: "" };
    }
  }

  private static async notifyDownloadComplete(data: ROMDownload) {
    const res = await fetch(this.restURL + "/roms/download/complete", {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      return await res.text();
    } else {
      LogController.error(`Failed to notify the backend of the completed download for ${data.path}:`, res.statusText);
      return "";
    }
  }

  private static async streamDownload(path: string, fileSize: number, onProgress: (progress: number) => void) {
    let downloaded = 0;

    const backslashIndex = path.lastIndexOf("\\");
    const slashIndex = path.lastIndexOf("/");
    const startIndex = backslashIndex > slashIndex ? backslashIndex : slashIndex;
    const filename = path.substring(startIndex + 1);
    const fileStream = streamSaver.createWriteStream(filename);
    const writer = fileStream.getWriter();

    while (downloaded < fileSize) {
      const end = Math.min(downloaded + this.STREAM_CHUNK_SIZE - 1, fileSize - 1);
      const range = `bytes=${downloaded}-${end}`;
      const length = end - downloaded;

      const response = await fetch(this.restURL + `/roms/download?romPath=${encodeURIComponent(path)}`, {
        headers: {
          "Range": range,
        }
      });

      if (!response.ok) {
        throw new Error("Failed to fetch the chunk");
      }

      const reader = response.body!.getReader();
      let result;
      while (!(result = await reader.read()).done) {
        writer.write(result.value);
      }

      downloaded += length;
      onProgress(downloaded);
    }

    writer.close();
  }

  /**
   * Downloads the requested rom.
   * @param rom The rom to download.
   * @param onStart Function to run on start.
   * @param onProgress Function to run on chunk update.
   * @param onEnd Function to run on download complete.
   */
  static async downloadRom(
    rom: ROM,
    onStart: (fileSize: number) => void = () => {},
    onProgress: (progress: number) => void = () => {},
    onEnd: () => void = () => {}
  ): Promise<void> {
    const romDownloadConfig = {
      path: rom.path,
      parent: rom.downloadStrategy.type === "folder" ? rom.downloadStrategy.parent : "",
    }

    const { size, path } = await this.getMetadata(romDownloadConfig);
    romDownloadConfig.path = path;
    onStart(size);


    await this.streamDownload(path, size, onProgress);
    

    await this.notifyDownloadComplete(romDownloadConfig);
    onEnd();
  }


  private static async prepareUpload(libraryPath: string, romsDir: string, system: string, filename: string): Promise<string> {
    const filePath = `${libraryPath}/${romsDir}/${system}/${filename}`;

    const res = await fetch(this.restURL + `/roms/upload/prepare?romPath=${encodeURIComponent(filePath)}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
      }
    });

    if (res.ok) {
      return filePath;
    } else {
      LogController.error(`Failed to prepare upload for ${filePath}:`, res.statusText);
      return "";
    }
  }

  private static async uploadComplete(data: ROMUploadComplete) {
    const res = await fetch(this.restURL + "/roms/upload/complete", {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      return await res.text();
    } else {
      LogController.error(`Failed to notify the backend of the completed download for ${data.path}:`, res.statusText);
      return "";
    }
  }

  private static async streamUpload(uploadId: string, path: string, file: File, onProgress: (progress: number) => void) {
    let sent = 0;

    const fileSize = file.size;

    while (sent < fileSize) {
      const end = Math.min(sent + this.STREAM_CHUNK_SIZE - 1, fileSize - 1);
      const range = `bytes=${sent}-${end}`;
      const length = end - sent + 1;

      const data = file.slice(sent, end + 1);

      const response = await fetch(this.restURL + `/roms/upload?romPath=${encodeURIComponent(path)}`, {
        method: "POST",
        mode: "cors",
        headers: {
          "Range": range,
          "Content-Length": length.toString(),
          "Upload-Id": uploadId,
          "File-Size": fileSize.toString(),
          "Content-Type": "application/octet-stream"
        },
        body: data
      });

      if (!response.ok) {
        throw new Error("Failed to send the chunk");
      }

      sent += length;
      onProgress(sent);
    }
  }

  /**
   * Uploads a rom to the server.
   * @param rom The rom to upload.
   * @param filename The filename of the rom.
   * @param reader The reader stream for the file input.
   * @param needsUnzip Whether this rom will need to be unzipped.
   * @param onStart Function to run on start.
   * @param onProgress Function to run on chunk update.
   * @param onEnd Function to run on upload complete.
   */
  static async uploadRom(
    uploadConfig: RomUploadConfig,
    onStart: () => void = () => {},
    onProgress: (progress: number) => void = () => {},
    onEnd: (success: boolean, filePath: string) => void = () => {}
  ) {
    const { system, file, needsUnzip } = uploadConfig;
    const lib = get(library);
    
    const filePath = await this.prepareUpload(lib.libraryPath, lib.romDir, system, file.name);
    onStart();

    const uploadId = hash64(filePath);

  
    await this.streamUpload(
      uploadId,
      filePath,
      file,
      onProgress
    );


    const finalPath = await this.uploadComplete({
      uploadId: uploadId,
      path: filePath,
      libraryPath: lib.libraryPath,
      system: system,
      unzip: needsUnzip,
    });
    onEnd(finalPath !== "", finalPath);
  }
  
  /**
   * Deletes a ROM from the server.
   * @param rom The rom to delete.
   * @returns Whether the delete was successful.
   */
  static async deleteRom(rom: ROM): Promise<boolean> {
    const res = await fetch(this.restURL + `/roms/delete?romPath=${encodeURIComponent(rom.path)}`, {
      method: "DELETE",
      mode: "cors",
      headers: {
        "Accept": "text/plain, */*"
      },
    });

    if (res.ok) {
      return true;
    } else {
      LogController.error(`Failed to delete rom ${rom.path}:`, res.statusText);
      return false;
    }
  }
  
  
  /**
   * Initializes the SGDB Client.
   * @returns True if the client was initialized.
   */
  static async initSGDBClient(): Promise<boolean> {
    const res = await fetch(this.restURL + "/proxy/sgdb/init", {
      method: "POST",
      mode: "cors",
    });

    if (res.ok) {
      return true;
    } else {
      showError(
        "No environment variable SGDB_API_KEY was found",
        "Please check your container to ensure SGDB_API_KEY is set",
        BackendErrorType.PANIC
      );

      return false;
    }
  }

  /**
   * Gets the SGDB grids for the given game and page.
   * @param id The id of the game to get grids for.
   * @param page The results page.
   * @returns The list of grids.
   */
  static async getSGDBGridsById(id: string, page: number): Promise<GridResults> {
    const res = await fetch(this.restURL + "/proxy/sgdb/grids", {
      headers: {
        "SGDB-Game-Id": id,
        "SGDB-Results-Page": page.toString(),
      }
    });

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting grids from SGDB."})

      return {
        images: [],
        total: 0,
        page: page,
      };
    }
  }
  
  /**
   * Searches SGDB for games matching the query.
   * @param query The query to search for.
   * @returns The search results.
   */
  static async searchSGDBForTitle(query: string): Promise<SGDBGame[]> {
    const res = await fetch(this.restURL + `/proxy/sgdb/search?query=${encodeURIComponent(query)}`);

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting SGDB id."})

      return [];
    }
  }
}