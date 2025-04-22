import { library, showInfoSnackbar, showWarningSnackbar } from "@stores/State";
import { BackendErrorType, type GridResults, type IGDBGame, type IGDBMetadataPlatform, type IGDBSearchResult, type ROM, type RomUploadConfig, type SGDBGame } from "@types";
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

  private static readonly GRIDS_BASE_URL = "http://127.0.0.1:1500/rest/grids";
  private static readonly BASE_URL = "http://127.0.0.1:1500/rest";

  private static currentDownload: ReadableStreamDefaultReader<Uint8Array<ArrayBufferLike>> | null = null;
  static currentUploadId: string | null = null;

  /**
   * Deletes the capsule for a title.
   * @param fullCapsuleUrl The url of the full capsule to delete.
   * @param thumbCapsuleUrl The url of the thumb capsule to delete.
   * @param id The id of the title whose capsule is being deleted.
   * @returns Whether the capsule was successfully deleted.
   */
  static async deleteCapsule(fullCapsuleUrl: string, thumbCapsuleUrl: string, id: string): Promise<boolean> {
    const res = await fetch(this.BASE_URL + `/grids/capsules/${id}`, {
      method: "DELETE",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json",
        "Full-Capsule-Extension": fullCapsuleUrl.substring(fullCapsuleUrl.lastIndexOf(".") + 1),
        "Thumb-Capsule-Extension": thumbCapsuleUrl.substring(thumbCapsuleUrl.lastIndexOf(".") + 1)
      },
    });

    if (res.ok) {
      return true;
    } else {
      LogController.error(`Failed to delete capsule for ${id}:`, res.statusText);
      return false;
    }
  }

  /**
   * Caches the capsule for a title.
   * @param fullCapsuleUrl The url of the capsule to cache.
   * @param thumbCapsuleUrl The url of the capsule preview to cache.
   * @param id The id of the title whose capsule is being cached.
   * @returns The path to the cached capsule.
   */
  static async cacheCapsule(fullCapsuleUrl: string, thumbCapsuleUrl: string, id: string): Promise<[string, string]> {
    const res = await fetch(this.BASE_URL + `/grids/capsules/${id}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        fullCapsuleUrl: fullCapsuleUrl,
        fullCapsuleExt: fullCapsuleUrl.substring(fullCapsuleUrl.lastIndexOf(".") + 1),
        thumbCapsuleUrl: thumbCapsuleUrl,
        thumbCapsuleExt: thumbCapsuleUrl.substring(thumbCapsuleUrl.lastIndexOf(".") + 1),
        timeout: 5000,
      })
    });

    if (res.ok) {
      const images = await res.text();
      const [thumb, full] = images.split(",");
      
      const currentTime = new Date().getTime();

      return [
        `${this.GRIDS_BASE_URL}/thumb/${thumb}#${currentTime}`,
        `${this.GRIDS_BASE_URL}/full/${full}#${currentTime}`,
      ];
    } else {
      LogController.error(`Failed to cache capsule ${fullCapsuleUrl}:`, res.statusText);
      return ["", ""];
    }
  }

  
  /**
   * Deletes the hero for a title.
   * @param heroUrl The url of the hero to delete.
   * @param id The id of the title whose hero is being deleted.
   * @returns Whether the hero was successfully deleted.
   */
  static async deleteHero(heroUrl: string, id: string): Promise<boolean> {
    const res = await fetch(this.BASE_URL + `/grids/heroes/${id}`, {
      method: "DELETE",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json",
        "Hero-Extension": heroUrl.substring(heroUrl.lastIndexOf(".") + 1),
      },
    });

    if (res.ok) {
      return true;
    } else {
      LogController.error(`Failed to delete hero for ${id}:`, res.statusText);
      return false;
    }
  }

  /**
   * Caches the hero for a title.
   * @param heroUrl The url of the hero to cache.
   * @param id The id of the title whose hero is being cached.
   * @returns The path to the cached hero.
   */
  static async cacheHero(heroUrl: string, id: string): Promise<string> {
    const res = await fetch(this.BASE_URL + `/grids/heroes/${id}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        heroUrl: heroUrl,
        heroExt: heroUrl.substring(heroUrl.lastIndexOf(".") + 1),
        timeout: 5000,
      })
    });

    if (res.ok) {
      const image = await res.text();
      const currentTime = new Date().getTime();

      return `${this.GRIDS_BASE_URL}/hero/${image}#${currentTime}`;
    } else {
      LogController.error(`Failed to cache hero ${heroUrl}:`, res.statusText);
      return "";
    }
  }


  private static async getMetadata(data: ROMDownload): Promise<{ size: number, path: string }> {
    const res = await fetch(this.BASE_URL + `/roms/download/metadata?romPath=${encodeURIComponent(data.path)}&romParent=${encodeURIComponent(data.parent)}`, {
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

  private static async downloadNative(url: string, filename: string, onProgress: (progress: number) => void) {
    let downloaded = 0;

    const newHandle = await window.showSaveFilePicker({ suggestedName: filename });
    const writableStream = await newHandle.createWritable();

    const pageHideOptions = { capture: true };
    const onPageHideChange = () => writableStream.abort();

    window.onbeforeunload = () => {
      return "A download is in progress, are you sure you want to leave?";
    }
    window.addEventListener("pagehide", onPageHideChange, pageHideOptions);

    await fetch(url).then(async (response) => {
      const reader = response.body?.getReader();

      if (!reader) return;

      RestController.currentDownload = reader;
      
      while(true) {
        const {done, value} = await reader.read();
      
        if (done) {
          break;
        }

        if (!RestController.currentDownload) {
          writableStream.abort("User Canceled");
          break;
        }
        
        await writableStream.write(value);
      
        downloaded += value.length;
        onProgress(downloaded);
      }
    });
    
    await writableStream.close();
    
    window.onbeforeunload = null;
    window.removeEventListener("pagehide", onPageHideChange, pageHideOptions);
  }

  private static async downloadPolyfill(url: string, filename: string, fileSize: number, onProgress: (progress: number) => void) {
    let downloaded = 0;
    const fileStream = streamSaver.createWriteStream(filename, { size: fileSize });
    const writer = fileStream.getWriter();
    
    const pageHideOptions = { capture: true };
    const onPageHideChange = () => writer.abort();

    window.onbeforeunload = () => {
      return "A download is in progress, are you sure you want to leave?";
    }
    window.addEventListener("pagehide", onPageHideChange, pageHideOptions);

    await fetch(url).then(async (response) => {
      const reader = response.body?.getReader();

      if (!reader) return;

      RestController.currentDownload = reader;
      
      while(true) {
        const { done, value } = await reader.read();
      
        if (done) {
          break;
        }

        if (!RestController.currentDownload) {
          writer.abort("User Canceled");
          break;
        }
        
        await writer.write(value);
      
        downloaded += value.length;
        onProgress(downloaded);
      }
    });

    writer.close();
    
    window.onbeforeunload = null;
    window.removeEventListener("pagehide", onPageHideChange, pageHideOptions);
  }

  private static async streamDownload(path: string, fileSize: number, onProgress: (progress: number) => void) {
    const backslashIndex = path.lastIndexOf("\\");
    const slashIndex = path.lastIndexOf("/");
    const startIndex = backslashIndex > slashIndex ? backslashIndex : slashIndex;
    const filename = path.substring(startIndex + 1);
    
    const romURL = this.BASE_URL + `/roms/download?romPath=${encodeURIComponent(path)}`;

    // @ts-expect-error This error is because we have a type package installed. The File System API is still not supported in all browsers.
    // ? See https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream#browser_compatibility
    if (window.showSaveFilePicker) {
      await this.downloadNative(romURL, filename, onProgress);
    } else {
      await this.downloadPolyfill(romURL, filename, fileSize, onProgress);
    }
  }

  private static async notifyDownloadComplete(data: ROMDownload) {
    const res = await fetch(this.BASE_URL + "/roms/download/complete", {
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
    onEnd: (finished: boolean) => void = () => {}
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
    onEnd(!!RestController.currentDownload);

    RestController.currentDownload = null;
  }

  /**
   * Cancels the current download if it exists.
   */
  static async cancelDownload() {
    if (RestController.currentDownload) {
      RestController.currentDownload.cancel("User Canceled");
    } else {
      get(showWarningSnackbar)({ message: "There is no download currently" });
    }
  }


  private static async prepareUpload(libraryPath: string, romsDir: string, system: string, filename: string): Promise<string> {
    const filePath = `${libraryPath}/${romsDir}/${system}/${filename}`;

    const res = await fetch(this.BASE_URL + `/roms/upload/prepare?romPath=${encodeURIComponent(filePath)}`, {
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
    const res = await fetch(this.BASE_URL + "/roms/upload/complete", {
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
      if (!RestController.currentUploadId) break;

      const end = Math.min(sent + this.STREAM_CHUNK_SIZE - 1, fileSize - 1);
      const range = `bytes=${sent}-${end}`;
      const length = end - sent + 1;

      const data = file.slice(sent, end + 1);

      const response = await fetch(this.BASE_URL + `/roms/upload?romPath=${encodeURIComponent(path)}`, {
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

      if (!response.ok && RestController.currentUploadId) {
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
    RestController.currentUploadId = uploadId;

  
    await this.streamUpload(
      uploadId,
      filePath,
      file,
      onProgress
    );


    if (RestController.currentUploadId) {
      RestController.currentUploadId = null;

      const finalPath = await this.uploadComplete({
        uploadId: uploadId,
        path: filePath,
        libraryPath: lib.libraryPath,
        system: system,
        unzip: needsUnzip,
      });
      onEnd(finalPath !== "", finalPath);
    }
  }

  /**
   * Cancels the current upload if one exists.
   * @returns True if successful, false if not.
   */
  static async cancelUpload(): Promise<boolean> {
    if (RestController.currentUploadId) {
      const res = await fetch(this.BASE_URL + "/roms/upload/cancel", {
        method: "POST",
        mode: "cors",
        headers: {
          "Upload-Id": RestController.currentUploadId,
        },
      });
  
      if (res.ok) {
        get(showInfoSnackbar)({ message: "Upload canceled" });
        RestController.currentUploadId = null;
        return true;
      } else {
        LogController.error(`Failed to cancel the upload for ${RestController.currentUploadId}:`, res.statusText);
      }
    } else {
      get(showWarningSnackbar)({ message: "There is no upload currently" });
    }

    return false;
  }
  
  /**
   * Deletes a ROM from the server.
   * @param romPath The path of the rom to delete.
   * @returns Whether the delete was successful.
   */
  static async deleteRom(romPath: string): Promise<boolean> {
    const res = await fetch(this.BASE_URL + `/roms/delete?romPath=${encodeURIComponent(romPath)}`, {
      method: "DELETE",
      mode: "cors",
      headers: {
        "Accept": "text/plain, */*"
      },
    });

    if (res.ok) {
      return true;
    } else {
      LogController.error(`Failed to delete rom ${romPath}:`, res.statusText);
      return false;
    }
  }
  
  
  /**
   * Initializes the SGDB Client.
   * @returns True if the client was initialized.
   */
  static async initSGDBClient(): Promise<boolean> {
    const res = await fetch(this.BASE_URL + "/proxy/sgdb/init", {
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
   * @param gridType The type of the grids to get.
   * @returns The list of grids.
   */
  static async getSGDBGridsById(id: string, page: number, gridType: "grids" | "heroes"): Promise<GridResults> {
    const res = await fetch(this.BASE_URL + "/proxy/sgdb/grids", {
      headers: {
        "SGDB-Game-Id": id,
        "SGDB-Results-Page": page.toString(),
        "SGDB-Grid-Type": gridType
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
    const res = await fetch(this.BASE_URL + `/proxy/sgdb/search?query=${encodeURIComponent(query)}`);

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting SGDB id."})

      return [];
    }
  }
  
  
  /**
   * Initializes the IGDB Client.
   * @returns True if the client was initialized.
   */
  static async initIGDBClient(): Promise<boolean> {
    const res = await fetch(this.BASE_URL + "/proxy/igdb/init", {
      method: "POST",
      mode: "cors",
    });

    if (res.ok) {
      return true;
    } else {
      showError(
        "Environment variable IGDB_CLIENT_ID or IGDB_CLIENT_SECRET was missing",
        "Please check your container to ensure IGDB_CLIENT_ID and IGDB_CLIENT_SECRET are set",
        BackendErrorType.PANIC
      );

      return false;
    }
  }

  /**
   * Gets the IGDB metadata for the given game.
   * @param id The id of the game to get grids for.
   * @returns The list of grids.
   */
  static async getIGDBMetadataById(id: string): Promise<IGDBGame | null> {
    const res = await fetch(this.BASE_URL + "/proxy/igdb/metadata", {
      headers: {
        "IGDB-Game-Id": id,
      }
    });

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting metadata from IGDB."})

      return null;
    }
  }
  
  /**
   * Searches IGDB for games matching the query.
   * @param query The query to search for.
   * @returns The best match for the search.
   */
  static async searchIGDBForTitle(query: string, igdbPlatformId: string): Promise<IGDBSearchResult[]> {
    const res = await fetch(this.BASE_URL + `/proxy/igdb/search/games?query=${encodeURIComponent(query)}&platform-id=${igdbPlatformId}`);

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting IGDB id."})

      return [];
    }
  }
  
  /**
   * Searches IGDB for platforms matching the query.
   * @param query The query to search for.
   * @returns The best match for the search.
   */
  static async searchIGDBForPlatform(query: string): Promise<IGDBMetadataPlatform[]> {
    const res = await fetch(this.BASE_URL + `/proxy/igdb/search/platforms?query=${encodeURIComponent(query)}`);

    if (res.ok) {
      return await res.json();
    } else {
      get(showWarningSnackbar)({ message: "Error getting IGDB platforms."})

      return [];
    }
  }
}