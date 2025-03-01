import { writable, type Updater, type Writable } from "svelte/store";

import { browser } from '$app/environment';

export type LocalStorageWritable<T> = Writable<T>;

/**
 * A wrapper around Svelte's stores that saves the value to localStorage.
 * @param key The store's localStorage key.
 * @param defaultValue The store's default value.
 * @returns The store.
 */
export function localStorageWritable<T>(key: string, defaultValue: T): LocalStorageWritable<T> {
  if (!browser) {
    return writable<T>(defaultValue);
  }

  const locallyStored = localStorage.getItem(key);
  const currentValue: T = locallyStored ? JSON.parse(locallyStored).data : defaultValue;

  const { set, update, subscribe } = writable(currentValue);

  return {
    set: (value: T) => {
      localStorage.setItem(key, JSON.stringify({
        data: value
      }));

      set(value);
    },
    update: (updater: Updater<T>) => {
      update((value: T) => {
        const newValue = updater(value);

        localStorage.setItem(key, JSON.stringify({
          data: newValue
        }));

        return newValue;
      })
    },
    subscribe
  }
}