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

  /**
   * Function to run on cleanup.
   */
  static destroy() {
    
  }
}