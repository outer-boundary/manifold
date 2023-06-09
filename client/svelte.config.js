import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess({
		scss: {
			prependData: `@import './src/styles/globalStyles.scss';`,      
		}
	}),

	kit: {
		adapter: adapter(),
		csp: {
			mode: "hash",
			directives: {
				"default-src": ["self", "ws://10.1.1.123:5183/"],
				"img-src": ["self"],
				"script-src": ["self", "unsafe-inline"],
				"style-src": ["self", "unsafe-inline"],
				// Vite uses WebSockets for HMR, so WebSocket connections to localhost:5173 are whitelisted during development
				"connect-src":
					process.env.TAURI_DEBUG === "true"
						? ["self", "ws://10.1.1.123:5183/", "ws://localhost:5173"]
						: undefined
			}
		}
	}
};

export default config;
