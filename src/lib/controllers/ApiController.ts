import { SGDB } from "@models";
import type { SGDBImage } from "@types";
import { RustInterop } from "./utils/RustInterop";

/**
 * The api controller.
 */
export class ApiController {
  private static client: SGDB;

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
  private static async cacheCover(url: string, id: string): Promise<string> {
    return "";
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