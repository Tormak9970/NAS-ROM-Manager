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

/**
 * The Logger Service.
 * ! Should do no logging here.
 */
export class LogService {
  private static APP_NAME = "NAS ROM Manager";
  private static APP_THEME_COLOR = "#73169e";
  private static APP_INFO_COLOR = "#1abc9c";
  private static APP_WARN_COLOR = "#e3c907";
  private static APP_ERROR_COLOR = "#c70808";

  /**
   * Logs a message with level [INFO] to the core log file.
   */
  static log = console.log.bind(
    globalThis.console,
    `%c ${LogService.APP_NAME} %c INFO %c`,
    `background: ${LogService.APP_THEME_COLOR}; color: white;`,
    `background: ${LogService.APP_INFO_COLOR}; color: black;`,
    "background: transparent;"
  );

  /**
   * Logs a message with level [WARNING] to the core log file.
   */
  static warn = console.warn.bind(
    globalThis.console,
    `%c ${LogService.APP_NAME} %c WARNING %c`,
    `background: ${LogService.APP_THEME_COLOR}; color: white;`,
    `background: ${LogService.APP_WARN_COLOR}; color: black;`,
    "background: transparent;",
  );

  /**
   * Logs a message with level [ERROR] to the core log file.
   */
  static error = console.error.bind(
    globalThis.console,
    `%c ${LogService.APP_NAME} %c ERROR %c`,
    `background: ${LogService.APP_THEME_COLOR}; color: white;`,
    `background: ${LogService.APP_ERROR_COLOR}; color: black;`,
    "background: transparent;"
  );
}