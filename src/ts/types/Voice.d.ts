type VoiceLang =
    | 'en-us'
    | 'zh-cn'
    | 'ja-jp'
    | 'ko-kr';

type InstalledVoice = {
    installed: VoiceLang[];
    active: VoiceLang|null;
};

export type {
    VoiceLang,
    InstalledVoice
};
