import { writable } from "svelte/store";
import { localStorageWritable } from "@utils";

export const isSignedIn = writable(false);
export const username = writable("");
export const rememberMe = localStorageWritable("rememberMe", true);