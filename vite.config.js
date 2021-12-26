import { defineConfig } from 'vite';

import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
    server: {
        port: 8080
    },
    plugins: [svelte()],
    base: '',
    build: {
        outDir: 'bundle',
        rollupOptions: {
            input: [
                'index.html',
                'settings.html'
            ]
        }
    }
});
