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
          break;
        case "missing_env_variable":
          // TODO: go to the an error page indicating which environment variable is missing and explaining how to fix it.
          break;
      }
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

    let interopInfo = message;

    if (message !== "user_auth") {
      interopInfo += ` ${RustInterop.hash}`;
    }

    RustInterop.ws.send(`${interopInfo} ${args.join(" ")}`);

    return await result;
  }

  /**
   * Authenticates the user.
   * @param user The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(user: string, passwordHash: string): Promise<boolean> {
    const parts = await RustInterop.invoke("user_auth", user, passwordHash);
    const success = parts[2] === "true";

    if (success) {
      sessionStorage.setItem("hash", passwordHash);
      sessionStorage.setItem("user", user);
      RustInterop.hash = passwordHash;
      username.set(user);
      isSignedIn.set(true);
    }

    return success;
  }
}