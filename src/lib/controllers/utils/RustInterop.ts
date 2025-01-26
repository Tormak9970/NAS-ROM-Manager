/**
 * Copyright (C) 2023 Travis Lane (Tormak)
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

import type { Library, LoadedLibrary, Settings } from "@types";
import { get } from "svelte/store";
import { showErrorSnackbar } from "../../../stores/State";
import { goto } from "$app/navigation";

/**
 * The available logging levels.
 */
export enum LogLevel {
  INFO,
  WARN,
  ERROR
}

type Response<T> = { data: T, }

/**
 * Handles wrapping ipc communication into an easy to use JS bindings.
 */
export class RustInterop {
  private static ws: WebSocket;
  private static hash: string;

  /**
   * Initializes the Rust <-> Svelte communication.
   * @param onOpen The callback to run when the websocket connection opens.
   * @param onLogout The callback to run when the app should log out.
   */
  static init(onOpen: () => Promise<void>, onLogout: () => void) {
    RustInterop.ws = new WebSocket("ws://127.0.0.1:1500/api");

    RustInterop.ws.addEventListener("open", () => {
      RustInterop.ws.send("Hello World!");
      onOpen();
    });

    // * Handles generic messages such as token expiration.
    RustInterop.ws.addEventListener("message", (event) => {
      const parts = event.data.split(" ");

      switch(parts[0]) {
        case "hash_mismatch":
          RustInterop.hash = "";
          onLogout();
          get(showErrorSnackbar)({ message: "Something went wrong verifying your request"});
          break;
        case "missing_env_variable":
          const variable = JSON.parse(parts[1]).data;
          const message = `No environment variable ${variable} was found`;
          const fix = `Please check your container to ensure ${variable} is set`;
          goto(`/error?message=${message}&fix=${fix}`);
          break;
        case "reload_library":
          // TODO: implement reloading the libraries. Path that caused the reload will be .data
          break;
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
          RustInterop.ws.removeEventListener("message", handler);

          const jsonStart = event.data.indexOf(" ") + 1;
          const data = JSON.parse(event.data.substring(jsonStart)) as Response<T>;

          resolve(data);
        }
      }

      RustInterop.ws.addEventListener("message", handler);
    });

    if (message !== "user_auth") {
      data.passwordHash = RustInterop.hash;
    }

    RustInterop.ws.send(`${message} ${JSON.stringify(data)}`);

    return await result;
  }


  /**
   * Authenticates the user.
   * @param user The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(user: string, passwordHash: string): Promise<boolean> {
    const res = await RustInterop.invoke<boolean>("user_auth", { user, passwordHash});
    const success = res.data;

    if (success) {
      RustInterop.hash = passwordHash;
    }

    return success;
  }


  /**
   * Loads the app's settings from the file system.
   * @returns The app's settings.
   */
  static async loadSettings(): Promise<Settings> {
    const res = await RustInterop.invoke<Settings>("load_settings", {});
    return res.data;
  }

  /**
   * Writes the app's settings to the file system.
   * @returns True if the write was successful, false otherwise.
   */
  static async writeSettings(): Promise<boolean> {
    const res = await RustInterop.invoke<boolean>("write_settings", {});
    return res.data;
  }

  /**
   * Updates a setting and writes the app's settings to the file system.
   * @param key The key of the setting to set. (Ex: "palette" or "something.something.something")
   * @param value The setting's new value.
   * @returns True if the update was successful, false otherwise.
   */
  static async setSetting<T>(key: string, value: T): Promise<boolean> {
    const res = await RustInterop.invoke<boolean>("set_setting", { key, value });
    return res.data;
  }


  /**
   * Loads the app's libraries.
   * @returns The loaded libraries data.
   */
  static async loadLibraries(): Promise<LoadedLibrary[]> {
    const res = await RustInterop.invoke<LoadedLibrary[]>("load_libraries", {});
    return res.data;
  }

  /**
   * Adds a library to the manager.
   * @param library The library to add.
   * @returns The loaded library data.
   */
  static async addLibrary(library: Library): Promise<LoadedLibrary> {
    const res = await RustInterop.invoke<LoadedLibrary>("add_library", { library });
    return res.data;
  }

  /**
   * Removes a library from the manager.
   * @param library The library to remove.
   * @returns True if the library was removed, false otherwise.
   */
  static async removeLibrary(library: Library): Promise<boolean> {
    const res = await RustInterop.invoke<boolean>("remove_library", { library });
    return res.data;
  }

  /**
   * Gets the user's SGDB api key.
   * @returns The user's SGDB api key.
   */
  static async getSGDBKey(): Promise<string> {
    const res = await RustInterop.invoke<string>("get_sgdb_key", {});
    return res.data;
  }
}