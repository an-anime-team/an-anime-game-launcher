type NotificationsOptions = {
    title: string;
    body: string;

    /**
     * Icon name or path
     */
    icon?: string;

    /**
     * Number of seconds this notification
     * will be visible
     */
    duration?: number;

    /**
     * Importance of the notification
     * 
     * @default "normal"
     */
    importance?: 'low' | 'normal' | 'critical';
};

export type { NotificationsOptions };
