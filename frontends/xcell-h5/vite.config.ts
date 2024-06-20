import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig(async () => ({
	plugins: [vue()],

	clearScreen: false,
	server: {
		port: 5177,
		strictPort: false,
		watch: {
			ignored: ["**/src-tauri/**"],
		},
	},
}));
