import { systems } from "@stores/State";
import { error } from '@sveltejs/kit';
import { get } from "svelte/store";

export async function load({ params }) {
	if (Object.keys(get(systems)).includes(params.system)) {
    return {
      system: params.system
    }
  }

  error(404, 'Not found');
};