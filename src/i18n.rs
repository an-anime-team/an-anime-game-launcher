use std::sync::OnceLock;
use unic_langid::{langid, LanguageIdentifier};

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./assets/locales",
        core_locales: "./assets/locales/common.ftl",
        fallback_language: "en"
    };
}

/// Map of supported languages
pub const SUPPORTED_LANGUAGES: &[LanguageIdentifier] = &[
    langid!("en-us"),
    langid!("ru-ru"),
    langid!("de-de"),
    langid!("fr-fr"),
    langid!("es-es"),
    langid!("tr-tr"),
    langid!("it-it"),
    langid!("id-id"),
    langid!("zh-cn"),
    langid!("ja-jp"),
    langid!("ko-kr"),
    langid!("hu-hu"),
    langid!("sv-se"),
    langid!("pt-br"),
    langid!("pl-pl"),
    langid!("vi-vn"),
    langid!("nl-nl"),
    langid!("uk-ua"),
    langid!("th-th"),
    langid!("cs-cz")
];

/// Fallback used if the system language is not supported
static FALLBACK: LanguageIdentifier = langid!("en-us");

pub static LANG: OnceLock<LanguageIdentifier> = OnceLock::new();

/// Set launcher language
pub fn set_lang(lang: LanguageIdentifier) -> anyhow::Result<()> {
    if SUPPORTED_LANGUAGES.iter().any(|item| item.language == lang.language) {
        LANG.set(lang).expect("Can't overwrite language!");

        Ok(())
    }

    else {
        anyhow::bail!("Language '{lang}' is not supported")
    }
}

/// Get launcher language
pub fn get_lang() -> &'static LanguageIdentifier {
    LANG.get().expect("Language hasn't been initialized!")
}

/// Get system language or default language if system one is not supported
///
/// Checks env variables in following order:
/// - `LC_ALL`
/// - `LC_MESSAGES`
/// - `LANG`
pub fn get_default_lang() -> &'static LanguageIdentifier {
    let current = std::env::var("LC_ALL")
        .unwrap_or_else(|_| std::env::var("LC_MESSAGES")
        .unwrap_or_else(|_| std::env::var("LANG")
        .unwrap_or_else(|_| String::from("en_us"))))
        .to_ascii_lowercase();

    for lang in SUPPORTED_LANGUAGES {
        if current.starts_with(lang.language.as_str()) {
            return lang;
        }
    }

    &FALLBACK
}

pub fn format_lang(lang: &LanguageIdentifier) -> String {
    format!("{}-{}", lang.language, match lang.region {
        Some(region) => region.to_string().to_ascii_lowercase(),
        None => lang.language.to_string()
    })
}

#[macro_export]
/// Get translated message by key, with optional translation parameters
///
/// # Examples:
///
/// Without parameters:
///
/// ```no_run
/// println!("Translated message: {}", tr!("launch"));
/// ```
///
/// With parameters:
///
/// ```no_run
/// println!("Translated message: {}", tr!("game-outdated", {
///     "latest" = "3.3.0"
/// }));
/// ```
macro_rules! tr {
    ($id:expr) => {
        {
            use fluent_templates::Loader;

            $crate::i18n::LOCALES.lookup($crate::i18n::get_lang(), $id)
        }
    };

    ($id:expr, { $($key:literal = $value:expr),* }) => {
        {
            use std::collections::HashMap;
            use fluent_templates::fluent_bundle::FluentValue;

            let mut args = HashMap::new();

            $(
                args.insert($key, FluentValue::from($value));
            )*

            $crate::i18n::LOCALES.lookup_no_default_fallback($crate::i18n::get_lang(), $id, Some(&args))
                .unwrap_or_default()
        }
    };
}
