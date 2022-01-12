type DebugOptions = {
    /**
     * Some random-generated thread id
     */
    thread?: number;

    /**
     * Some function name
     */
    function?: string;

    /**
     * Some log message
     */
    message: string|string[]|object;
};

type LogRecord = {
    time: number;
    log: string[];
};

export type { DebugOptions, LogRecord };
