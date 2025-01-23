import { isSignedIn, username } from "@stores/Auth";
import { AppController } from "./AppController";
import { RustInterop } from "./utils/RustInterop";

/**
 * The user authentication controller.
 */
export class AuthController {
  /**
   * Authenticates the user.
   * @param user The username to authenticate with.
   * @param passwordHash The hash of the user's password.
   * @returns The backend's response.
   */
  static async authenticate(user: string, passwordHash: string): Promise<boolean> {
    const success = await RustInterop.authenticate(user, passwordHash);

    if (success) {
      sessionStorage.setItem("hash", passwordHash);
      sessionStorage.setItem("user", user);
      username.set(user);
      isSignedIn.set(true);
      AppController.load();
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
    AppController.unload();
  }
}