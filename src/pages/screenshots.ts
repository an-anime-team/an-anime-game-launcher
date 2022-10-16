import '../i18n';

import App from '../screenshots.svelte';

declare const Neutralino;

Neutralino.init();

export default new App({
    target: document.getElementById('app')!
});
