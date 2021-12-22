type VoicePack = {
    language: string;
    name: string;
    path: string;
    size: string;
    md5: string;
}

type Latest = {
    name: string;
    version: string;
    path: string;
    size: string;
    md5: string;
    entry: string;
    voice_packs: VoicePack[];
    decompressed_path: string;
    segments: any[];
}

type Diff = {
    name: string;
    version: string;
    path: string;
    size: string;
    md5: string;
    is_recommended_update: boolean;
    voice_packs: VoicePack[];
}

type Game = {
    latest: Latest;
    diffs: Diff[];
}

type Plugins = {
    name: string;
    version: string;
    path: string;
    size: string;
    md5: string;
    entry: string;
}

type Plugin = {
    plugins: Plugins[];
    version: string;
}

type DeprecatedPackage = {
    name: string;
    md5: string;
}

type Data = {
    game: Game;
    plugin: Plugin;
    web_url: string;
    force_update?: any;
    pre_download_game?: any;
    deprecated_packages: DeprecatedPackage[];
    sdk?: any;
}

type ServerResponse = {
    retcode: number;
    message: string;
    data: Data;
};

export default ServerResponse;

export type {
    VoicePack,
    Latest,
    Diff,
    Game,
    Plugins,
    Plugin,
    DeprecatedPackage,
    Data,
    ServerResponse
};
