import type { NotificationsOptions } from '../types/Notifications';

import Process from '../neutralino/Process';

declare const Neutralino;

export default class Notifications
{
    /**
     * Show notification
     */
    public static show(options: NotificationsOptions)
    {
        let command = `notify-send "${Process.addSlashes(options.title)}" "${Process.addSlashes(options.body)}"`;

        // Specify notification icon
        if (options.icon)
            command += ` -i "${Process.addSlashes(options.icon)}"`;

        // Specify notification duration
        if (options.duration)
            command += ` -d ${options.duration}`;

        // Specify notification importance
        if (options.importance)
            command += ` -u ${options.importance}`;

        Neutralino.os.execCommand(command, {
            background: true
        });
    }
};
