declare module GIJSON {

    export interface VoicePack {
        language: string;
        name: string;
        path: string;
        size: string;
        md5: string;
    }

    export interface Latest {
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

    export interface Diff {
        name: string;
        version: string;
        path: string;
        size: string;
        md5: string;
        is_recommended_update: boolean;
        voice_packs: VoicePack[];
    }

    export interface Game {
        latest: Latest;
        diffs: Diff[];
    }

    export interface Plugins {
        name: string;
        version: string;
        path: string;
        size: string;
        md5: string;
        entry: string;
    }

    export interface Plugin {
        plugins: Plugins[];
        version: string;
    }

    export interface DeprecatedPackage {
        name: string;
        md5: string;
    }

    export interface Data {
        game: Game;
        plugin: Plugin;
        web_url: string;
        force_update?: any;
        pre_download_game?: any;
        deprecated_packages: DeprecatedPackage[];
        sdk?: any;
    }

    export interface Type {
        retcode: number;
        message: string;
        data: Data;
    }

}