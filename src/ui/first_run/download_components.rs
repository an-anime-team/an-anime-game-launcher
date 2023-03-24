use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::components::*;
use anime_launcher_sdk::components::wine::WincompatlibWine;
use anime_launcher_sdk::anime_game_core::installer::prelude::*;
use anime_launcher_sdk::config;
use anime_launcher_sdk::wincompatlib::prelude::*;

use std::path::PathBuf;

use super::main::FirstRunAppMsg;
use crate::ui::components::*;
use crate::i18n::*;
use crate::*;

fn get_installer(uri: &str, temp: Option<&PathBuf>) -> anyhow::Result<Installer> {
    let mut installer = Installer::new(uri)?;

    if let Some(temp) = temp {
        installer.set_temp_folder(temp);
    }

    Ok(installer)
}

pub struct DownloadComponentsApp {
    progress_bar: AsyncController<ProgressBar>,

    wine_combo: adw::ComboRow,
    dxvk_combo: adw::ComboRow,

    wine_versions: Vec<wine::Version>,
    dxvk_versions: Vec<dxvk::Version>,

    selected_wine: Option<wine::Version>,
    selected_dxvk: Option<dxvk::Version>,

    /// `None` - default,
    /// `Some(false)` - processing,
    /// `Some(true)` - done
    downloading_wine: Option<bool>,
    downloading_wine_version: String,

    /// `None` - default,
    /// `Some(false)` - processing,
    /// `Some(true)` - done
    creating_prefix: Option<bool>,
    creating_prefix_path: String,

    /// `None` - default,
    /// `Some(false)` - processing,
    /// `Some(true)` - done
    downloading_dxvk: Option<bool>,
    downloading_dxvk_version: String,

    /// `None` - default,
    /// `Some(false)` - processing,
    /// `Some(true)` - done
    applying_dxvk: Option<bool>,

    downloading: bool
}

#[derive(Debug, Clone)]
pub enum DownloadComponentsAppMsg {
    UpdateVersionsLists,
    DownloadWine,
    CreatePrefix,
    DownloadDXVK,
    ApplyDXVK,
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DownloadComponentsApp {
    type Init = ();
    type Input = DownloadComponentsAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr("download-components"),
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: !model.downloading,

                #[local_ref]
                wine_combo -> adw::ComboRow {
                    set_title: &tr("wine-version"),

                    #[watch]
                    set_model: Some(&gtk::StringList::new(model.wine_versions.iter()
                        .map(|version| version.title.as_ref())
                        .collect::<Vec<&str>>()
                        .as_slice()))
                },

                #[local_ref]
                dxvk_combo -> adw::ComboRow {
                    set_title: &tr("dxvk-version"),

                    #[watch]
                    set_model: Some(&gtk::StringList::new(model.dxvk_versions.iter()
                        .map(|version| version.name.as_ref())
                        .collect::<Vec<&str>>()
                        .as_slice()))
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: !model.downloading,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: &tr("download"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DownloadComponentsAppMsg::DownloadWine
                    },

                    gtk::Button {
                        set_label: &tr("exit"),
                        add_css_class: "pill",

                        connect_clicked => DownloadComponentsAppMsg::Exit
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: model.downloading,

                adw::ActionRow {
                    set_title: &tr("download-wine"),

                    #[watch]
                    set_subtitle: &model.downloading_wine_version,

                    #[watch]
                    set_icon_name: match model.downloading_wine {
                        Some(true) => Some("emblem-ok-symbolic"),
                        Some(false) => None, // Some("process-working"),
                        None => None
                    },

                    add_prefix = &gtk::Spinner {
                        set_spinning: true,

                        #[watch]
                        set_visible: model.downloading_wine == Some(false),
                    }
                },

                adw::ActionRow {
                    set_title: &tr("create-prefix"),

                    #[watch]
                    set_subtitle: &model.creating_prefix_path,

                    #[watch]
                    set_icon_name: match model.creating_prefix {
                        Some(true) => Some("emblem-ok-symbolic"),
                        Some(false) => None, // Some("process-working"),
                        None => None
                    },

                    add_prefix = &gtk::Spinner {
                        set_spinning: true,

                        #[watch]
                        set_visible: model.creating_prefix == Some(false),
                    }
                },

                adw::ActionRow {
                    set_title: &tr("download-dxvk"),

                    #[watch]
                    set_subtitle: &model.downloading_dxvk_version,

                    #[watch]
                    set_icon_name: match model.downloading_dxvk {
                        Some(true) => Some("emblem-ok-symbolic"),
                        Some(false) => None, // Some("process-working"),
                        None => None
                    },

                    add_prefix = &gtk::Spinner {
                        set_spinning: true,

                        #[watch]
                        set_visible: model.downloading_dxvk == Some(false),
                    }
                },

                adw::ActionRow {
                    set_title: &tr("apply-dxvk"),

                    #[watch]
                    set_icon_name: match model.applying_dxvk {
                        Some(true) => Some("emblem-ok-symbolic"),
                        Some(false) => None, // Some("process-working"),
                        None => None
                    },

                    add_prefix = &gtk::Spinner {
                        set_spinning: true,

                        #[watch]
                        set_visible: model.applying_dxvk == Some(false),
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Start,
                set_vexpand: true,

                #[watch]
                set_visible: model.downloading,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 20,
                    set_margin_top: 64,

                    append = model.progress_bar.widget(),
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            progress_bar: ProgressBar::builder()
                .launch(ProgressBarInit {
                    caption: None,
                    display_progress: true,
                    display_fraction: true,
                    visible: true
                })
                .detach(),

            wine_combo: adw::ComboRow::new(),
            dxvk_combo: adw::ComboRow::new(),

            wine_versions: vec![],
            dxvk_versions: vec![],

            selected_wine: None,
            selected_dxvk: None,

            downloading_wine: None,
            downloading_wine_version: String::new(),

            creating_prefix: None,
            creating_prefix_path: String::new(),

            downloading_dxvk: None,
            downloading_dxvk_version: String::new(),

            applying_dxvk: None,

            downloading: false
        };

        model.progress_bar.widget().set_width_request(360);

        let wine_combo = &model.wine_combo;
        let dxvk_combo = &model.dxvk_combo;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            DownloadComponentsAppMsg::UpdateVersionsLists => {
                let config = config::get().unwrap_or_else(|_| CONFIG.clone());

                // 4 latest versions of 4 first available wine group
                self.wine_versions = wine::get_groups(&config.components.path).unwrap()
                    .into_iter()
                    .take(4)
                    .flat_map(|group| group.versions.into_iter().take(4))
                    .collect();

                // 4 latest versions of 4 first available dxvk group
                self.dxvk_versions = dxvk::get_groups(&config.components.path).unwrap()
                    .into_iter()
                    .take(4)
                    .flat_map(|group| group.versions.into_iter().take(4))
                    .collect();
            }

            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::DownloadWine => {
                let config = config::get().unwrap_or_else(|_| CONFIG.clone());

                self.selected_wine = Some(self.wine_versions[self.wine_combo.selected() as usize].clone());
                self.selected_dxvk = Some(self.dxvk_versions[self.dxvk_combo.selected() as usize].clone());

                self.downloading_wine_version = self.selected_wine.clone().unwrap().title;
                self.downloading_dxvk_version = self.selected_dxvk.clone().unwrap().name;
                self.creating_prefix_path     = config.game.wine.prefix.to_string_lossy().to_string();

                self.downloading = true;
                self.downloading_wine = Some(false);

                let wine = self.selected_wine.clone().unwrap();
                let progress_bar_input = self.progress_bar.sender().clone();

                // Skip wine downloading if it was already done
                if wine.is_downloaded_in(&config.game.wine.builds) {
                    tracing::info!("Wine already installed: {}", wine.name);

                    let mut config = config::get().unwrap_or_else(|_| CONFIG.clone());

                    config.game.wine.selected = Some(wine.name);

                    if let Err(err) = config::update_raw(config) {
                        tracing::error!("Failed to update config: {err}");

                        sender.output(Self::Output::Toast {
                            title: tr("config-update-error"),
                            description: Some(err.to_string())
                        });
                    }

                    sender.input(DownloadComponentsAppMsg::CreatePrefix);
                }

                // Otherwise download wine
                else {
                    std::thread::spawn(move || {
                        tracing::info!("Installing wine: {}", wine.name);

                        // Install wine
                        match get_installer(&wine.uri, config.launcher.temp.as_ref()) {
                            Ok(mut installer) => {
                                // Create wine builds folder
                                if config.game.wine.builds.exists() {
                                    std::fs::create_dir_all(&config.game.wine.builds)
                                        .expect("Failed to create wine builds directory");
                                }

                                installer.install(&config.game.wine.builds, move |update| {
                                    match &update {
                                        InstallerUpdate::DownloadingError(err) => {
                                            tracing::error!("Failed to download wine: {err}");

                                            sender.output(Self::Output::Toast {
                                                title: tr("wine-download-error"),
                                                description: Some(err.to_string())
                                            });
                                        }

                                        InstallerUpdate::UnpackingError(err) => {
                                            tracing::error!("Failed to unpack wine: {err}");

                                            sender.output(Self::Output::Toast {
                                                title: tr("wine-unpack-errror"),
                                                description: Some(err.clone())
                                            });
                                        }

                                        // Create prefix
                                        InstallerUpdate::UnpackingFinished => {
                                            let mut config = config::get().unwrap_or_else(|_| CONFIG.clone());

                                            config.game.wine.selected = Some(wine.name.clone());

                                            if let Err(err) = config::update_raw(config) {
                                                tracing::error!("Failed to update config: {err}");

                                                sender.output(Self::Output::Toast {
                                                    title: tr("config-update-error"),
                                                    description: Some(err.to_string())
                                                });
                                            }

                                            sender.input(DownloadComponentsAppMsg::CreatePrefix);
                                        },

                                        _ => ()
                                    }

                                    progress_bar_input.send(ProgressBarMsg::UpdateFromState(update));
                                });
                            }

                            Err(err) => {
                                tracing::error!("Failed to initialize wine installer: {err}");

                                sender.output(Self::Output::Toast {
                                    title: tr("wine-install-failed"),
                                    description: Some(err.to_string())
                                });
                            }
                        }
                    });
                }
            }

            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::CreatePrefix => {
                self.downloading_wine = Some(true);
                self.creating_prefix = Some(false);

                let config = config::get().unwrap_or_else(|_| CONFIG.clone());

                tracing::info!("Creating wine prefix");

                let wine = self.selected_wine.as_ref().unwrap();

                let wine = wine
                    .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
                    .with_prefix(&config.game.wine.prefix)
                    .with_loader(WineLoader::Current)
                    .with_arch(WineArch::Win64);

                std::thread::spawn(move || {
                    match wine.update_prefix::<&str>(None) {
                        // Download DXVK
                        Ok(_) => sender.input(DownloadComponentsAppMsg::DownloadDXVK),

                        Err(err) => {
                            tracing::error!("Failed to create prefix: {err}");

                            sender.output(Self::Output::Toast {
                                title: tr("wine-prefix-update-failed"),
                                description: Some(err.to_string())
                            });
                        }
                    }
                });
            }

            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::DownloadDXVK => {
                self.creating_prefix = Some(true);
                self.downloading_dxvk = Some(false);

                let config = config::get().unwrap_or_else(|_| CONFIG.clone());

                let dxvk = self.selected_dxvk.clone().unwrap();
                let progress_bar_input = self.progress_bar.sender().clone();

                if dxvk.is_downloaded_in(&config.game.dxvk.builds) {
                    tracing::info!("DXVK is already downloaded: {}", dxvk.name);

                    sender.input(DownloadComponentsAppMsg::ApplyDXVK);
                }

                else {
                    std::thread::spawn(move || {
                        // Install DXVK
                        tracing::info!("Installing DXVK: {}", dxvk.name);

                        match get_installer(&dxvk.uri, config.launcher.temp.as_ref()) {
                            Ok(mut installer) => {
                                let progress_bar_input = progress_bar_input.clone();
                                let sender = sender.clone();

                                // Create DXVK builds folder
                                if config.game.dxvk.builds.exists() {
                                    std::fs::create_dir_all(&config.game.dxvk.builds)
                                        .expect("Failed to create DXVK builds directory");
                                }

                                installer.install(&config.game.dxvk.builds, move |update| {
                                    match &update {
                                        InstallerUpdate::DownloadingError(err) => {
                                            tracing::error!("Failed to download dxvk: {err}");

                                            sender.output(Self::Output::Toast {
                                                title: tr("dxvk-download-error"),
                                                description: Some(err.to_string())
                                            });
                                        }

                                        InstallerUpdate::UnpackingError(err) => {
                                            tracing::error!("Failed to unpack dxvk: {err}");
    
                                            sender.output(Self::Output::Toast {
                                                title: tr("dxvk-unpack-error"),
                                                description: Some(err.clone())
                                            });
                                        }

                                        // Apply DXVK
                                        InstallerUpdate::UnpackingFinished => {
                                            sender.input(DownloadComponentsAppMsg::ApplyDXVK);
                                        }

                                        _ => ()
                                    }

                                    progress_bar_input.send(ProgressBarMsg::UpdateFromState(update));
                                });
                            }

                            Err(err) => {
                                tracing::error!("Failed to initialize dxvk installer: {err}");

                                sender.output(Self::Output::Toast {
                                    title: tr("dxvk-install-failed"),
                                    description: Some(err.to_string())
                                });
                            }
                        }
                    });
                }
            }

            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::ApplyDXVK => {
                self.downloading_dxvk = Some(true);
                self.applying_dxvk = Some(false);

                let config = config::get().unwrap_or_else(|_| CONFIG.clone());

                tracing::info!("Applying DXVK");

                let wine = self.selected_wine.clone().unwrap();
                let dxvk = self.selected_dxvk.clone().unwrap();

                let group = wine.find_group(&config.components.path).unwrap().unwrap();

                // Apply DXVK if we need it
                if wine.features_in(&group).unwrap_or_default().need_dxvk {
                    let wine = wine
                        .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
                        .with_loader(WineLoader::Current)
                        .with_arch(WineArch::Win64)
                        .with_prefix(config.game.wine.prefix);

                    std::thread::spawn(move || {
                        let params = InstallParams {
                            // We just created prefix so don't need to repair it
                            repair_dlls: false,

                            ..InstallParams::default()
                        };

                        let WincompatlibWine::Default(wine) = wine else {
                            sender.input(DownloadComponentsAppMsg::Continue);

                            return;
                        };

                        match wine.install_dxvk(config.game.dxvk.builds.join(&dxvk.name), params) {
                            // Go to next page
                            Ok(_) => sender.input(DownloadComponentsAppMsg::Continue),

                            Err(err) => {
                                tracing::error!("Failed to apply DXVK: {err}");

                                sender.output(Self::Output::Toast {
                                    title: tr("dxvk-apply-error"),
                                    description: Some(err.to_string())
                                });
                            }
                        }
                    });
                }

                // Skip DXVK applying if we don't need it
                else {
                    tracing::info!("Selected wine version has `need_dxvk = false` feature. Skipping DXVK applying...");

                    sender.input(DownloadComponentsAppMsg::Continue);
                }
            }

            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::Continue => {
                std::fs::remove_file(FIRST_RUN_FILE.as_path());

                sender.output(Self::Output::ScrollToFinish);
            }

            DownloadComponentsAppMsg::Exit => relm4::main_application().quit()
        }
    }
}
