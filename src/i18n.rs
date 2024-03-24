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
    langid!("uk-ua")
];

pub static mut LANG: LanguageIdentifier = langid!("en-us");

/// Set launcher language
pub fn set_lang(lang: LanguageIdentifier) -> anyhow::Result<()> {
    if SUPPORTED_LANGUAGES.iter().any(|item| item.language == lang.language) {
        unsafe {
            LANG = lang
        }

        Ok(())
    }

    else {
        anyhow::bail!("Language '{lang}' is not supported")
    }
}

/// Get launcher language
pub fn get_lang() -> LanguageIdentifier {
    unsafe { LANG.clone() }
}

/// Get system language or default language if system one is not supported
/// 
/// Checks env variables in following order:
/// - `LC_ALL`
/// - `LC_MESSAGES`
/// - `LANG`
pub fn get_default_lang() -> LanguageIdentifier {
    let current = std::env::var("LC_ALL")
        .unwrap_or_else(|_| std::env::var("LC_MESSAGES")
        .unwrap_or_else(|_| std::env::var("LANG")
        .unwrap_or_else(|_| String::from("en_us"))))
        .to_ascii_lowercase();

    for lang in SUPPORTED_LANGUAGES {
        if current.starts_with(lang.language.as_str()) {
            return lang.clone();
        }
    }

    get_lang()
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

            #[allow(unused_unsafe)]
            $crate::i18n::LOCALES
                .lookup(unsafe { &$crate::i18n::LANG }, $id)
                .expect(&format!("Failed to find a message with given id: {}", stringify!($id)))
        }
    };

    ($id:expr, { $($key:literal = $value:expr),* }) => {
        {
            use std::collections::HashMap;

            use fluent_templates::Loader;
            use fluent_templates::fluent_bundle::FluentValue;

            let mut args = HashMap::new();

            $(
                args.insert($key, FluentValue::from($value));
            )*

            #[allow(unused_unsafe)]
            $crate::i18n::LOCALES
                .lookup_complete(unsafe { &$crate::i18n::LANG }, $id, Some(&args))
                .expect(&format!("Failed to find a message with given id: {}", stringify!($id)))
        }
    };
}
