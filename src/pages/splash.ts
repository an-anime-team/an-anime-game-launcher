import '../i18n';
import App from '../splash.svelte';



Neutralino.init();

const app = new App({
    target: document.getElementById('app')!
});

export default app;
