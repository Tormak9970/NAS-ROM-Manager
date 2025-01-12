import { SettingsController } from "./utils/SettingsController";

/**
 * The core app controller.
 */
export class AppController {
  /**
   * Checks if the app has an internet connection.
   */
  static isOnline() {
    return navigator.onLine;
  }

  /**
   * Loads the app's state.
   */
  static async load() {
    await SettingsController.init();
  }

  static async loadROMsFromLibrary() {
    // TODO: load from system
    // TODO: apply customizations from settings
    // TODO: set state
  }

  /**
   * Unloads the app's state and performs any cleanup.
   */
  static unload() {
    SettingsController.destroy();
  }
}