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

import type { Library, Settings } from "@types";
import { LogController } from "./LogController";
import { RustInterop } from "./RustInterop";
import type { Unsubscriber } from "svelte/store";
import { collections, hasLoadedSettings, landingPage, libraries, palette, themePrimaryColor, useOledPalette } from "@stores/State";

/**
 * The controller for settings.
 */
export class SettingsController {
  private static settings: Settings;
  private static subscriptions: Unsubscriber[] = [];

  /**
   * Initializes the SettingsController.
   */
  static async init() {
    this.settings = await RustInterop.loadSettings();
    LogController.log("Finished loading settings.");

    await this.setStores();
    
    hasLoadedSettings.set(true);
  }

  private static async writeAll(): Promise<boolean> {
    return await RustInterop.writeSettings();
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
    return await RustInterop.setSetting<T>(key, value);
  }

  private static async setStores(): Promise<void> {
    const themeSettings = this.settings.theme;
    themePrimaryColor.set(themeSettings.primaryColor);
    palette.set(themeSettings.palette);
    useOledPalette.set(themeSettings.useOledPalette);

    landingPage.set(this.settings.landingPage);

    collections.set(this.settings.collections);
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
      this.set<T>(key, value);
    }
  }

  /**
   * Registers the store listeners responsible for automatically updating the settings.
   */
  static registerSubs() {
    this.subscriptions = [
      themePrimaryColor.subscribe(this.setOnChange("theme.primaryColor")),
      palette.subscribe(this.setOnChange("theme.palette")),
      useOledPalette.subscribe(this.setOnChange("theme.useOledPalette")),
      landingPage.subscribe(this.setOnChange("landingPage")),

      libraries.subscribe((libraries) => {
        const libraryList = Object.values(libraries);
        this.settings.libraries = libraryList;
        this.set<Library[]>("libraries", libraryList);
      }),
      collections.subscribe(this.setOnChange("collections")),
    ];
  }

  /**
   * Handles destroying the settings.
   */
  static destroy() {
    for (const unsub of this.subscriptions) {
      unsub();
    }
    
    hasLoadedSettings.set(false);
  }
}