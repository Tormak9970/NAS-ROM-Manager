import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

  server: {
    port: 1420,
    // strictPort: true,
    // host: "0.0.0.0",
    // hmr: {
    //   protocol: "ws",
    //   host: await internalIpV4(),
    //   port: 1421,
    // },
    watch: {
      // 3. tell vite to ignore watching `src-rust`
      ignored: ["**/src-rust/**"],
    },
  }
});
