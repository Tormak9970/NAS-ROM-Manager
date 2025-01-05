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

/**
 * The available logging levels.
 */
export enum LogLevel {
  INFO,
  WARN,
  ERROR
}

/**
 * Handles wrapping ipc communication into an easy to use JS bindings.
 * ! Should do no logging here.
 */
export class RustInterop {
  private static ws: WebSocket;

  static init() {
    RustInterop.ws = new WebSocket("ws://127.0.0.1:1500/ws");

    RustInterop.ws.addEventListener("open", () => {
      RustInterop.ws.send("Hello World!");
    });

    // TODO: initialize global listeners, such as token_expired
    RustInterop.ws.addEventListener("message", (event) => {

    });
  }

  private static async invoke(message: string, ...args: string[]): Promise<string[]> {
    const result = new Promise<string[]>((resolve, reject) => {
      const handler = (event: MessageEvent<string>) => {
        if (event.data.startsWith(message)) {
          const parts = event.data.split(" ");
          RustInterop.ws.removeEventListener("message", handler);

          resolve(parts);
        }
      }

      RustInterop.ws.addEventListener("message", handler);
    });

    RustInterop.ws.send(`${message} ${args.join(" ")}`);

    return await result;
  }

  /**
   * Cleans the app's log file.
   */
  static async cleanOutLog(): Promise<void> {
    // await invoke("clean_out_log", {});
  }

  /**
   * Logs a message to the log file.
   * @param message The message to log.
   * @param level The log level.
   */
  static async logToFile(message: string, level: LogLevel): Promise<void> {
    // await invoke("log_to_file", { message: message, level: level });
  }

  /**
   * Authenticates the user.
   * @param username The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(username: string, passwordHash: string): Promise<boolean> {
    const parts = await RustInterop.invoke("user_auth", username, passwordHash);
    const success = parts[2] === "true";

    if (success) sessionStorage.setItem("jwt", parts[3]);

    return success;
  }
}