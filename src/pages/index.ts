import '../i18n';
import App from '../index.svelte';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('ready', () => import('../defaultSettings'));

const app = new App({
    target: document.getElementById('app')!
});

export default app;
