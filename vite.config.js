import { defineConfig } from 'vite';

import vue from '@vitejs/plugin-vue';

// https://vitejs.dev/config/
export default defineConfig({
    server: {
        port: 8080
    },
    plugins: [
        vue()
    ],
    base: '',
    build: {
        outDir: 'bundle',
        rollupOptions: {
            input: [
                'index.html',
                'about.html'
            ]
        }
    }
});
