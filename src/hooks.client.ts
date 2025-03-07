import { AuthController, WebsocketController } from "@controllers";
import { rememberMe } from "@stores/Auth";
import type { ClientInit } from '@sveltejs/kit';
import { get } from "svelte/store";

export const init: ClientInit = async () => {
  await new Promise<void>((resolve) => {
    WebsocketController.init(
      async () => {
        const user = sessionStorage.getItem("user");
        const hash = sessionStorage.getItem("hash");

        if (user && hash && get(rememberMe)) {
          await AuthController.authenticate(user, hash);
        }

        resolve();
      },
      AuthController.logout
    );
  });
};