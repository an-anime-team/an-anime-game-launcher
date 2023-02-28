use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;
use anime_launcher_sdk::config::launcher::LauncherStyle;

use crate::i18n::tr;

use super::general::*;
use super::enhancements::*;
use super::environment::*;

pub static mut PREFERENCES_WINDOW: Option<adw::PreferencesWindow> = None;

pub struct PreferencesApp {
    general: AsyncController<GeneralApp>,
    enhancements: AsyncController<EnhancementsApp>,
    environment: AsyncController<EnvironmentApp>
}

#[derive(Debug, Clone)]
pub enum PreferencesAppMsg {
    /// Supposed to be called automatically on app's run when the latest game version
    /// was retrieved from the API
    SetGameDiff(Option<VersionDiff>),

    /// Supposed to be called automatically on app's run when the latest patch version
    /// was retrieved from remote repos
    SetPatch(Option<Patch>),

    SetLauncherStyle(LauncherStyle),

    UpdateLauncherState,

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for PreferencesApp {
    type Init = gtk::Window;
    type Input = PreferencesAppMsg;
    type Output = crate::ui::main::AppMsg;

    view! {
        preferences_window = adw::PreferencesWindow {
            set_title: Some(&tr("preferences")),
            set_default_size: (700, 560),
            set_hide_on_close: true,
            set_modal: true,

            // FIXME: doesn't work for any reason
            set_search_enabled: false,

            add = model.general.widget(),
            add = model.enhancements.widget(),
            add = model.environment.widget(),

            connect_close_request[sender] => move |_| {
                if let Err(err) = anime_launcher_sdk::config::flush() {
                    sender.input(PreferencesAppMsg::Toast {
                        title: tr("config-update-error"),
                        description: Some(err.to_string())
                    });
                }

                gtk::Inhibit::default()
            }
        }
    }

    async fn init(
        parent: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing preferences window");

        let model = Self {
            general: GeneralApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            enhancements: EnhancementsApp::builder()
                .launch(())
                .detach(),

            environment: EnvironmentApp::builder()
                .launch(())
                .detach()
        };

        let widgets = view_output!();

        widgets.preferences_window.set_transient_for(Some(&parent));

        unsafe {
            PREFERENCES_WINDOW = Some(widgets.preferences_window.clone());
        }

        #[allow(unused_must_use)] {
            model.general.sender().send(GeneralAppMsg::UpdateDownloadedWine);
            model.general.sender().send(GeneralAppMsg::UpdateDownloadedDxvk);
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called preferences window event: {:?}", msg);

        match msg {
            #[allow(unused_must_use)]
            PreferencesAppMsg::SetGameDiff(diff) => {
                self.general.sender().send(GeneralAppMsg::SetGameDiff(diff));
            }

            #[allow(unused_must_use)]
            PreferencesAppMsg::SetPatch(patch) => {
                self.general.sender().send(GeneralAppMsg::SetPatch(patch));
            }

            #[allow(unused_must_use)]
            PreferencesAppMsg::SetLauncherStyle(style) => {
                sender.output(Self::Output::SetLauncherStyle(style));
            }

            #[allow(unused_must_use)]
            PreferencesAppMsg::UpdateLauncherState => {
                sender.output(Self::Output::UpdateLauncherState {
                    perform_on_download_needed: false,
                    show_status_page: false
                });
            }

            PreferencesAppMsg::Toast { title, description } => unsafe {
                let toast = adw::Toast::new(&title);

                toast.set_timeout(5);

                if let Some(description) = description {
                    toast.set_button_label(Some(&tr("details")));

                    let dialog = adw::MessageDialog::new(PREFERENCES_WINDOW.as_ref(), Some(&title), Some(&description));

                    dialog.add_response("close", &tr("close"));
                    dialog.add_response("save", &tr("save"));

                    dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);

                    #[allow(unused_must_use)]
                    dialog.connect_response(Some("save"), |_, _| {
                        let result = std::process::Command::new("xdg-open")
                            .arg(crate::DEBUG_FILE.as_os_str())
                            .output();

                        if let Err(err) = result {
                            tracing::error!("Failed to open debug file: {}", err);
                        }
                    });

                    toast.connect_button_clicked(move |_| {
                        dialog.show();
                    });
                }

                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().add_toast(&toast);
            }
        }
    }
}
