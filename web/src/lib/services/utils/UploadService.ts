import { DialogService } from "@services/utils/DialogService";
import { library, showInfoSnackbar, showWarningSnackbar, systems } from "@stores/State";
import type { CompletedUploadData, UploadConfig } from "@types";
import { hash64 } from "@utils";
import { get } from "svelte/store";
import { LogService } from "./LogService";

export class UploadService {
  private static readonly STREAM_CHUNK_SIZE = 10 * 1024 * 1024;

  private static readonly BASE_URL = `http://${import.meta.env.NRM_SERVER_URL}/rest`;

  static currentUploadId: string | null = null;
  
  static async prepareReplace(filePath: string): Promise<string> {
    const res = await fetch(UploadService.BASE_URL + `/upload/replace/prepare?filePath=${encodeURIComponent(filePath)}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
      }
    });

    if (res.ok) {
      return filePath;
    } else {
      LogService.error(`Failed to prepare replace for ${filePath}:`, res.statusText);
      return "";
    }
  }

  static async prepareUpload(libraryPath: string, dir: string, system: string, filename: string, romId?: string): Promise<string> {
    const filePath = `${libraryPath}/${dir}/${system}${romId ? "/" + romId : ""}/${filename}`;

    const res = await fetch(UploadService.BASE_URL + `/upload/prepare?filePath=${encodeURIComponent(filePath)}`, {
      method: "POST",
      mode: "cors",
      headers: {
        "Accept": "application/json, text/plain, */*",
      }
    });

    if (res.ok) {
      const body = await res.text();
      const fileExists = body === "true";

      if (fileExists) {
        await DialogService.message(
          "Warning!",
          "A file with that filename already exists. Please rename the file and try again.",
          "Ok",
          true
        );

        return "canceled";
      }

      return filePath;
    } else {
      LogService.error(`Failed to prepare upload for ${filePath}:`, res.statusText);
      return "";
    }
  }
  
  private static async streamUpload(uploadId: string, path: string, file: File, onProgress: (progress: number) => void) {
    let sent = 0;

    const fileSize = file.size;

    while (sent < fileSize) {
      if (!UploadService.currentUploadId) break;

      const end = Math.min(sent + this.STREAM_CHUNK_SIZE - 1, fileSize - 1);
      const range = `bytes=${sent}-${end}`;
      const length = end - sent + 1;

      const data = file.slice(sent, end + 1);

      const response = await fetch(UploadService.BASE_URL + `/upload?filePath=${encodeURIComponent(path)}`, {
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

      if (!response.ok && UploadService.currentUploadId) {
        throw new Error("Failed to send the chunk");
      }

      sent += length;
      onProgress(sent);
    }
  }

  /**
   * Uploads a bios file to the server.
   * @param uploadConfig The bios upload config.
   * @param onStart Function to run on start.
   * @param onProgress Function to run on chunk update.
   * @param onComplete Function to run on upload complete.
   * @param onEnd Function to run on after the upload has fully finished.
   * @param isReplace If true, a file is being replaced.
   * @returns True if the upload was canceled by a duplicate filename, false if not.
   */
  static async upload(
    uploadConfig: UploadConfig,
    onStart: () => void = () => {},
    onProgress: (progress: number) => void = () => {},
    onComplete: (data: CompletedUploadData) => Promise<string>,
    onEnd: (success: boolean, filePath: string) => void = () => {},
    isReplace = false
  ) {
    const { uploadFolder, romId, file, system, needsUnzip } = uploadConfig;
    const lib = get(library);
    
    const systemFolder = get(systems)[system].folder;

    let filePath = "";

    if (isReplace) {
      filePath = await UploadService.prepareReplace(uploadConfig.path!);
    } else {
      filePath = await UploadService.prepareUpload(lib.libraryPath, uploadFolder, systemFolder, file.name, romId);
    }

    if (filePath === "canceled") {
      return true;
    }

    onStart();

    const uploadId = hash64(filePath);
    UploadService.currentUploadId = uploadId;

  
    await UploadService.streamUpload(
      uploadId,
      filePath,
      file,
      onProgress
    );


    if (UploadService.currentUploadId) {
      UploadService.currentUploadId = null;

      const finalPath = await onComplete({
        uploadId: uploadId,
        path: filePath,
        libraryPath: lib.libraryPath,
        system: systemFolder,
        unzip: needsUnzip,
      });

      onEnd(finalPath !== "", finalPath);
    }

    return false;
  }

  /**
   * Cancels the current upload if one exists.
   * @returns True if successful, false if not.
   */
  static async cancelUpload(): Promise<boolean> {
    if (UploadService.currentUploadId) {
      const res = await fetch(UploadService.BASE_URL + "/upload/cancel", {
        method: "POST",
        mode: "cors",
        headers: {
          "Upload-Id": UploadService.currentUploadId,
        },
      });
  
      if (res.ok) {
        get(showInfoSnackbar)({ message: "Upload canceled" });
        UploadService.currentUploadId = null;
        return true;
      } else {
        LogService.error(`Failed to cancel the upload for ${UploadService.currentUploadId}:`, res.statusText);
      }
    } else {
      get(showWarningSnackbar)({ message: "There is no upload currently" });
    }

    return false;
  }
}