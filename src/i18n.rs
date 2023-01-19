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
