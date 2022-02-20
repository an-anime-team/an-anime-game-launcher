<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, Windows, IPC } from './empathize';

    import constants from './ts/Constants';
    import Locales from './ts/launcher/Locales';

    import Button from './components/Button.svelte';

    Neutralino.events.on('windowClose', async () => {
        await IPC.write({
            type: 'tos-violation',
            agreed: false
        });

        Neutralino.app.exit();
    });

    onMount(() => {
        Windows.current.show();
        Windows.current.center();

        const title = Locales.translate('tos_violation.title') as string|null;

        if (title)
            Windows.current.setTitle(title);
    });

    let timer = 30;

    const updateTimer = () => {
        if (--timer > 0)
            setTimeout(updateTimer, 1000);
    };

    setTimeout(updateTimer, 1000);

    // Auto theme switcher
    Configs.get('theme').then((theme) => {
        if (theme === 'system')
            theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

        document.body.setAttribute('data-theme', theme as string);
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <h1>{$_('tos_violation.heading')}</h1>

        <p>{$_('tos_violation.body', {
            values: {
                company: constants.placeholders.uppercase.company,
                company_alterego: constants.placeholders.uppercase.company_alterego,
                game: constants.placeholders.uppercase.full.global
            }
        })}</p>

        <div class="action-buttons">
            <!-- svelte-ignore missing-declaration -->
            <Button
                lang={timer > 0 ? timer.toString() : 'tos_violation.buttons.ok.title'}
                tooltip={timer > 0 ? { lang: 'tos_violation.buttons.ok.tooltip', direction: 'top' } : undefined}
                primary={true}
                disabled={timer > 0}

                click={async () => {
                    await IPC.write({
                        type: 'tos-violation',
                        agreed: true
                    });

                    Neutralino.app.exit();
                }}
            />

            <!-- svelte-ignore missing-declaration -->
            <Button lang="tos_violation.buttons.cancel" primary={true} click={async () => {
                await IPC.write({
                    type: 'tos-violation',
                    agreed: false
                });

                Neutralino.app.exit();
            }} />

            <div class="buttons-right">
                <!-- svelte-ignore missing-declaration -->
                <Button lang="tos_violation.buttons.discord" click={() => Neutralino.os.open(constants.uri.discord)} />
            </div>
        </div>
    </main>
{/if}
