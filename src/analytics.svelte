
<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, Windows, fetch, IPC } from './empathize';

    import constants from './ts/Constants';

    import YanfeiIcon from './assets/images/yanfei.png';

    import LeftCheckbox from './components/LeftCheckbox.svelte';
    import Button from './components/Button.svelte';

    let shareCountry = true;

    const closeWindow = async () => {
        await Neutralino.filesystem.removeFile(`${await constants.paths.launcherDir}/.analytics`);

        Neutralino.app.exit();
    };

    const switchTheme = (theme: string) => {
        if (theme === 'system')
            theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

        document.body.setAttribute('data-theme', theme as string);
    };

    // Auto theme switcher
    Configs.get('theme').then((theme) => switchTheme(theme as string));

    onMount(async () => {
        await Windows.current.show();
        await Windows.current.center(700, 460);

        // This thing will fix window resizing
        // in several cases (wayland + gnome + custom theme)
        const resizer = () => {
            if (window.innerWidth < 640)
                setTimeout(resizer, 10);

            else
            {
                Windows.current.setSize({
                    width: 700 + (700 - window.innerWidth),
                    height: 460 + (460 - window.innerHeight),
                    resizable: false
                });
            }
        }

        setTimeout(resizer, 10);
    });

    Neutralino.events.on('windowClose', async () => {
        await IPC.write('analytics-close');

        Neutralino.app.exit();
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <div class="header">
            <img src={YanfeiIcon} alt="">

            <h2>{$_('analytics.header')}</h2>
        </div>

        <p>{$_('analytics.body.0')}</p>
        <p>{$_('analytics.body.1')}</p>

        <div class="footer">
            <LeftCheckbox
                lang="analytics.actions.share_country.title"
                tooltip={{
                    lang: 'analytics.actions.share_country.hint',
                    direction: 'top',
                    size: 'large'
                }}
                valueChanged={(value) => shareCountry = value}
            />
    
            <div class="actions">
                <!-- svelte-ignore missing-declaration -->
                <Button
                    lang="analytics.actions.participate"
                    primary={true}
                    click={async () => {
                        await fetch(`${constants.uri.analytics}/${shareCountry ? '' : '?hide-geo'}`);

                        closeWindow();
                    }}
                />

                <div class="actions-right">
                    <!-- svelte-ignore missing-declaration -->
                    <Button
                        lang="analytics.actions.skip"
                        click={closeWindow}
                    />

                    <!-- svelte-ignore missing-declaration -->
                    <Button
                        lang="analytics.actions.skip_forever"
                        click={async () => {
                            await Configs.set('skip_analytics', true);

                            closeWindow();
                        }}
                    />
                </div>
            </div>
        </div>
    </main>
{/if}
