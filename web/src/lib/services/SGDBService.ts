import { hasMorePagesCache, showWarningSnackbar, steamGridSearchCache } from "@stores/State";
import type { GridResults, SGDBGame, SGDBImage } from "@types";
import { get } from "svelte/store";
import { LogService } from "./utils/LogService";
import { RestService } from "./utils/RestService";

/**
 * The SGDB API Service.
 */
export class SGDBService {
  private static readonly SGDB_GRID_RESULT_LIMIT = 25;

  private static gridsCache: { [steamGridId: string]: Record<string, SGDBImage[]> } = {};
  private static currentGridCountCache: { [steamGridId: string]: Record<string, number> } = {};
  private static totalGridCountCache: { [steamGridId: string]: Record<string, number> } = {};

  /**
   * Initializes the api controller.
   */
  static async init(): Promise<void> {
    await RestService.initSGDBClient();
  }

  private static async getGridsForGame(steamGridAppId: string, gridType: "grids" | "heroes"): Promise<SGDBImage[]> {
    if (!this.gridsCache[steamGridAppId]) {
      this.gridsCache[steamGridAppId] = {
        grids: [],
        heroes: [],
      };
    }
    if (!this.currentGridCountCache[steamGridAppId]) {
      this.currentGridCountCache[steamGridAppId] = {
        grids: 0,
        heroes: 0,
      };
    }
    if (!this.totalGridCountCache[steamGridAppId]) {
      this.totalGridCountCache[steamGridAppId] = {
        grids: 0,
        heroes: 0,
      };
    }
    
    const morePagesCache = get(hasMorePagesCache);
    if (!Object.keys(morePagesCache).includes(steamGridAppId.toString())) {
      morePagesCache[steamGridAppId] = {
        grids: true,
        heroes: true,
      };
    }

    if (!morePagesCache[steamGridAppId][gridType] && this.gridsCache[steamGridAppId][gridType]) {
      return this.gridsCache[steamGridAppId][gridType];
    }

    let page = 0;

    // * checking undefined here because 0 is falsy.
    if (this.currentGridCountCache[steamGridAppId] !== undefined && this.totalGridCountCache[steamGridAppId] !== undefined) {
      page = Math.max(Math.floor(this.currentGridCountCache[steamGridAppId][gridType] / this.SGDB_GRID_RESULT_LIMIT), 0);
    }

    try {
      LogService.log(`Need to fetch ${gridType} page ${page} for ${steamGridAppId}.`);

      const gridResults: GridResults = await RestService.getSGDBGridsById(steamGridAppId, page, gridType);
      
      this.gridsCache[steamGridAppId][gridType] = this.gridsCache[steamGridAppId][gridType].concat(gridResults.images);
      this.currentGridCountCache[steamGridAppId][gridType] = this.gridsCache[steamGridAppId][gridType].length;
      this.totalGridCountCache[steamGridAppId][gridType] = gridResults.total;
      morePagesCache[steamGridAppId][gridType] = this.currentGridCountCache[steamGridAppId][gridType] !== this.totalGridCountCache[steamGridAppId][gridType];

      hasMorePagesCache.set(morePagesCache);

      return this.gridsCache[steamGridAppId][gridType];
    } catch (e: any) {
      LogService.error(`Error fetching ${gridType} for game: ${steamGridAppId}. Error: ${e.message}.`);
      get(showWarningSnackbar)({ message: `Error fetching ${gridType} for game.` });
      return [];
    }
  }

  /**
   * Fetches the SGDB capsules for the provided title.
   * @param steamGridAppId The sgdb appId of the app to get.
   * @returns A promise resolving to a list of grids.
   * ? Logging complete.
   */
  static async getCapsulesForGame(steamGridAppId: string): Promise<SGDBImage[]> {
    return await this.getGridsForGame(steamGridAppId, "grids");
  }

  /**
   * Fetches the SGDB heroes for the provided title.
   * @param steamGridAppId The sgdb appId of the app to get.
   * @returns A promise resolving to a list of grids.
   * ? Logging complete.
   */
  static async getHeroesForGame(steamGridAppId: string): Promise<SGDBImage[]> {
    return await this.getGridsForGame(steamGridAppId, "heroes");
  }

  /**
   * Gets the sgdb game id for the provided rom or system.
   * @param id The id of the rom / system to gete the SGDB id for.
   * @param gameName The name of the game to fetch grids for.
   * @returns A promise resolving to the grids.
   * ? Logging complete.
   */
  static async chooseSteamGridGameId(id: string, gameName: string): Promise<string> {
    LogService.log(`Finding SGDB game for ${id}...`);

    const searchCache = get(steamGridSearchCache);

    let results = searchCache[id];

    if (!results) {
      try {
        results = await RestService.searchSGDBForTitle(gameName);
        searchCache[id] = results;
      } catch (e: any) {
        LogService.warn(`Error searching for game on SGDB. Game: ${gameName}. Error: ${e.message}.`);
        get(showWarningSnackbar)({ message: "Error searching for game on SGDB." });
        return "None";
      }
    }

    let chosenResult: SGDBGame | undefined;
    
    chosenResult = results.find((game) => game.name === gameName);
    if (!chosenResult && results.length > 0) chosenResult = results[0];

    if (chosenResult?.id) {
      steamGridSearchCache.set(searchCache);
      return chosenResult.id.toString();
    } else {
      LogService.log(`No results for ${gameName}.`);
      return "None";
    }
  }

  /**
   * Searches SGDB for the provided query.
   * @param query The search query to use.
   * @returns A promise resolving to the results array, or null if the request timed out.
   */
  static async searchForGame(query: string): Promise<SGDBGame[]> {
    try {
      return await RestService.searchSGDBForTitle(query);
    } catch (e: any) {
      LogService.warn(`SGDB Search Request "${query}" timed out.`);
      return [];
    }
  }
}