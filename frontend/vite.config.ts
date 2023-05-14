import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: process.env.BACKEND_URL,
				secure: false,
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, ''),
			}
		}
	}
});
