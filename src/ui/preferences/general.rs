use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::{
    AsyncFactoryVecDeque,
    AsyncFactoryComponent,
    AsyncFactorySender
};

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::config::launcher::LauncherStyle;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::components::*;
use anime_launcher_sdk::wincompatlib::prelude::*;

use super::main::PreferencesAppMsg;
use crate::ui::components;
use crate::ui::components::*;
use crate::i18n::*;
use crate::*;

#[derive(Debug)]
struct VoicePackageComponent {
    locale: VoiceLocale,
    installed: bool,
    sensitive: bool
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for VoicePackageComponent {
    type Init = (VoiceLocale, bool);
    type Input = GeneralAppMsg;
    type Output = GeneralAppMsg;
    type CommandOutput = ();
    type ParentInput = GeneralAppMsg;
    type ParentWidget = adw::ExpanderRow;

    view! {
        root = adw::ActionRow {
            set_title: &tr(&self.locale.to_name().to_ascii_lowercase()),

            add_suffix = &gtk::Button {
                #[watch]
                set_visible: self.installed,

                #[watch]
                set_sensitive: self.sensitive,

                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(GeneralAppMsg::RemoveVoicePackage(index.clone()));
                }
            },

            add_suffix = &gtk::Button {
                #[watch]
                set_visible: !self.installed,

                #[watch]
                set_sensitive: self.sensitive,

                set_icon_name: "document-save-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(GeneralAppMsg::AddVoicePackage(index.clone()));
                }
            }
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            locale: init.0,
            installed: init.1,
            sensitive: true
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        self.installed = !self.installed;

        sender.output(msg);
    }
}

pub struct GeneralApp {
    voice_packages: AsyncFactoryVecDeque<VoicePackageComponent>,

    wine_components: AsyncController<ComponentsList<GeneralAppMsg>>,
    dxvk_components: AsyncController<ComponentsList<GeneralAppMsg>>,

    game_diff: Option<VersionDiff>,
    patch: Option<Patch>,

    style: LauncherStyle,

    languages: Vec<String>,

    downloaded_wine_versions: Vec<(wine::Version, wine::Features)>,
    downloaded_dxvk_versions: Vec<dxvk::Version>,
    allow_dxvk_selection: bool,

    selected_wine_version: u32,
    selected_dxvk_version: u32,

    selecting_wine_version: bool,
    selecting_dxvk_version: bool
}

#[derive(Debug, Clone)]
pub enum GeneralAppMsg {
    /// Supposed to be called automatically on app's run when the latest game version
    /// was retrieved from the API
    SetGameDiff(Option<VersionDiff>),

    /// Supposed to be called automatically on app's run when the latest patch version
    /// was retrieved from remote repos
    SetPatch(Option<Patch>),

    // If one ever wich to change it to accept VoiceLocale
    // I'd recommend to use clone!(@strong self.locale as locale => move |_| { .. })
    // in the VoicePackage component
    AddVoicePackage(DynamicIndex),
    RemoveVoicePackage(DynamicIndex),
    SetVoicePackageSensitivity(DynamicIndex, bool),

    RepairGame,

    UpdateLauncherStyle(LauncherStyle),

    WineRecommendedOnly(bool),
    DxvkRecommendedOnly(bool),

    UpdateDownloadedWine,
    UpdateDownloadedDxvk,

    SelectWine(usize),
    SelectDxvk(usize),

    ResetWineSelection(usize),
    ResetDxvkSelection(usize),

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GeneralApp {
    type Init = ();
    type Input = GeneralAppMsg;
    type Output = PreferencesAppMsg;

    view! {
        adw::PreferencesPage {
            set_title: &tr("general"),
            set_icon_name: Some("applications-system-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("appearance"),

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,

                    set_spacing: 32,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        gtk::ToggleButton {
                            add_css_class: "card",

                            set_width_request: 180,
                            set_height_request: 120,

                            #[watch]
                            set_active: model.style == LauncherStyle::Modern,

                            gtk::Image {
                                set_from_resource: Some("/org/app/images/modern.svg")
                            },

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Modern)
                        },

                        gtk::Label {
                            set_text: &tr("modern"),

                            set_margin_top: 16
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        gtk::ToggleButton {
                            add_css_class: "card",

                            set_width_request: 180,
                            set_height_request: 120,

                            #[watch]
                            set_active: model.style == LauncherStyle::Classic,

                            gtk::Image {
                                set_from_resource: Some("/org/app/images/classic.svg")
                            },

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Classic)
                        },

                        gtk::Label {
                            set_text: &tr("classic"),

                            set_margin_top: 16
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_visible: model.style == LauncherStyle::Classic,

                adw::ActionRow {
                    set_title: &tr("update-background"),
                    set_subtitle: &tr("update-background-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_active: !KEEP_BACKGROUND_FILE.exists(),

                        connect_state_notify => |switch| {
                            #[allow(unused_must_use)]
                            if switch.state() {
                                std::fs::remove_file(KEEP_BACKGROUND_FILE.as_path());
                            } else {
                                std::fs::write(KEEP_BACKGROUND_FILE.as_path(), "");
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("general"),

                adw::ComboRow {
                    set_title: &tr("launcher-language"),
                    set_subtitle: &tr("launcher-language-description"),

                    // TODO: maybe simplify it by some way? e.g. specify such stuff in i18n mod

                    set_model: Some(&gtk::StringList::new(&model.languages.iter().map(|lang| lang.as_str()).collect::<Vec<&str>>())),

                    set_selected: {
                        let selected = crate::i18n::get_lang().language;

                        SUPPORTED_LANGUAGES.iter()
                            .position(|lang| lang.language == selected)
                            .unwrap_or(0) as u32
                    },

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.launcher.language = crate::i18n::format_lang(SUPPORTED_LANGUAGES
                                    .get(row.selected() as usize)
                                    .unwrap_or(&SUPPORTED_LANGUAGES[0]));
    
                                config::update(config);
                            }
                        }
                    }
                },

                #[local_ref]
                voice_packages -> adw::ExpanderRow {
                    set_title: &tr("game-voiceovers")
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_top: 16,

                    gtk::Button {
                        set_label: &tr("repair-game"),

                        connect_clicked => GeneralAppMsg::RepairGame
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("status"),

                adw::ActionRow {
                    set_title: &tr("game-version"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.game_diff.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(current) |
                                VersionDiff::Predownload { current, .. } |
                                VersionDiff::Diff { current, .. } |
                                VersionDiff::Outdated { current, .. } => current.to_string(),

                                VersionDiff::NotInstalled { .. } => tr("game-not-installed")
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.game_diff.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => &["success"],
                                VersionDiff::Predownload { .. } => &["accent"],
                                VersionDiff::Diff { .. } => &["warning"],
                                VersionDiff::Outdated { .. } => &["error"],
                                VersionDiff::NotInstalled { .. } => &[]
                            }

                            None => &["success"]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.game_diff.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => String::new(),
                                VersionDiff::Predownload { current, latest, .. } => tr_args("game-predownload-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Diff { current, latest, .. } => tr_args("game-update-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Outdated { latest, ..} => tr_args("game-outdated", [
                                    ("latest", latest.to_string().into())
                                ]),
                                VersionDiff::NotInstalled { .. } => String::new()
                            }

                            None => String::new()
                        })
                    }
                },

                adw::ActionRow {
                    set_title: &tr("patch-version"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.patch.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available"),
                                Patch::Outdated { current, .. } => tr_args("patch-outdated", [("current", current.to_string().into())]),
                                Patch::Preparation { .. } => tr("patch-preparation"),
                                Patch::Testing { version, .. } |
                                Patch::Available { version, .. } => version.to_string()
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.patch.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => &["error"],
                                Patch::Outdated { .. } |
                                Patch::Preparation { .. } |
                                Patch::Testing { .. } => &["warning"],
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = model.patch.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        &["success"]
                                    } else {
                                        &["warning"]
                                    }
                                }
                            }

                            None => &[]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.patch.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available-tooltip"),
                                Patch::Outdated { current, latest, .. } => tr_args("patch-outdated-tooltip", [
                                    ("current", current.to_string().into()),
                                    ("latest", latest.to_string().into())
                                ]),
                                Patch::Preparation { .. } => tr("patch-preparation-tooltip"),
                                Patch::Testing { .. } => tr("patch-testing-tooltip"),
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = model.patch.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        String::new()
                                    } else {
                                        tr("patch-testing-tooltip")
                                    }
                                }
                            }

                            None => String::new()
                        })
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("wine-version"),

                adw::ComboRow {
                    set_title: &tr("selected-version"),

                    #[watch]
                    #[block_signal(wine_selected_notify)]
                    set_model: Some(&gtk::StringList::new(&model.downloaded_wine_versions.iter().map(|(version, _)| version.title.as_str()).collect::<Vec<&str>>())),

                    #[watch]
                    #[block_signal(wine_selected_notify)]
                    set_selected: model.selected_wine_version,

                    #[watch]
                    set_activatable: !model.selecting_wine_version,

                    #[watch]
                    set_icon_name: if model.selecting_wine_version {
                        Some("process-working-symbolic")
                    } else {
                        None
                    },

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            sender.input(GeneralAppMsg::SelectWine(row.selected() as usize));
                        }
                    } @wine_selected_notify
                },

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("wine-recommended-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        #[block_signal(wine_recommended_notify)]
                        set_state: true,

                        connect_state_notify[sender] => move |switch| {
                            if is_ready() {
                                sender.input(GeneralAppMsg::WineRecommendedOnly(switch.state()));
                            }
                        } @wine_recommended_notify
                    }
                }
            },

            add = &adw::PreferencesGroup {
                add = model.wine_components.widget(),
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("dxvk-version"),

                #[watch]
                set_description: Some(&if !model.allow_dxvk_selection {
                    tr("dxvk-selection-disabled")
                } else {
                    String::new()
                }),

                #[watch]
                set_sensitive: model.allow_dxvk_selection,

                adw::ComboRow {
                    set_title: &tr("selected-version"),

                    #[watch]
                    #[block_signal(dxvk_selected_notify)]
                    set_model: Some(&gtk::StringList::new(&model.downloaded_dxvk_versions.iter().map(|version| version.name.as_str()).collect::<Vec<&str>>())),

                    #[watch]
                    #[block_signal(dxvk_selected_notify)]
                    set_selected: model.selected_dxvk_version,

                    #[watch]
                    set_activatable: !model.selecting_dxvk_version,

                    #[watch]
                    set_icon_name: if model.selecting_dxvk_version {
                        Some("process-working-symbolic")
                    } else {
                        None
                    },

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            sender.input(GeneralAppMsg::SelectDxvk(row.selected() as usize));
                        }
                    } @dxvk_selected_notify
                },

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("dxvk-recommended-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        #[block_signal(dxvk_recommended_notify)]
                        set_state: true,

                        connect_state_notify[sender] => move |switch| {
                            if is_ready() {
                                sender.input(GeneralAppMsg::DxvkRecommendedOnly(switch.state()));
                            }
                        } @dxvk_recommended_notify
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_sensitive: model.allow_dxvk_selection,

                add = model.dxvk_components.widget(),
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing general settings");

        let mut model = Self {
            voice_packages: AsyncFactoryVecDeque::new(adw::ExpanderRow::new(), sender.input_sender()),

            wine_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.wine.builds.clone(),
                        groups: wine::get_groups(&CONFIG.components.path).unwrap_or_default()
                            .into_iter()
                            .map(|mut group| {
                                group.versions = group.versions.into_iter().take(12).collect();

                                let mut group: ComponentsListGroup = group.into();

                                if group.versions.len() > 6 {
                                    for i in 6..group.versions.len() {
                                        group.versions[i].recommended = false;
                                    }
                                }

                                group
                            })
                            .collect()
                    },
                    on_downloaded: Some(GeneralAppMsg::UpdateDownloadedWine),
                    on_deleted: Some(GeneralAppMsg::UpdateDownloadedWine)
                })
                .forward(sender.input_sender(), std::convert::identity),

            dxvk_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.dxvk.builds.clone(),
                        groups: dxvk::get_groups(&CONFIG.components.path).unwrap_or_default()
                            .into_iter()
                            .map(|mut group| {
                                group.versions = group.versions.into_iter().take(12).collect();

                                let mut group: ComponentsListGroup = group.into();

                                if group.versions.len() > 6 {
                                    for i in 6..group.versions.len() {
                                        group.versions[i].recommended = false;
                                    }
                                }

                                group
                            })
                            .collect()
                    },
                    on_downloaded: Some(GeneralAppMsg::UpdateDownloadedDxvk),
                    on_deleted: Some(GeneralAppMsg::UpdateDownloadedDxvk)
                })
                .forward(sender.input_sender(), std::convert::identity),

            game_diff: None,
            patch: None,

            style: CONFIG.launcher.style,

            languages: SUPPORTED_LANGUAGES.iter().map(|lang| tr(format_lang(lang).as_str())).collect(),

            downloaded_wine_versions: vec![],
            downloaded_dxvk_versions: vec![],

            allow_dxvk_selection: match &CONFIG.game.wine.selected {
                Some(version) => match wine::Group::find_in(&CONFIG.components.path, version) {
                    Ok(Some(group)) => group.features.need_dxvk,
                    _ => true
                }

                None => true
            },

            selected_wine_version: 0,
            selected_dxvk_version: 0,

            selecting_wine_version: false,
            selecting_dxvk_version: false
        };

        for package in VoiceLocale::list() {
            model.voice_packages.guard().push_back((
                *package,
                CONFIG.game.voices.iter().any(|voice| VoiceLocale::from_str(voice) == Some(*package))
            ));
        }

        let voice_packages = model.voice_packages.widget();

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called general settings event: {:?}", msg);

        match msg {
            GeneralAppMsg::SetGameDiff(diff) => {
                self.game_diff = diff;
            }

            GeneralAppMsg::SetPatch(patch) => {
                self.patch = patch;
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::AddVoicePackage(index) => {
                if let Some(package) = self.voice_packages.get(index.current_index()) {
                    if let Ok(mut config) = config::get() {
                        if !config.game.voices.iter().any(|voice| VoiceLocale::from_str(voice) == Some(package.locale)) {
                            config.game.voices.push(package.locale.to_code().to_string());

                            config::update(config);
    
                            sender.output(PreferencesAppMsg::UpdateLauncherState);
                        }
                    }
                }
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::RemoveVoicePackage(index) => {
                if let Some(package) = self.voice_packages.guard().get_mut(index.current_index()) {
                    if let Ok(mut config) = config::get() {
                        package.sensitive = false;

                        config.game.voices.retain(|voice| VoiceLocale::from_str(voice) != Some(package.locale));

                        config::update(config.clone());

                        let package = VoicePackage::with_locale(package.locale).unwrap();

                        if package.is_installed_in(&config.game.path) {
                            std::thread::spawn(move || {
                                if let Err(err) = package.delete_in(&config.game.path) {
                                    tracing::error!("Failed to delete voice package: {:?}", package.locale());

                                    sender.input(GeneralAppMsg::Toast {
                                        title: tr("voice-package-deletion-error"),
                                        description: Some(err.to_string())
                                    });
                                }

                                sender.input(GeneralAppMsg::SetVoicePackageSensitivity(index, true));
                                sender.output(PreferencesAppMsg::UpdateLauncherState);
                            });
                        }

                        else {
                            sender.input(GeneralAppMsg::SetVoicePackageSensitivity(index, true));
                        }
                    }
                }
            }

            GeneralAppMsg::SetVoicePackageSensitivity(index, sensitive) => {
                if let Some(package) = self.voice_packages.guard().get_mut(index.current_index()) {
                    package.sensitive = sensitive;
                }
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::RepairGame => {
                sender.output(Self::Output::RepairGame);
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::UpdateLauncherStyle(style) => {
                if style == LauncherStyle::Classic && !KEEP_BACKGROUND_FILE.exists() {
                    if let Err(err) = crate::background::download_background() {
                        tracing::error!("Failed to download background picture");
    
                        sender.input(GeneralAppMsg::Toast {
                            title: tr("background-downloading-failed"),
                            description: Some(err.to_string())
                        });

                        return;
                    }
                }

                if let Ok(mut config) = config::get() {
                    config.launcher.style = style;

                    config::update(config);
                }

                self.style = style;

                sender.output(Self::Output::SetLauncherStyle(style));
            }

            GeneralAppMsg::WineRecommendedOnly(state) => {
                // todo
                self.wine_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            GeneralAppMsg::DxvkRecommendedOnly(state) => {
                // todo
                self.dxvk_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            GeneralAppMsg::UpdateDownloadedWine => {
                self.downloaded_wine_versions = wine::get_downloaded(&CONFIG.components.path, &CONFIG.game.wine.builds)
                    .unwrap_or_default()
                    .into_iter()
                    .flat_map(|group| group.versions
                        .into_iter()
                        .map(move |version| (
                            version.clone(),
                            version.features.unwrap_or_else(|| group.features.clone()))
                        )
                    )
                    .collect();

                self.selected_wine_version = if let Some(selected) = &CONFIG.game.wine.selected {
                    let mut index = 0;

                    for (i, (version, _)) in self.downloaded_wine_versions.iter().enumerate() {
                        if &version.name == selected {
                            index = i;

                            break;
                        }
                    }

                    index as u32
                }

                else {
                    0
                };
            }

            GeneralAppMsg::UpdateDownloadedDxvk => {
                self.downloaded_dxvk_versions = dxvk::get_downloaded(&CONFIG.components.path, &CONFIG.game.dxvk.builds)
                    .unwrap_or_default()
                    .into_iter()
                    .flat_map(|group| group.versions)
                    .collect();

                self.selected_dxvk_version = if let Ok(Some(selected)) = CONFIG.get_selected_dxvk() {
                    let mut index = 0;
        
                    for (i, version) in self.downloaded_dxvk_versions.iter().enumerate() {
                        if version.name == selected.name {
                            index = i;
        
                            break;
                        }
                    }
        
                    index as u32
                }

                else {
                    0
                };
            }

            GeneralAppMsg::SelectWine(index) => {
                if let Ok(mut config) = config::get() {
                    if let Some((version, features)) = self.downloaded_wine_versions.get(index) {
                        if config.game.wine.selected.as_ref() != Some(&version.title) {
                            self.selecting_wine_version = true;
                            self.allow_dxvk_selection = features.need_dxvk;

                            let wine = version.to_wine(Some(config.game.wine.builds.join(&version.name)));
                            let wine_name = version.name.to_string();
                            let true_prefix = version.prefix_path(&config.components.path, config.game.wine.prefix.clone());

                            std::thread::spawn(move || {
                                match wine.update_prefix(true_prefix) {
                                    Ok(_) => {
                                        config.game.wine.selected = Some(wine_name); 

                                        config::update(config);
                                    }

                                    Err(err) => {
                                        sender.input(GeneralAppMsg::Toast {
                                            title: tr("wine-prefix-update-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }
                                }

                                sender.input(GeneralAppMsg::ResetWineSelection(index));
                            });
                        }
                    }
                }
            }

            GeneralAppMsg::ResetWineSelection(index) => {
                self.selecting_wine_version = false;
                self.selected_wine_version = index as u32;
            }

            GeneralAppMsg::SelectDxvk(index) => {
                if let Ok(config) = config::get() {
                    if let Some(version) = self.downloaded_dxvk_versions.get(index) {
                        if let Ok(selected) = config.get_selected_dxvk() {
                            if selected.is_none() || selected.unwrap().name != version.name {
                                self.selecting_dxvk_version = true;

                                let mut wine = match config.get_selected_wine() {
                                    Ok(Some(version)) => version.to_wine(Some(config.game.wine.builds.join(&version.name))),
                                    _ => Wine::default()
                                };

                                wine = wine.with_prefix(config.game.wine.prefix);

                                let dxvk_folder = config.game.dxvk.builds.join(&version.name);

                                std::thread::spawn(move || {
                                    if let Err(err) = Dxvk::install(&wine, dxvk_folder, InstallParams::default()) {
                                        sender.input(GeneralAppMsg::Toast {
                                            title: tr("dxvk-install-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }

                                    sender.input(GeneralAppMsg::ResetDxvkSelection(index));
                                });
                            }
                        }
                    }
                }
            }

            GeneralAppMsg::ResetDxvkSelection(index) => {
                self.selecting_dxvk_version = false;
                self.selected_dxvk_version = index as u32;
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::Toast { title, description } => {
                sender.output(Self::Output::Toast { title, description });
            }
        }
    }
}
