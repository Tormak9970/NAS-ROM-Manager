/**
 * Copyright (C) 2025 Travis Lane (Tormak)
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>
 */

import { library, roms, romsBySystem, showWarningSnackbar, systems } from "@stores/State";
import { BackendErrorType, type AvailableStorage, type BackendError, type FilePickerConfig, type FilePickerEntry, type Library, type LoadResult, type ROM, type ROMMetadata, type Settings } from "@types";
import { hash64, showError, systemToParser } from "@utils";
import { get } from "svelte/store";
import { LogController } from "./LogController";

type Response<T> = { data: T }

/**
 * Handles wrapping websocket communication into an easy to use JS bindings.
 */
export class WebsocketController {
  private static ws: WebSocket;
  private static hash: string;

  /**
   * Initializes the Rust <-> Svelte communication.
   * @param onOpen The callback to run when the websocket connection opens.
   * @param onLogout The callback to run when the app should log out.
   */
  static init(onOpen: () => Promise<void>, onLogout: () => void) {
    WebsocketController.ws = new WebSocket("ws://127.0.0.1:1500/ws");

    WebsocketController.ws.addEventListener("error", (e) => {
      const message = `Failed to reach NRM's websocket at ws://127.0.0.1:1500/ws`;
      const fix = `Please check the backend's logs to see if there was an error, or restart the container`;
      showError(message, fix, BackendErrorType.PANIC);
    });

    WebsocketController.ws.addEventListener("open", () => {
      WebsocketController.ws.send("Hello World!");
      onOpen();
    });

    // * Handles generic messages such as token expiration.
    WebsocketController.ws.addEventListener("message", async (event) => {
      const firstSpace = event.data.indexOf(" ");
      const message = event.data.substring(0, firstSpace);
      const data = event.data.substring(firstSpace + 1);

      switch(message) {
        case "hash_mismatch": {
          WebsocketController.hash = "";
          onLogout();
          get(showWarningSnackbar)({ message: "Something went wrong verifying your request"});
          break;
        }
        case "missing_env_variable": {
          const variable = JSON.parse(data).data;
          const message = `No environment variable ${variable} was found`;
          const fix = `Please check your container to ensure ${variable} is set`;
          showError(message, fix, BackendErrorType.PANIC);
          break;
        }
        case "backend_error": {
          const { message, fix, type } = JSON.parse(data).data as BackendError;
          showError(message, fix, type);
          break;
        }
        case "reload_library": {
          // if (get(showUploadProgressModal) || RestController.currentUploadId) break;

          // const path: string = JSON.parse(data).data;

          // this.addRomFromPath(path);
          break;
        }
      }
    });
  }

  /**
   * Sends a message to the backend.
   * @param message The message name.
   * @param data The data to send. **Always use an object literal**
   * @returns The backend's response.
   */
  private static async invoke<T>(message: string, data: Record<string, any>): Promise<Response<T>> {
    const result = new Promise<Response<T>>((resolve, reject) => {
      const handler = (event: MessageEvent<string>) => {
        if (event.data.startsWith(message)) {
          WebsocketController.ws.removeEventListener("message", handler);

          const jsonStart = event.data.indexOf(" ") + 1;
          const data = JSON.parse(event.data.substring(jsonStart)) as Response<T>;

          resolve(data);
        }
      }

      WebsocketController.ws.addEventListener("message", handler);
    });

    if (message !== "user_auth") {
      data.passwordHash = WebsocketController.hash;
    }

    WebsocketController.ws.send(`${message} ${JSON.stringify(data)}`);

    return await result;
  }


  /**
   * Authenticates the user.
   * @param user The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(user: string, passwordHash: string): Promise<boolean> {
    const res = await WebsocketController.invoke<boolean>("user_auth", { user, passwordHash});
    const success = res.data;

    if (success) {
      WebsocketController.hash = passwordHash;
    }

    return success;
  }


  /**
   * Loads the app's settings from the file system.
   * @returns The app's settings.
   */
  static async loadSettings(): Promise<Settings> {
    const res = await WebsocketController.invoke<Settings>("load_settings", {});
    return res.data;
  }

  /**
   * Writes the app's settings to the file system.
   * @returns True if the write was successful, false otherwise.
   */
  static async writeSettings(): Promise<boolean> {
    const res = await WebsocketController.invoke<boolean>("write_settings", {});
    return res.data;
  }

  /**
   * Updates a setting and writes the app's settings to the file system.
   * @param key The key of the setting to set. (Ex: "palette" or "something.something.something")
   * @param value The setting's new value.
   * @returns True if the update was successful, false otherwise.
   */
  static async setSetting<T>(key: string, value: T): Promise<boolean> {
    const res = await WebsocketController.invoke<boolean>("set_setting", { key, value });
    return res.data;
  }


  /**
   * ! Useful if live reloading is added in the future.
   * Adds a rom based on its path.
   * @param path The path of the Rom to add.
   */
  private static async addRomFromPath(path: string) {
    const lib = get(library)
    const systemMap = get(systems);
    const systemList = Object.values(systemMap);

    const romMap = get(roms);
    const romSystemMap = get(romsBySystem);

    let libraryName = null;

    if (!libraryName) {
      LogController.log(`\"${path}\" did not start with a library path. Skipping...`);
      return;
    }

    let pathNoLibrary = path.substring(lib.libraryPath.length + lib.romDir.length + 1);

    let systemName = null;

    for (const system of systemList) {
      const parserName = systemToParser(system.abbreviation);

      if (pathNoLibrary.startsWith(parserName)) {
        systemName = parserName;
        break;
      }
    }

    if (!systemName) {
      LogController.log(`\"${path}\" did not contain a system. Skipping...`);
      return;
    }

    const rom = await this.parseAddedRom(systemName, path);
    const id = hash64(rom.path);

    romMap[id] = rom;
    romSystemMap[systemName].push(id);
    
    roms.set({ ...romMap });
    romsBySystem.set({ ...romSystemMap });
  }

  
  /**
   * Gets the rom metadata from the server.
   * @returns The rom metadata.
   */
  static async getMetadata(): Promise<Record<string, ROMMetadata>> {
    const res = await WebsocketController.invoke<Record<string, ROMMetadata>>("load_metadata", {});
    return res.data;
  }
  
  /**
   * Saves the rom metadata to the server.
   * @param data The metadata to save.
   * @returns True if the save was a success.
   */
  static async saveMetadata(data: Record<string, ROMMetadata>): Promise<boolean> {
    const res = await WebsocketController.invoke<boolean>("save_metadata", { data });
    return res.data;
  }
  

  /**
   * Loads the app's library.
   * @returns The loaded library data.
   */
  static async loadLibrary(): Promise<LoadResult> {
    const res = await WebsocketController.invoke<LoadResult>("load_library", {});
    return res.data;
  }

  /**
   * Updates the app's library.
   * @param library The updated library.
   * @returns The loaded library data.
   */
  static async updateLibrary(library: Library): Promise<LoadResult> {
    const res = await WebsocketController.invoke<LoadResult>("update_library", { library });
    return res.data;
  }

  /**
   * Parses all of the necessary data from an uploaded rom's path.
   * @param parser The parser for the rom's system.
   * @param romPath The rom's path.
   * @returns The parsed rom data.
   */
  static async parseAddedRom(parser: string, romPath: string): Promise<ROM> {
    const res = await WebsocketController.invoke<ROM>("parse_rom", { parser, romPath });
    return res.data;
  }
  
  /**
   * Gets the entries to render for the file picker.
   * @param path The path to read.
   * @param config The file picker config.
   * @returns The list of entries.
   */
  static async getFilePickerEntries(path: string, config: FilePickerConfig): Promise<FilePickerEntry[]> {
    const res = await WebsocketController.invoke<FilePickerEntry[]>("file_picker", {
      path: path,
      config: {
        showFiles: config.showFiles ?? true,
        extensions: config.extensions ?? [],
        showHiddenFiles: config.showHiddenFiles ?? false,
        max: config.max ?? 1000,
      }
    });
    return res.data;
  }
  
  /**
   * Gets info about used and total space available.
   * @returns The storage info.
   */
  static async getStorageInfo(): Promise<AvailableStorage> {
    const res = await WebsocketController.invoke<AvailableStorage>("available_storage", {});
    return res.data;
  }
}