import { SGDB } from "@models";
import type { DownloadStrategy, ROM, SGDBImage } from "@types";
import { LogController } from "./utils/LogController";
import { RustInterop } from "./utils/RustInterop";

type ROMDownload = {
  path: string,
  downloadStrategy: DownloadStrategy,
}

/**
 * The api controller.
 */
export class ApiController {
  private static client: SGDB;

  private static restURL = "http://127.0.0.1:1500/rest"

  /**
   * Initializes the api controller.
   */
  static async init(): Promise<void> {
    const key = await RustInterop.getSGDBKey();

    this.client = new SGDB(key);
  }

  /**
   * Caches the cover for a title.
   * @param url The url of the cover to cache.
   * @param id The id of the title whose cover is being cached.
   * @returns The path to the cached cover.
   */
  static async deleteCover(url: string, id: string): Promise<string> {
    const res = await fetch(this.restURL + `/covers/${id}`, {
      method: 'DELETE',
      mode: 'cors',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json',
        'Cover-Extension': url.substring(url.lastIndexOf(".") + 1)
      },
    });

    if (res.ok) {
      return await res.text();
    } else {
      LogController.error(`Failed to cache cover ${url}:`, res.statusText);
      return "";
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
      method: 'POST',
      mode: 'cors',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json'
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


  private static async getMetadata(data: ROMDownload) {
    const res = await fetch(this.restURL + "/roms/metadata", {
      method: 'POST',
      mode: 'cors',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      return await res.text();
    } else {
      LogController.error(`Failed to get metadata for ${data.path}:`, res.statusText);
      return "";
    }
  }

  private static async notifyDownloadComplete(data: ROMDownload) {
    const res = await fetch(this.restURL + "/roms/complete", {
      method: 'POST',
      mode: 'cors',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json'
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
    onStart: () => void = () => {},
    onProgress: (progress: number) => void = () => {},
    onEnd: () => void = () => {}
  ): Promise<void> {
    const romDownloadConfig = {
      path: rom.path,
      downloadStrategy: rom.downloadStrategy,
    }

    const adjustedFilePath = await this.getMetadata(romDownloadConfig);
    romDownloadConfig.path = adjustedFilePath;
    onStart();

    // TODO: download the rom in chunks

    await this.notifyDownloadComplete(romDownloadConfig);
    onEnd();
  }

  // ! Possible method for handling the rom downloads
  // async function downloadFile(url, fileSize) {
  //     const chunkSize = 5 * 1024 * 1024;  // 5MB per chunk
  //     let downloaded = 0;

  //     const fileStream = streamSaver.createWriteStream('largefile.dat'); // Using the StreamSaver library for saving to disk
  //     const writer = fileStream.getWriter();

  //     while (downloaded < fileSize) {
  //         const end = Math.min(downloaded + chunkSize - 1, fileSize - 1);
  //         const range = `bytes=${downloaded}-${end}`;

  //         const response = await fetch(url, {
  //             headers: { 'Range': range }
  //         });

  //         if (!response.ok) {
  //             throw new Error('Failed to fetch the chunk');
  //         }

  //         const reader = response.body.getReader();
  //         let result;
  //         while (!(result = await reader.read()).done) {
  //             writer.write(result.value);
  //         }

  //         downloaded += chunkSize;
  //         console.log(`Downloaded ${downloaded} of ${fileSize} bytes.`);
  //     }

  //     writer.close();
  // TODO: make a request to the server telling it the download is complete, and to cleanup as needed
  // }

  // async function getFileSize(url) {
  //     const response = await fetch(url, { method: 'HEAD' });
  //     if (!response.ok) throw new Error('Failed to fetch file size');
  //     return parseInt(response.headers.get('Content-Length'));
  // }

  // // Usage
  // const url = 'http://localhost:3030/download/largefile';
  // const fileSize = await getFileSize(url);
  // downloadFile(url, fileSize);
  
  /**
   * Fetches the SGDB covers for the provided title.
   * @param title The title to fetch the covers for.
   * @returns The fetched covers.
   */
  static async getCoversForGame(title: string): Promise<SGDBImage[]> {
    return []
  }
}