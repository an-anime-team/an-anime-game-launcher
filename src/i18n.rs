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
pub const SUPPORTED_LANGUAGES: &[(&str, LanguageIdentifier)] = &[
    ("en-us", langid!("en")),
    ("ru-ru", langid!("ru"))
];

static mut LANG: LanguageIdentifier = langid!("en");

/// Set launcher language
pub fn set_lang<T: AsRef<str>>(lang: T) -> anyhow::Result<()> {
    for (key, id) in SUPPORTED_LANGUAGES {
        if key == &lang.as_ref() {
            unsafe {
                LANG = id.to_owned()
            }

            return Ok(());
        }
    }

    anyhow::bail!("Language {} is not supported", lang.as_ref())
}

/// Get launcher language
pub fn get_lang() -> &'static str {
    for (key, lang) in SUPPORTED_LANGUAGES {
        if lang == unsafe { &LANG } {
            return key;
        }
    }

    unreachable!()
}

/// Get system language or default language if system one is not supported
/// 
/// Checks env variables in following order:
/// - `LC_ALL`
/// - `LC_MESSAGES`
/// - `LANG`
pub fn get_default_lang() -> String {
    let lang = std::env::var("LC_ALL")
        .unwrap_or_else(|_| std::env::var("LC_MESSAGES")
        .unwrap_or_else(|_| std::env::var("LANG")
        .unwrap_or_else(|_| String::from("en_US.UTF-8"))));

    let lang = lang.split('.')
        .next()
        .unwrap_or("en_US")
        .replace('_', "-")
        .to_ascii_lowercase();

    for (key, _) in SUPPORTED_LANGUAGES {
        if key == &lang {
            return lang;
        }
    }

    String::from("en-us")
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
