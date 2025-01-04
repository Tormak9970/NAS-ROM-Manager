/**
 * Copyright (C) 2023 Travis Lane (Tormak)
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>
 */

import { type Settings, DEFAULT_SETTINGS } from "@types";
import { debounce } from "@utils";
import { LogController } from "./LogController";

/**
 * The controller for settings.
 */
export class SettingsController {
  private static readonly STORE_NAME = "settings.dat";
  private static settings: Settings;

  private static async loadSettings(): Promise<Settings> {
    const defaultEntries = Object.entries(DEFAULT_SETTINGS);

    // TODO: load settings

    let settings = {} as Settings;

    settings.version = APP_VERSION;

    LogController.log("Finished checking settings for new app version and/or migration.");

    return settings;
  }

  private static async saveSettings() {
    
  }
  private static debouncedSave = debounce(this.saveSettings.bind(this), 1000) as () => Promise<void>;

  private static async save() {
    await this.debouncedSave();
  }

  /**
   * Initializes the SettingsController.
   */
  static async init() {
    // TODO: load

    this.save();

    await this.setStores();

    LogController.log("Initialized Settings.");
  }

  /**
   * Sets the Svelte stores associated with the settings.
   */
  private static async setStores(): Promise<void> {
    
  }

  /**
   * Registers the subscriptions to stores.
   */
  static registerSubs() {
    
  }

  /**
   * Handles destroying the settings.
   */
  static destroy() {
    
  }
}