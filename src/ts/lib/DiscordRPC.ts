const discordRpc = require('discord-rpc');

export class DiscordRPC
{
    protected static readonly clientId = '901534333360304168';

    protected static rpc: any = null;

    public static init ()
    {
        this.rpc = new discordRpc.Client({ transport: 'ipc' });
        
        this.rpc.login({ clientId: this.clientId }).catch(console.error);

        this.rpc.on('ready', () => {
            this.rpc.setActivity({
                details: 'Preparing to launch',
                largeImageKey: 'launcher',
                largeImageText: 'An Anime Game Launcher',
                instance: false
            });
        });
    }

    public static setActivity (activity: any): void
    {
        this.rpc?.setActivity({
            startTimestamp: parseInt(new Date().setDate(new Date().getDate()).toString()),
            instance: false,
            ...activity
        });
    }

    public static isActive (): boolean
    {
        return this.rpc !== null;
    }

    public static close (): void
    {
        this.rpc?.clearActivity();
        this.rpc?.destroy();

        this.rpc = null;
    }
}
