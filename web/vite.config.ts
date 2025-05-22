import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

  server: {
    port: 1420,
    watch: {
      ignored: ["**/server/**", "docs/**", "docker/**"],
    },
  },

  envPrefix: ["VITE_", "NRM_"],

  build: {
    // don't minify for debug builds
    minify: !process.env.NRM_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.NRM_DEBUG,

    rollupOptions: {
      external: [
        "/public/readme-images"
      ],
    },
  },

  define: {
    'NRM_FRONTEND_VERSION': JSON.stringify(process.env.npm_package_version),
  }
});
