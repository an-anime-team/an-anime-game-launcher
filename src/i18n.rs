use fluent_templates::Loader;
use unic_langid::{langid, LanguageIdentifier};

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./assets/locales",
        fallback_language: "en"
    };
}

pub static mut LANG: LanguageIdentifier = langid!("en");

pub fn tr(id: &str) -> String {
    unsafe {
        LOCALES
            .lookup(&LANG, id)
            .expect("Failed to get message with given id")
    }
}
