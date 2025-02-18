import { emulators } from "@stores/State";
import { error } from '@sveltejs/kit';
import { get } from "svelte/store";

export async function load({ params }) {
  if (Object.keys(get(emulators)).includes(params.emulator)) {
    return {
      emulator: params.emulator
    }
  }

  error(404, 'Not found');
};