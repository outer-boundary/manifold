import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		csp: {
			mode: "hash",
			directives: {
				"default-src": ["self", "ws://10.1.1.123:5183/"],
				"img-src": ["self", "https://external-content.duckduckgo.com"],
				"script-src": ["self", "unsafe-inline"],
				"style-src": ["self", "unsafe-inline"],
				// Vite uses WebSockets for HMR, so WebSocket connections to localhost:5173 are whitelisted during development
				"connect-src":
					process.env.TAURI_DEBUG === "true"
						? ["self"]
						: [
								"self",
								"ws://192.168.20.0:5183",
								"ws://10.1.1.123:5183/",
								"ws://localhost:5173",
								"http://10.1.1.123:5183/",
								"api.iconify.design",
								"api.simplesvg.com",
								"api.unisvg.com"
						  ]
			}
		}
	}
};

export default config;
