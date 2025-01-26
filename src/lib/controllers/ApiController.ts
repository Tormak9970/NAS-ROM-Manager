import { RustInterop } from "./utils/RustInterop";
import { SGDB } from "@models";

/**
 * The api controller.
 */
export class ApiController {
  private static client: SGDB;

  /**
   * Initializes the api controller.
   */
  static async init(): Promise<void> {
    const key = await RustInterop.getSGDBKey();

    this.client = new SGDB(key);
  }
  
  
}