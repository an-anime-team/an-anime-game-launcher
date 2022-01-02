type VoiceLang =
    | 'en-us'
    | 'zh-cn'
    | 'ja-jp'
    | 'ko-kr';

type InstalledVoiceInfo = {
    lang: VoiceLang;
    version: string|null;
};

type VoiceInfo = {
    installed: InstalledVoiceInfo[];
    active: InstalledVoiceInfo|null;
};

export type {
    VoiceLang,
    InstalledVoiceInfo,
    VoiceInfo
};
