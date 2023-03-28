import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		csp: {
			mode: 'hash',
			directives: {
				'default-src': ['none'],
				'img-src': ['self'],
				'script-src': ['self'],
				'style-src': ['self', 'unsafe-inline'],
				// Vite uses WebSockets for HMR, so WebSocket connections to localhost:5173 are whitelisted during development
				'connect-src': process.env.TAURI_DEBUG === 'true' ? ['ws://localhost:5173'] : undefined
			}
		}
	}
};

export default config;
