use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::config;

use std::path::PathBuf;

use crate::*;
use crate::i18n::*;
use super::main::*;

pub struct DefaultPathsApp {
    show_additional: bool,

    launcher: PathBuf,
    runners: PathBuf,
    dxvks: PathBuf,
    prefix: PathBuf,
    game: PathBuf,
    fps_unlocker: PathBuf,
    components: PathBuf,
    patch: PathBuf,
    temp: PathBuf
}

#[derive(Debug, Clone)]
pub enum Folders {
    Launcher,
    Runners,
    DXVK,
    Prefix,
    Game,
    FpsUnlocker,
    Components,
    Patch,
    Temp
}

#[derive(Debug, Clone)]
pub enum DefaultPathsAppMsg {
    ToggleShowAdditional,
    ChoosePath(Folders),
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DefaultPathsApp {
    type Init = ();
    type Input = DefaultPathsAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr("choose-default-paths"),
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::End,
                set_vexpand: true,

                adw::ActionRow {
                    set_title: &tr("launcher-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.launcher.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Launcher)
                },
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Start,
                set_vexpand: true,

                adw::ActionRow {
                    set_title: &tr("show-all-folders"),
                    set_subtitle: &tr("show-all-folders-subtitle"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        connect_state_notify => DefaultPathsAppMsg::ToggleShowAdditional
                    }
                },
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: model.show_additional,

                adw::ActionRow {
                    set_title: &tr("runners-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.runners.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Runners)
                },

                adw::ActionRow {
                    set_title: &tr("dxvks-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.dxvks.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::DXVK)
                },

                adw::ActionRow {
                    set_title: &tr("wine-prefix-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.prefix.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Prefix)
                },

                adw::ActionRow {
                    set_title: &tr("game-installation-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.game.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Game)
                },

                adw::ActionRow {
                    set_title: &tr("fps-unlocker-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.components.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::FpsUnlocker)
                },

                adw::ActionRow {
                    set_title: &tr("components-index"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.components.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Components)
                },

                adw::ActionRow {
                    set_title: &tr("patch-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.patch.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Patch)
                },

                adw::ActionRow {
                    set_title: &tr("temp-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.temp.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Temp)
                },
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,
    
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,
    
                    gtk::Button {
                        set_label: &tr("continue"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DefaultPathsAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: &tr("exit"),
                        add_css_class: "pill",

                        connect_clicked => DefaultPathsAppMsg::Exit
                    }
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            show_additional: false,

            launcher: LAUNCHER_FOLDER.to_path_buf(),
            runners: CONFIG.game.wine.builds.clone(),
            dxvks: CONFIG.game.dxvk.builds.clone(),
            prefix: CONFIG.game.wine.prefix.clone(),
            game: CONFIG.game.path.clone(),
            fps_unlocker: CONFIG.game.enhancements.fps_unlocker.path.clone(),
            components: CONFIG.components.path.clone(),
            patch: CONFIG.patch.path.clone(),

            #[allow(clippy::or_fun_call)]
            temp: CONFIG.launcher.temp.clone().unwrap_or(PathBuf::from("/tmp"))
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            DefaultPathsAppMsg::ToggleShowAdditional => self.show_additional = !self.show_additional,

            DefaultPathsAppMsg::ChoosePath(folder) => {
                let result = rfd::AsyncFileDialog::new()
                    .set_directory(&self.launcher)
                    .pick_folder().await;

                if let Some(result) = result {
                    let result = result.path().to_path_buf();

                    match folder {
                        Folders::Launcher => {
                            self.runners      = result.join("runners");
                            self.dxvks        = result.join("dxvks");
                            self.prefix       = result.join("game");
                            self.game         = result.join("game/drive_c/Program Files/Genshin Impact");
                            self.fps_unlocker = result.join("fps-unlocker");
                            self.components   = result.join("components");
                            self.patch        = result.join("patch");
                            self.temp         = result.join("temp");

                            self.launcher = result;
                        }

                        Folders::Runners     => self.runners      = result,
                        Folders::DXVK        => self.dxvks        = result,
                        Folders::Prefix      => self.prefix       = result,
                        Folders::Game        => self.game         = result,
                        Folders::FpsUnlocker => self.fps_unlocker = result,
                        Folders::Components  => self.components   = result,
                        Folders::Patch       => self.patch        = result,
                        Folders::Temp        => self.temp         = result
                    }
                }
            }

            #[allow(unused_must_use)]
            DefaultPathsAppMsg::Continue => {
                match self.update_config() {
                    Ok(_) => sender.output(Self::Output::ScrollToSelectVoiceovers),
    
                    Err(err) => sender.output(Self::Output::Toast {
                        title: tr("config-update-error"),
                        description: Some(err.to_string())
                    })
                };
            }

            DefaultPathsAppMsg::Exit => relm4::main_application().quit()
        }
    }
}

impl DefaultPathsApp {
    pub fn update_config(&self) -> anyhow::Result<()> {
        let mut config = config::get()?;

        config.game.wine.builds = self.runners.clone();
        config.game.dxvk.builds = self.dxvks.clone();
        config.game.wine.prefix = self.prefix.clone();
        config.game.path        = self.game.clone();
        config.components.path  = self.components.clone();
        config.patch.path       = self.patch.clone();
        config.launcher.temp    = Some(self.temp.clone());

        config.game.enhancements.fps_unlocker.path = self.fps_unlocker.clone();

        config::update_raw(config)
    }
}
