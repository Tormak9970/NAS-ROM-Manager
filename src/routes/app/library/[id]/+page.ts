import { roms } from "@stores/State";
import { error } from '@sveltejs/kit';
import { get } from "svelte/store";

export async function load({ params }) {
	if (Object.keys(get(roms)).includes(params.id)) {
    return {
      id: params.id
    }
  }

  error(404, 'Not found');
};