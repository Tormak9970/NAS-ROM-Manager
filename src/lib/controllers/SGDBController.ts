import { RequestError, SGDB } from "@models";
import { hasMorePagesCache, showWarningSnackbar, steamGridSearchCache } from "@stores/State";
import type { SGDBGame, SGDBImage } from "@types";
import { get } from "svelte/store";
import { LogController } from "./utils/LogController";
import { WebsocketController } from "./utils/WebsocketController";

/**
 * The sgdb api controller.
 */
export class SGDBController {
  private static readonly SGDB_GRID_RESULT_LIMIT = 50;

  private static client: SGDB;

  private static gridsCache: { [steamGridId: string]: SGDBImage[] } = {};
  private static currentGridCountCache: { [steamGridId: string]: number } = {};
  private static totalGridCountCache: { [steamGridId: string]: number } = {};

  /**
   * Initializes the api controller.
   */
  static async init(): Promise<void> {
    const key = await WebsocketController.getSGDBKey();
    console.log("key:", key)

    this.client = new SGDB(key);
  }

  /**
   * Gets the grids for a non steam game.
   * @param steamGridAppId The sgdb appId of the app to get.
   * @returns A promise resolving to a list of grids.
   * ? Logging complete.
   */
  static async fetchGrids(steamGridAppId: string): Promise<SGDBImage[]> {
    if (!this.gridsCache[steamGridAppId]) this.gridsCache[steamGridAppId] = [];
    if (!this.currentGridCountCache[steamGridAppId]) this.currentGridCountCache[steamGridAppId] = 0;
    if (!this.totalGridCountCache[steamGridAppId]) this.totalGridCountCache[steamGridAppId] = 0;
    
    const morePagesCache = get(hasMorePagesCache);
    if (!morePagesCache[steamGridAppId.toString()]) morePagesCache[steamGridAppId] = true;
    const morePagesEntry = morePagesCache[steamGridAppId];

    if (!Object.keys(morePagesEntry)) morePagesCache[steamGridAppId] = true;

    if (!morePagesCache[steamGridAppId] && this.gridsCache[steamGridAppId]) return this.gridsCache[steamGridAppId];

    let page = 0;

    // * checking undefined here because 0 is falsy.
    if (this.currentGridCountCache[steamGridAppId] !== undefined && this.totalGridCountCache[steamGridAppId] !== undefined) {
      page = Math.max(Math.floor(this.currentGridCountCache[steamGridAppId] / this.SGDB_GRID_RESULT_LIMIT), 0);
    }

    try {
      LogController.log(`Need to fetch page ${page} for ${steamGridAppId}.`);

      // @ts-expect-error This will always be a function on this.client
      const gridResults: GridResults = await this.client.getGridsById(steamGridAppId, undefined, undefined, undefined, [ "static", "animated" ], "any", "any", "any", page);
      
      this.gridsCache[steamGridAppId] = this.gridsCache[steamGridAppId].concat(gridResults.images);
      this.currentGridCountCache[steamGridAppId] = this.gridsCache[steamGridAppId].length;
      this.totalGridCountCache[steamGridAppId] = gridResults.total;
      morePagesCache[steamGridAppId] = this.currentGridCountCache[steamGridAppId] !== this.totalGridCountCache[steamGridAppId];

      hasMorePagesCache.set(morePagesCache);

      return this.gridsCache[steamGridAppId];
    } catch (e: any) {
      LogController.error(`Error fetching grids for game: ${steamGridAppId}. Error: ${e.message}.`);
      get(showWarningSnackbar)({ message: "Error fetching grids for game." });
      return [];
    }
  }

  /**
   * Gets the sgdb game id for the provided game.
   * @param romId The id of the rom to gete the SGDB id for.
   * @param gameName The name of the game to fetch grids for.
   * @returns A promise resolving to the grids.
   * ? Logging complete.
   */
  static async chooseSteamGridGameId(romId: string, gameName: string): Promise<string> {
    LogController.log(`Finding SGDB game for ${romId}...`);

    const searchCache = get(steamGridSearchCache);

    let results = searchCache[romId];

    if (!results) {
      try {
        results = await this.client!.searchGame(gameName);
        searchCache[romId] = results;
      } catch (e: any) {
        LogController.warn(`Error searching for game on SGDB. Game: ${gameName}. Error: ${e.message}.`);
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
      LogController.log(`No results for ${gameName}.`);
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
      return await this.client.searchGame(query);
    } catch (e: any) {
      const error = e as RequestError;
      LogController.warn(`SGDB Search Request "${query}" timed out. Message: ${error.message}`);
      return [];
    }
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