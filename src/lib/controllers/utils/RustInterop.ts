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

import { isSignedIn, username } from "@stores/Auth";
import { LogController } from "./LogController";
import type { Library, LoadedLibrary, Settings } from "@types";
import { SettingsController } from "./SettingsController";
import { AppController } from "../AppController";

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

  static init(onOpen: () => Promise<void>) {
    RustInterop.ws = new WebSocket("ws://127.0.0.1:1500/ws");

    RustInterop.ws.addEventListener("open", () => {
      RustInterop.ws.send("Hello World!");
      onOpen();
    });

    // * Handles generic messages such as token expiration.
    RustInterop.ws.addEventListener("message", (event) => {
      const parts = event.data.split(" ");

      switch(parts[0]) {
        case "hash_mismatch":
          sessionStorage.removeItem("hash");
          sessionStorage.removeItem("user");
          RustInterop.hash = "";
          username.set("");
          isSignedIn.set(false);
          AppController.unload();
          break;
        case "missing_env_variable":
          // TODO: go to the an error page indicating which environment variable is missing and explaining how to fix it.
          break;
        case "file_added":
          // TODO: will provide what library its from as .library
          break;
        case "file_removed":
          // TODO: will provide what library its from as .library
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
      sessionStorage.setItem("hash", passwordHash);
      sessionStorage.setItem("user", user);
      RustInterop.hash = passwordHash;
      username.set(user);
      isSignedIn.set(true);
      AppController.load();
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
}