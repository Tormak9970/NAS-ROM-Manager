import { goto } from "$app/navigation";
import { isSignedIn, username } from "@stores/Auth";
import { landingPage } from "@stores/State";
import { get } from "svelte/store";
import { AppService } from "./AppService";
import { WebsocketService } from "./utils/WebsocketService";

/**
 * The user authentication Service.
 */
export class AuthService {
  /**
   * Authenticates the user.
   * @param user The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(user: string, passwordHash: string): Promise<boolean> {
    const success = await WebsocketService.authenticate(user, passwordHash);

    if (success) {
      sessionStorage.setItem("hash", passwordHash);
      sessionStorage.setItem("user", user);
      username.set(user);
      isSignedIn.set(true);
      await AppService.load();

      if (window.location.pathname === '/') {
        goto(`/${get(landingPage)}`);
      }
    }

    return success;
  }
  
  /**
   * Logs the current user out and resets the relevant state.
   */
  static logout() {
    sessionStorage.removeItem("hash");
    sessionStorage.removeItem("user");
    username.set("");
    isSignedIn.set(false);
    AppService.unload();
  }
}