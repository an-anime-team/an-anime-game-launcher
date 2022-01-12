type VoiceLang =
    | 'en-us'
    | 'zh-cn'
    | 'ja-jp'
    | 'ko-kr';

type InstalledVoice = {
    lang: VoiceLang;
    version: string|null;
};

export type {
    VoiceLang,
    InstalledVoice
};
