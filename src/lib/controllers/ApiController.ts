import { SGDB } from "@models";
import type { SGDBImage } from "@types";
import { RustInterop } from "./utils/RustInterop";

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

  /**
   * Fetches the SGDB covers for the provided title.
   * @param title The title to fetch the covers for.
   * @returns The fetched covers.
   */
  static async getCoversForGame(title: string): Promise<SGDBImage[]> {
    return []
  }
}