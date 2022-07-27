use std::io::Error;

use wait_not_await::Await;

use crate::ui::components::progress_bar::ProgressBar;

use crate::lib::wine_prefix::WinePrefix;
use crate::lib::config::Config;

pub fn create_prefix(config: Config, progress_bar: ProgressBar) -> Await<Result<(), (String, Error)>> {
    Await::new(move || {
        // Create prefix if needed
        let prefix = WinePrefix::new(&config.game.wine.prefix);

        if !prefix.exists() {
            progress_bar.update(0.0, Some("Creating prefix..."));

            match config.try_get_selected_wine_info() {
                Some(wine_version) => match prefix.update(&config.game.wine.builds, wine_version) {
                    Ok(_) => Ok(()),
                    Err(err) => Err((String::from("Failed to create prefix"), err))
                },
                None => {
                    // TODO: download default wine
                    todo!()
                }
            }
        }

        else {
            Ok(())
        }
    })
}
