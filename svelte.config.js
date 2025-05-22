import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from "svelte-adapter-bun";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
    alias: {
      "@interactables": "./src/components/interactables",
      "@layout": "./src/components/layout",
      "@navigation": "./src/components/navigation",
      "@views/*": "./src/components/views/*",
      "@component-utils": "./src/components/utils",
      "@stores/*": "./src/stores/*",
      "@services": "./src/lib/services",
      "@models": "./src/lib/models",
      "@directives": "./src/lib/directives",
      "@menus": "./src/lib/context-menus",
      "@utils": "./src/lib/utils",
      "@types": "./src/lib/types",
      "@icons": "./src/lib/icons"
    }
	}
};

export default config;
