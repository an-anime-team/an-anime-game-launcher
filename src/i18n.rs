use fluent_templates::Loader;
use unic_langid::{langid, LanguageIdentifier};

fluent_templates::static_loader! {
    static LOCALES = {
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
    langid!("fr-fr")
];

static mut LANG: LanguageIdentifier = langid!("en-us");

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
    let lang = std::env::var("LC_ALL")
        .unwrap_or_else(|_| std::env::var("LC_MESSAGES")
        .unwrap_or_else(|_| std::env::var("LANG")
        .unwrap_or_else(|_| String::from("en_US.UTF-8"))));

    lang.parse().unwrap_or_else(|_| langid!("en-us"))
}

pub fn format_lang(lang: &LanguageIdentifier) -> String {
    format!("{}-{}", lang.language, match lang.region {
        Some(region) => region.to_string().to_ascii_lowercase(),
        None => lang.language.to_string()
    })
}

/// Get translated message by key
/// 
/// ```no_run
/// println!("Translated message: {}", tr("launch"));
/// ``` 
#[allow(clippy::expect_fun_call)]
pub fn tr(id: &str) -> String {
    unsafe {
        LOCALES
            .lookup(&LANG, id)
            .expect(&format!("Failed to find message with given id: {id}"))
    }
}

/// Get translated message by key with filled arguments
/// 
/// ```no_run
/// println!("Translated message: {}", tr_args("game-outdated", [
///     ("latest", "3.3.0".into())
/// ]));
/// ``` 
#[allow(clippy::expect_fun_call)]
pub fn tr_args<I, T>(id: &str, args: I) -> String
where
    I: IntoIterator<Item = (T, fluent_templates::fluent_bundle::FluentValue<'static>)>,
    T: AsRef<str> + std::hash::Hash + Eq
{
    unsafe {
        LOCALES
            .lookup_with_args(&LANG, id, &std::collections::HashMap::from_iter(args.into_iter()))
            .expect(&format!("Failed to find message with given id: {id}"))
    }
}
