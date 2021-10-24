interface VoicePack
{
    language: string;
    name: string;
    path: string;
    size: string;
    md5: string;
}

interface Latest
{
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

interface Diff
{
    name: string;
    version: string;
    path: string;
    size: string;
    md5: string;
    is_recommended_update: boolean;
    voice_packs: VoicePack[];
}

interface Game
{
    latest: Latest;
    diffs: Diff[];
}

interface Plugins
{
    name: string;
    version: string;
    path: string;
    size: string;
    md5: string;
    entry: string;
}

interface Plugin
{
    plugins: Plugins[];
    version: string;
}

interface DeprecatedPackage
{
    name: string;
    md5: string;
}

interface Data
{
    game: Game;
    plugin: Plugin;
    web_url: string;
    force_update?: any;
    pre_download_game?: any;
    deprecated_packages: DeprecatedPackage[];
    sdk?: any;
}

export default interface GIJSON
{
    retcode: number;
    message: string;
    data: Data;
}