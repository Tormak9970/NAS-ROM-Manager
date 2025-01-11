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
   * Initializes the app.
   */
  static async init() {
    
  }

  static async loadROMsFromLibrary() {
    // TODO: load from system
    // TODO: apply customizations from settings
    // TODO: set state
  }

  /**
   * Function to run on cleanup.
   */
  static destroy() {
    
  }
}