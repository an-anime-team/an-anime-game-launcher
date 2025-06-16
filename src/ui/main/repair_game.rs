use relm4::prelude::*;
use relm4::Sender;

use anime_launcher_sdk::anime_game_core::reqwest::blocking::Client;
use anime_launcher_sdk::anime_game_core::sophon;
use anime_launcher_sdk::anime_game_core::sophon::repairer::{
    SophonRepairer,
    Update as SophonRepairerUpdate
};

use crate::*;
use crate::ui::components::*;

use super::{App, AppMsg};

#[allow(unused_must_use)]
pub fn repair_game(
    sender: ComponentSender<App>,
    progress_bar_input: Sender<ProgressBarMsg>
) {
    let config = Config::get().unwrap();

    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr!("verifying-files"))));
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let client = Client::new();

        let game_branches_info = sophon::get_game_branches_info(client.clone(), config.launcher.edition.into()).unwrap();

        let game_branch_info = game_branches_info.get_game_latest_by_id(config.launcher.edition.game_id()).unwrap();

        let downloads = sophon::installer::get_game_download_sophon_info(
            client.clone(),
            &game_branch_info.main,
            config.launcher.edition.into()
        ).unwrap();

        let game_download_info = downloads.manifests.iter()
            .find(|sdi| sdi.matching_field == "game")
            .unwrap();

        let mut manifests = vec![game_download_info];

        let game_path = config.game.path.for_edition(config.launcher.edition);

        let game = Game::new(game_path, config.launcher.edition);

        if let Ok(voiceovers) = game.get_voice_packages() {
            for package in voiceovers {
                let download_info = downloads.manifests.iter()
                    .find(|sdi| sdi.matching_field == package.locale().to_code());

                if let Some(download_info) = download_info {
                    manifests.push(download_info);
                }
            }
        }

        let repairer = SophonRepairer::new(
            &manifests,
            client,
            config.launcher.temp.unwrap_or_else(std::env::temp_dir)
        ).unwrap();

        let updater = move |msg: SophonRepairerUpdate| {
            match msg {
                SophonRepairerUpdate::VerifyingProgress { total, checked } => {
                    tracing::trace!("Verification progress [{checked}/{total}]");

                    progress_bar_input.send(ProgressBarMsg::UpdateProgressCounter(checked, total));
                }

                SophonRepairerUpdate::RepairingProgress { total, repaired } => {
                    tracing::trace!("Repairing progress [{repaired}/{total}]");

                    progress_bar_input.send(ProgressBarMsg::UpdateProgressCounter(repaired, total));
                }

                SophonRepairerUpdate::VerifyingStarted => {
                    tracing::trace!("Verification started");
                }

                SophonRepairerUpdate::VerifyingFinished { broken } => {
                    tracing::info!("Verification finished with {broken} broken files")
                }

                SophonRepairerUpdate::RepairingStarted => {
                    tracing::trace!("Repairing started");

                    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr!("repairing-files"))));
                }

                SophonRepairerUpdate::RepairingFinished => {
                    tracing::trace!("Repair finished");
                }

                SophonRepairerUpdate::DownloadingError(err) => {
                    tracing::error!("Error during repairing: {err}")
                }

                SophonRepairerUpdate::FileHashCheckFailed(path) => {
                    tracing::error!("File hash check error for `{path:?}`")
                }
            }
        };

        repairer.check_and_repair(game_path, config.launcher.repairer.threads as usize, updater);

        let _ = std::fs::remove_dir_all(repairer.downloading_temp());

        sender.input(AppMsg::SetDownloading(false));
    });
}
