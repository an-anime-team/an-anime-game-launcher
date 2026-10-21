// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use std::sync::LazyLock;

use agl_locale::unic_langid::LanguageIdentifier;

use crate::STARTUP_LANG;

/// Languages the launcher ships translations for. The order defines the
/// order of the entries in the language picker.
pub static SUPPORTED_LANGUAGES: LazyLock<Vec<LanguageIdentifier>> = LazyLock::new(|| {
    [
        "en-us",
        "ru-ru",
        "de-de",
        "fr-fr",
        "es-es",
        "pt-br",
        "it-it",
        "pl-pl",
        "tr-tr",
        "uk-ua",
        "zh-cn",
        "zh-tw",
        "ja-jp",
        "ko-kr",
        "vi-vn",
        "id-id",
        "th-th",
        "cs-cz",
        "hu-hu",
        "nl-nl",
        "sv-se"
    ]
    .into_iter()
    .map(|lang| lang.parse().expect("invalid supported language identifier"))
    .collect()
});

/// Load launcher's translations into the `agl-locale` bundle.
///
/// Must be called before any `i18n!` call.
pub fn init() {
    agl_locale::include_i18n!(
        "../assets/locales/components.toml",
        "../assets/locales/enhancements.toml",
        "../assets/locales/environment.toml",
        "../assets/locales/errors.toml",
        "../assets/locales/first_run.toml",
        "../assets/locales/game.toml",
        "../assets/locales/gamescope.toml",
        "../assets/locales/general.toml",
        "../assets/locales/main.toml",
        "../assets/locales/sandbox.toml"
    );
}

/// Get launcher's language identifier.
pub fn get_lang() -> LanguageIdentifier {
    (*STARTUP_LANG).clone()
}

/// Convert a language identifier to its `ll[-cc]` code, e.g. `zh-cn`.
pub fn format_lang(lang: &LanguageIdentifier) -> String {
    match lang.region {
        Some(region) => format!("{}-{}", lang.language, region.to_string().to_ascii_lowercase()),
        None => lang.language.to_string()
    }
}

/// Get translation of a string in the launcher's language.
pub fn translate(key: impl AsRef<str>) -> String {
    let key = key.as_ref();

    let bundle = agl_locale::DEFAULT_BUNDLE.read()
        .expect("failed to lock default translations bundle");

    bundle.get_translation(key, &*STARTUP_LANG)
        .map(String::from)
        .unwrap_or_else(|| key.to_string())
}

/// Get translation of a string in the launcher's language, substituting
/// `{arg}` placeholders.
pub fn translate_with_args<K, V>(
    key: impl AsRef<str>,
    args: impl IntoIterator<Item = (K, V)>
) -> String
where
    K: AsRef<str>,
    V: AsRef<str>
{
    let key = key.as_ref();
    let args = args.into_iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
        .collect::<HashMap<_, _>>();

    agl_locale::DEFAULT_BUNDLE.read()
        .expect("failed to lock default translations bundle")
        .get_translation_with_args(key, &*STARTUP_LANG, args)
        .unwrap_or_else(|| key.to_string())
}

/// Custom `i18n` macro based on the `agl-locale`'s one which respects
/// launcher's config file.
///
/// - `i18n("string_key") -> String`
/// - `i18n("string_key", { "arg" => "value", ... }) -> String`
#[macro_export]
macro_rules! i18n {
    ($key:expr) => {
        $crate::i18n::translate($key)
    };

    ($key:expr, {$( $arg_key:expr => $arg_value:expr $(,)* )+}) => {
        $crate::i18n::translate_with_args($key, [
            $( (
                stringify!($arg_key).trim_matches('"'),
                $arg_value.to_string()
            ) ),+
        ])
    };
}
