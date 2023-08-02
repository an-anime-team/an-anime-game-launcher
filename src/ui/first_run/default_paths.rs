use std::path::PathBuf;

use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use crate::*;
use crate::ui::components::progress_bar::*;

use super::main::*;

pub struct DefaultPathsApp {
    progress_bar: AsyncController<ProgressBar>,

    show_additional: bool,
    migrate_installation: bool,
    show_progress: bool,

    launcher: PathBuf,
    runners: PathBuf,
    dxvks: PathBuf,
    prefix: PathBuf,
    game_global: PathBuf,
    game_china: PathBuf,
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
    GameGlobal,
    GameChina,
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
    /// If `true`, then use migrate installation mode
    type Init = bool;
    type Input = DefaultPathsAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr!("choose-default-paths"),
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::End,
                set_vexpand: true,

                #[watch]
                set_sensitive: !model.show_progress,

                adw::ActionRow {
                    set_title: &tr!("launcher-folder"),
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
                    set_title: &tr!("show-all-folders"),
                    set_subtitle: &tr!("show-all-folders-subtitle"),

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

                #[watch]
                set_sensitive: !model.show_progress,

                adw::ActionRow {
                    set_title: &tr!("runners-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.runners.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Runners)
                },

                adw::ActionRow {
                    set_title: &tr!("dxvks-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.dxvks.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::DXVK)
                },

                adw::ActionRow {
                    set_title: &tr!("wine-prefix-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.prefix.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Prefix)
                },

                adw::ActionRow {
                    set_title: &tr!("global-game-installation-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.game_global.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::GameGlobal)
                },

                adw::ActionRow {
                    set_title: &tr!("chinese-game-installation-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.game_china.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::GameChina)
                },

                adw::ActionRow {
                    set_title: &tr!("fps-unlocker-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.fps_unlocker.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::FpsUnlocker)
                },

                adw::ActionRow {
                    set_title: &tr!("components-index"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.components.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Components)
                },

                adw::ActionRow {
                    set_title: &tr!("patch-folder"),
                    set_icon_name: Some("folder-symbolic"),
                    set_activatable: true,

                    #[watch]
                    set_subtitle: model.patch.to_str().unwrap(),

                    connect_activated => DefaultPathsAppMsg::ChoosePath(Folders::Patch)
                },

                adw::ActionRow {
                    set_title: &tr!("temp-folder"),
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

                #[watch]
                set_visible: !model.show_progress,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: &if model.migrate_installation {
                            tr!("migrate")
                        } else {
                            tr!("continue")
                        },

                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DefaultPathsAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: &if model.migrate_installation {
                            tr!("close", { "form" = "noun" })
                        } else {
                            tr!("exit")
                        },

                        add_css_class: "pill",

                        #[watch]
                        set_visible: !model.migrate_installation,

                        connect_clicked => DefaultPathsAppMsg::Exit
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: model.show_progress,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,

                    append = model.progress_bar.widget(),
                }
            }
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            progress_bar: ProgressBar::builder()
                .launch(ProgressBarInit {
                    caption: None,
                    display_progress: true,
                    display_fraction: false,
                    visible: false
                })
                .detach(),

            show_additional: false,
            migrate_installation: init,
            show_progress: false,

            launcher: LAUNCHER_FOLDER.to_path_buf(),
            runners: CONFIG.game.wine.builds.clone(),
            dxvks: CONFIG.game.dxvk.builds.clone(),
            prefix: CONFIG.game.wine.prefix.clone(),
            game_global: CONFIG.game.path.global.clone(),
            game_china: CONFIG.game.path.china.clone(),
            fps_unlocker: CONFIG.game.enhancements.fps_unlocker.path.clone(),
            components: CONFIG.components.path.clone(),
            patch: CONFIG.patch.path.clone(),

            #[allow(clippy::or_fun_call)]
            temp: CONFIG.launcher.temp.clone().unwrap_or(std::env::temp_dir())
        };

        // Set progress bar width
        model.progress_bar.widget().set_width_request(400);

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
                            self.prefix       = result.join("prefix");
                            self.game_global  = result.join(concat!("Ge", "nshi", "n Imp", "act"));
                            self.game_china   = result.join(concat!("Yu", "anS", "hen"));
                            self.fps_unlocker = result.join("fps-unlocker");
                            self.components   = result.join("components");
                            self.patch        = result.join("patch");
                            self.temp         = result.clone();

                            self.launcher = result;
                        }

                        Folders::Runners     => self.runners      = result,
                        Folders::DXVK        => self.dxvks        = result,
                        Folders::Prefix      => self.prefix       = result,
                        Folders::GameGlobal  => self.game_global  = result,
                        Folders::GameChina   => self.game_china   = result,
                        Folders::FpsUnlocker => self.fps_unlocker = result,
                        Folders::Components  => self.components   = result,
                        Folders::Patch       => self.patch        = result,
                        Folders::Temp        => self.temp         = result
                    }
                }
            }

            #[allow(unused_must_use)]
            DefaultPathsAppMsg::Continue => {
                let old_config = Config::get().unwrap_or_else(|_| CONFIG.clone());

                match self.update_config() {
                    Ok(_) => {
                        if self.migrate_installation {
                            self.progress_bar.sender().send(ProgressBarMsg::SetVisible(true));

                            self.show_progress = true;

                            let folders = [
                                (old_config.game.wine.builds, &self.runners),
                                (old_config.game.dxvk.builds, &self.dxvks),
                                (old_config.game.wine.prefix, &self.prefix),
                                (old_config.game.path.global, &self.game_global),
                                (old_config.game.path.china,  &self.game_china),
                                (old_config.components.path,  &self.components),
                                (old_config.patch.path,       &self.patch),

                                (old_config.game.enhancements.fps_unlocker.path, &self.fps_unlocker)
                            ];

                            #[allow(clippy::expect_fun_call)]
                            for (i, (from, to)) in folders.iter().enumerate() {
                                self.progress_bar.sender().send(ProgressBarMsg::UpdateCaption(Some(
                                    from.to_str().map(|str| str.to_string()).unwrap_or_else(|| format!("{:?}", from))
                                )));

                                if &from != to && from.exists() {
                                    move_folder::move_folder(from, to).expect(&format!("Failed to move folder: {:?} -> {:?}", from, to));
                                }

                                self.progress_bar.sender().send(ProgressBarMsg::UpdateProgress(i as u64 + 1, folders.len() as u64));
                            }

                            // Restart the app

                            std::process::Command::new(std::env::current_exe().unwrap()).spawn().unwrap();

                            relm4::main_application().quit();
                        }

                        else {
                            sender.output(Self::Output::ScrollToSelectVoiceovers);
                        }
                    }

                    Err(err) => {
                        sender.output(Self::Output::Toast {
                            title: tr!("config-update-error"),
                            description: Some(err.to_string())
                        });
                    }
                }
            }

            DefaultPathsAppMsg::Exit => {
                if self.migrate_installation {
                    // TODO: this shit should return message to general preferences component somehow to close MigrateInstallation window
                    todo!();
                }

                else {
                    relm4::main_application().quit();
                }
            }
        }
    }
}

impl DefaultPathsApp {
    pub fn update_config(&self) -> anyhow::Result<()> {
        let mut config = Config::get()?;

        config.game.wine.builds = self.runners.clone();
        config.game.dxvk.builds = self.dxvks.clone();
        config.game.wine.prefix = self.prefix.clone();
        config.game.path.global = self.game_global.clone();
        config.game.path.china  = self.game_china.clone();
        config.components.path  = self.components.clone();
        config.patch.path       = self.patch.clone();
        config.launcher.temp    = Some(self.temp.clone());

        config.game.enhancements.fps_unlocker.path = self.fps_unlocker.clone();

        Config::update_raw(config)
    }
}
