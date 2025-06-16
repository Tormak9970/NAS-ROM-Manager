import { defineConfig } from "vitepress"

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "NAS ROM Manager",
  description: "Documentation for Nas ROM Manager",
  // head: [
	// 	["link", { rel: "icon", href: "/icon.png" }],
	// 	["meta", { name: "description", content: "Docker container and built in Web Application for managing Nginx proxy hosts with a simple, powerful interface, providing free SSL support via Let's Encrypt" }],
	// 	["meta", { property: "og:title", content: "Nginx Proxy Manager" }],
	// 	["meta", { property: "og:description", content: "Docker container and built in Web Application for managing Nginx proxy hosts with a simple, powerful interface, providing free SSL support via Let's Encrypt"}],
	// 	["meta", { property: "og:type", content: "website" }],
	// 	["meta", { property: "og:url", content: "https://nginxproxymanager.com/" }],
	// 	["meta", { property: "og:image", content: "https://nginxproxymanager.com/icon.png" }],
	// 	["meta", { name: "twitter:card", content: "summary"}],
	// 	["meta", { name: "twitter:title", content: "Nginx Proxy Manager"}],
	// 	["meta", { name: "twitter:description", content: "Docker container and built in Web Application for managing Nginx proxy hosts with a simple, powerful interface, providing free SSL support via Let's Encrypt"}],
	// 	["meta", { name: "twitter:image", content: "https://nginxproxymanager.com/icon.png"}],
	// 	["meta", { name: "twitter:alt", content: "Nginx Proxy Manager"}],
	// 	// GA
	// 	['script', { async: 'true', src: 'https://www.googletagmanager.com/gtag/js?id=G-TXT8F5WY5B'}],
	// 	['script', {}, "window.dataLayer = window.dataLayer || [];\nfunction gtag(){dataLayer.push(arguments);}\ngtag('js', new Date());\ngtag('config', 'G-TXT8F5WY5B');"],
	// ],
	// sitemap: {
	// 	hostname: 'https://nginxproxymanager.com'
	// },
	// metaChunk: true,
  srcDir: "./pages",
	outDir: './dist',
	themeConfig: {
		// https://vitepress.dev/reference/default-theme-config
		logo: { src: '/logo.svg', width: 24, height: 24 },
    nav: [
      { text: "Home", link: "/" },
      { text: "Docs", link: "/introduction" }
    ],

    sidebar: [
      {
        text: "Getting Started",
        items: [
          { text: "Introduction", link: "/introduction" },
          { text: "Installation", link: "/getting-started/installation" },
          { text: "Configuration", link: "/getting-started/configuration" }
        ]
      },
      {
        text: "Features",
        items: [
          { text: "ROMs", link: "/features/roms" },
          { text: "ROM Metadata", link: "/features/rom-metadata" },
          { text: "Parsers", link: "/features/parsers" },
          { text: "BIOS Files", link: "/features/bios-files" },
          { text: "Settings", link: "/features/settings" }
        ]
      },
      {
        text: "Contributing",
        link: "/contributing"
      },
      {
        text: "Building NRM",
        link: "/building-nrm"
      },
      {
        text: "Acknowledgements",
        link: "/acknowledgements"
      }
    ],
    footer: {
      message: 'Released under the GNU General Public License Version 3',
      copyright: 'Copyright © 2024-present Travis Lane'
    },
    search: {
			provider: 'local'
		},
    socialLinks: [
      { icon: "github", link: "https://github.com/Tormak9970/NAS-ROM-Manager" }
    ]
  }
})
