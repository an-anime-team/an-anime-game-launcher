use fluent_templates::Loader;
use unic_langid::{langid, LanguageIdentifier};

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./assets/locales",
        fallback_language: "en"
    };
}

pub static mut LANG: LanguageIdentifier = langid!("en");

#[allow(clippy::expect_fun_call)]
pub fn tr(id: &str) -> String {
    unsafe {
        LOCALES
            .lookup(&LANG, id)
            .expect(&format!("Failed to find message with a given id: {id}"))
    }
}

#[allow(clippy::expect_fun_call)]
pub fn tr_args<I, T>(id: &str, args: I) -> String
where
    I: IntoIterator<Item = (T, fluent_templates::fluent_bundle::FluentValue<'static>)>,
    T: AsRef<str> + std::hash::Hash + Eq
{
    unsafe {
        LOCALES
            .lookup_with_args(&LANG, id, &std::collections::HashMap::from_iter(args.into_iter()))
            .expect(&format!("Failed to find message with a given id: {id}"))
    }
}
