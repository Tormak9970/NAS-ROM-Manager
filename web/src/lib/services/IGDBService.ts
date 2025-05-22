import type { IGDBGame, IGDBMetadataPlatform, IGDBSearchResult } from "@types";
import { LogService } from "./utils/LogService";
import { RestService } from "./utils/RestService";

/**
 * The IGDB API Service.
 */
export class IGDBService {
  /**
   * Initializes the service.
   */
  static async init(): Promise<void> {
    await RestService.initIGDBClient();
  }

  /**
   * Gets the IGDB metadata for a given game.
   * @param igdbId The IGDB id of the game.
   * @returns A promise resolving to metadata, or null if the request timed out.
   */
  static async getMetadata(igdbId: string): Promise<IGDBGame | null> {
    try {
      return await RestService.getIGDBMetadataById(igdbId);
    } catch (e: any) {
      LogService.warn(`IGDB Metadata Request for "${igdbId}" timed out.`);
      LogService.warn(e.message);
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
      return await RestService.searchIGDBForTitle(query, igdbPlatformId);
    } catch (e: any) {
      LogService.warn(`IGDB Rom Search Request "${query}" timed out.`);
      LogService.warn(e.message);
      return [];
    }
  }
  
  /**
   * Searches IGDB for the provided query.
   * @param query The search query to use.
   * @returns A promise resolving to the results array, or null if the request timed out.
   */
  static async searchForPlatform(query: string): Promise<IGDBMetadataPlatform[]> {
    try {
      return await RestService.searchIGDBForPlatform(query);
    } catch (e: any) {
      LogService.warn(`IGDB Platform Search Request "${query}" timed out.`);
      LogService.warn(e.message);
      return [];
    }
  }
}