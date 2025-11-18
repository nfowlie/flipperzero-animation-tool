import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	server: {
		headers: {
			'Access-Control-Allow-Origin': '*',
			'Access-Control-Allow-Credentials': false
		}
	},
	plugins: [sveltekit()]
});
