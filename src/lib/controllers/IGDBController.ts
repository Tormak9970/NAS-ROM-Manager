import type { IGDBGame, IGDBSearchResult } from "@types";
import { LogController } from "./utils/LogController";
import { RestController } from "./utils/RestController";

/**
 * The igdb api controller.
 */
export class IGDBController {
  /**
   * Initializes the api controller.
   */
  static async init(): Promise<void> {
    await RestController.initIGDBClient();
  }

  /**
   * Gets the IGDB metadata for a given game.
   * @param igdbId The IGDB id of the game.
   * @returns A promise resolving to metadata, or null if the request timed out.
   */
  static async getMetadata(igdbId: string): Promise<IGDBGame | null> {
    try {
      return await RestController.getIGDBMetadataById(igdbId);
    } catch (e: any) {
      LogController.warn(`IGDB Metadata Request for "${igdbId}" timed out.`);
      LogController.warn(e.message);
      return null;
    }
  }
  
  /**
   * Searches IGDB for the provided query.
   * @param query The search query to use.
   * @param igdbPlatformId The IGDB id for the query's platform.
   * @returns A promise resolving to the results array, or null if the request timed out.
   */
  static async searchForGame(query: string, igdbPlatformId: string): Promise<IGDBSearchResult[]> {
    try {
      return await RestController.searchIGDBForTitle(query, igdbPlatformId);
    } catch (e: any) {
      LogController.warn(`IGDB Search Request "${query}" timed out.`);
      LogController.warn(e.message);
      return [];
    }
  }
}