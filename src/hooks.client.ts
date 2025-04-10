// import type { HandleClientError } from "@sveltejs/kit";

export async function handleError({ error, event, status, message }) {
	return {
		message: message,
		code: status
	};
}