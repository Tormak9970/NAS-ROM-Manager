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

import { landingPage, landscapeViews, library, loadedSettings, palette, portraitViews, reducedMotion, romMetadata, saveMetadataAlongside, systems, themePrimaryColor, useOledPalette } from "@stores/State";
import type { Settings } from "@types";
import type { Unsubscriber } from "svelte/store";
import { LogService } from "./LogService";
import { WebsocketService } from "./WebsocketService";

/**
 * The Settings Service.
 */
export class SettingsService {
  private static settings: Settings;
  private static subscriptions: Unsubscriber[] = [];

  /**
   * Initializes the SettingsService.
   */
  static async init() {
    this.settings = await WebsocketService.loadSettings();
    LogService.log("Finished loading settings.");

    await this.setStores();
    
    loadedSettings.set(true);
  }

  private static async writeAll(): Promise<boolean> {
    return await WebsocketService.writeSettings();
  }

  /**
   * Gets a setting.
   * @param key The key of the setting to get.
   */
  static get<T>(key: string): T {
    return this.settings[key as keyof Settings] as T;
  }

  /**
   * Sets a setting's value.
   * @param key The key of the setting to set.
   * @param value The setting's value
   * @returns True if the update was successful, false otherwise.
   */
  static async set<T>(key: string, value: T): Promise<boolean> {
    this.setOnChange(key)(value);
    return await WebsocketService.setSetting<T>(key, value);
  }

  private static setOnChange<T>(key: string): (value: T) => void {
    const keys = key.split(".");
    const lastKey = keys[keys.length - 1];

    let parentObject = this.settings as any;
    for (let i = 0; i < keys.length - 1; i++) {
      parentObject = parentObject[keys[i]];
    }

    return (value: T) => {
      parentObject[lastKey] = value;
      WebsocketService.setSetting<T>(key, value);
    }
  }

  private static async setStores(): Promise<void> {
    const theme = this.settings.theme;
    themePrimaryColor.set(theme.primaryColor);
    palette.set(theme.palette);
    useOledPalette.set(theme.useOledPalette);

    const navigation = this.settings.navigation;
    landingPage.set(navigation.landingPage);
    landscapeViews.set(navigation.landscapeViews);
    portraitViews.set(navigation.portraitViews);
    
    const metadata = this.settings.metadata;
    saveMetadataAlongside.set(metadata.saveAlongsideROMs);
    
    const accessibility = this.settings.accessibility;
    reducedMotion.set(accessibility.reducedMotion);

    library.set(this.settings.library);
  }

  /**
   * Registers the store listeners responsible for automatically updating the settings.
   */
  static registerSubs() {
    this.subscriptions = [
      themePrimaryColor.subscribe(this.setOnChange("theme.primaryColor")),
      palette.subscribe(this.setOnChange("theme.palette")),
      useOledPalette.subscribe(this.setOnChange("theme.useOledPalette")),

      landingPage.subscribe(this.setOnChange("navigation.landingPage")),
      landscapeViews.subscribe(this.setOnChange("navigation.landscapeViews")),
      portraitViews.subscribe(this.setOnChange("navigation.portraitViews")),

      saveMetadataAlongside.subscribe(this.setOnChange("metadata.saveAlongsideROMs")),

      reducedMotion.subscribe(this.setOnChange("accessibility.reducedMotion")),

      romMetadata.subscribe(WebsocketService.saveMetadata),

      systems.subscribe(WebsocketService.saveParsers),
    ];
  }

  /**
   * Handles destroying the settings.
   */
  static destroy() {
    for (const unsub of this.subscriptions) {
      unsub();
    }
    
    loadedSettings.set(false);
  }
}