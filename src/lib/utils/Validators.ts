/**
 * Checks if a string is a valid absolute path on Windows, with only forward slashes.
 * @param path The path to validate.
 * @returns True if the path is a valid absolute windows path.
 * 
 * @example
 * ```ts
 * // Valid Absolute Windows paths:
 * const path1 = "C:/User/Tormak/Desktop/config";
 * 
 * // Invalid Absolute Windows paths:
 * const path2 = "C:\\User\\Tormak\\Desktop\\config";
 * const path3 = "C:\\Users//Tormak/Desktop/config";
 * const path4 = "C:\\\Users/Tormak/Desktop/config";
 * const path5 = "C://Users/Tormak/Desktop/config";
 * ```
 */
export function isValidWindowsPath(path: string): boolean {
  // ! This allows actual windows paths, i.e. with backslashes
  // const match = path.match(/^[a-zA-Z]:(?:(?:\\{2}|\/)[\w-]+)+(?:(?:\\{2}|\/)[\w-]+\.[a-zA-Z0-9]+)?$/);
  const match = path.match(/^[a-zA-Z]:(?:\/[\w-]+)+(?:\/[\w-]+\.[a-zA-Z0-9]+)?$/);
  return !!match && match.length > 0;
}

/**
 * Checks if a string is a valid absolute path on Linux.
 * @param path The path to validate.
 * @returns True if the path is a valid absolute linux path.
 * 
 * @example
 * ```ts
 * // Valid Absolute Linux paths:
 * const path1 = "/usr/Tormak/home/Desktop/config";
 * 
 * // Invalid Absolute Linux paths:
 * const path2 = "//usr/Tormak/home/Desktop/config";
 * const path3 = "\\usr/Tormak/home/Desktop/config";
 * ```
 */
export function isValidLinuxPath(path: string): boolean {
  const match = path.match(/^(\/(?:[a-zA-Z0-9_-]+)(?:\/[a-zA-Z0-9_-]+)*)?$/);
  return !!match && match.length > 0;
}