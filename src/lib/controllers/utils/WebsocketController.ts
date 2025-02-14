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

import { goto } from "$app/navigation";
import { LogController } from "@controllers/utils/LogController";
import { libraries, roms, romsByLibrary, romsBySystem, showErrorSnackbar, systems } from "@stores/State";
import { BackendErrorType, type BackendError, type FilePickerConfig, type FilePickerEntry, type Library, type LoadedLibrary, type ROM, type Settings } from "@types";
import { hash64, systemToParser } from "@utils";
import { get } from "svelte/store";

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
          get(showErrorSnackbar)({ message: "Something went wrong verifying your request"});
          break;
        }
        case "missing_env_variable": {
          const variable = JSON.parse(data).data;
          const message = `No environment variable ${variable} was found`;
          const fix = `Please check your container to ensure ${variable} is set`;
          goto(`/error?message=${message}&fix=${fix}&type=${BackendErrorType.PANIC}`);
          break;
        }
        case "backend_error": {
          const { message, fix, type } = JSON.parse(data).data as BackendError;
          goto(`/error?message=${message}&fix=${fix}&type=${BackendErrorType.PANIC}`);
          break;
        }
        case "reload_library": {
          const paths: string[] = JSON.parse(data).data;

          const libraryMap = get(libraries);
          const libraryList = Object.values(libraryMap);
          const systemMap = get(systems);
          const systemList = Object.values(systemMap);

          const romMap = get(roms);
          const romLibraryMap = get(romsByLibrary);
          const romSystemMap = get(romsBySystem);

          for (const path of paths) {
            let libraryName = null;

            for (const library of libraryList) {
              if (path.startsWith(library.path)) {
                libraryName = library.name;
                break;
              }
            }

            if (!libraryName) {
              LogController.log(`\"${path}\" did not start with a library path. Skipping...`);
              continue;
            }

            let pathNoLibrary = path.substring(libraryName.length + 1);

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
              continue;
            }

            const rom = await this.parseAddedRom(libraryName, systemName, path);
            const id = hash64(rom.path);

            romMap[id] = rom;
            romLibraryMap[libraryName].push(id);
            romSystemMap[systemName].push(id);
          }
          
          roms.set({ ...romMap });
          romsByLibrary.set({ ...romLibraryMap });
          romsBySystem.set({ ...romSystemMap });
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
   * Loads the app's libraries.
   * @returns The loaded libraries data.
   */
  static async loadLibraries(): Promise<LoadedLibrary[]> {
    const res = await WebsocketController.invoke<LoadedLibrary[]>("load_libraries", {});
    return res.data;
  }

  /**
   * Adds a library to the manager.
   * @param library The library to add.
   * @returns The loaded library data.
   */
  static async addLibrary(library: Library): Promise<LoadedLibrary> {
    const res = await WebsocketController.invoke<LoadedLibrary>("add_library", { library });
    return res.data;
  }

  /**
   * Removes a library from the manager.
   * @param library The library to remove.
   * @returns True if the library was removed, false otherwise.
   */
  static async removeLibrary(library: Library): Promise<boolean> {
    const res = await WebsocketController.invoke<boolean>("remove_library", { library });
    return res.data;
  }

  /**
   * Parses all of the necessary data from an uploaded rom's path.
   * @param libraryName The name of the library the rom belongs to.
   * @param parser The parser for the rom's system.
   * @param romPath The rom's path.
   * @returns The parsed rom data.
   */
  static async parseAddedRom(libraryName: string, parser: string, romPath: string): Promise<ROM> {
    const res = await WebsocketController.invoke<ROM>("parse_rom", { libraryName, parser, romPath });
    return res.data;
  }

  /**
   * Gets the user's SGDB api key.
   * @returns The user's SGDB api key.
   */
  static async getSGDBKey(): Promise<string> {
    const res = await WebsocketController.invoke<string>("get_sgdb_key", {});
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
        extensions: config.extensions,
        showHiddenFiles: config.showHiddenFiles ?? false,
        max: config.max ?? 1000,
      }
    });
    return res.data;
  }
}