import { writable } from "svelte/store";

export const updateManifest = writable<Update | null>(null);