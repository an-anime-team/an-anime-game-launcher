import '../i18n';
import App from '../tos-violation.svelte';



Neutralino.init();

const app = new App({
    target: document.getElementById('app')!
});

export default app;
