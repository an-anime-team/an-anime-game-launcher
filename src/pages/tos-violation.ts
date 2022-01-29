import '../i18n';
import App from '../tos-violation.svelte';

declare const Neutralino;

Neutralino.init();

const app = new App({
    target: document.getElementById('app')!
});

export default app;
