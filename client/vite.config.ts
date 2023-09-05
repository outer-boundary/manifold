import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";
import { internalIpV4 } from "internal-ip";

export default defineConfig(async () => {
	const config = {
		plugins: [sveltekit()],
		server: {
			host: "0.0.0.0",
			port: 5173,
			strictPort: true,
			hmr: {
				protocol: "ws",
				host: 'localhost',
			},
			// Allow serving files from the public folder
			fs: {
				allow: ["public"]
			}
		},
		build: {
			// // The Windows webview (WebView2) is supposed to update itself automatically, so it's safe to target the latest 3 versions (i.e. Blink 108).
			// // The macOS webview (WebKit) only updates with new OS versions, so the build target is more conservative; the earliest Safari version for macOS Big Sur is 14 (i.e. WebKit 610.2.11).
			// // The Linux webview (WebKitGTK) is based on WebKit, so the target is the same as macOS.
			// // Reference: https://discord.com/channels/616186924390023171/1082055425907753063
			// target: process.env.TAURI_PLATFORM == 'windows' ? 'edge108' : ['es2021', 'safari14'],

			// By default, Vite inlines assets smaller than 4 KiB as base64 (see https://vitejs.dev/config/build-options.html#build-assetsinlinelimit).
			// These assets are blocked by the CSP in production builds, so inlining is explicitly disabled.
			assetsInlineLimit: 0
		},
		test: {
			include: ["src/**/*.{test,spec}.{js,ts}"]
		},
		css: {
			preprocessorOptions: {
				scss: {
					additionalData: `@import './src/styles/globalStyles.scss';`
				}
			}
		}
	};

	return config;
});
