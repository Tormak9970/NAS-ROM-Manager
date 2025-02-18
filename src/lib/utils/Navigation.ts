import { goto } from "$app/navigation";
import type { BackendErrorType } from "@types";

/**
 * Displays an error.
 * @param message The error message.
 * @param fix The error fix.
 * @param type The type of error.
 */
export function showError(message: string, fix: string, type: BackendErrorType) {
  goto(`/error?message=${message}&fix=${fix}&type=${type}`);
}

/**
 * Navigates to the provided rom.
 * @param id The id of the ROM to navigate to.
 */
export function goToROM(id: string) {
  goto(`/library/${id}`);
}

/**
 * Navigates to the provided system.
 * @param system The system to navigate to.
 */
export function goToSystem(system: string) {
  goto(`/systems/${system}`);
}

/**
 * Navigates to the provided emulator.
 * @param emulator The emulator to navigate to.
 */
export function goToEmulator(emulator: string) {
  goto(`/emulators/${emulator}`);
}