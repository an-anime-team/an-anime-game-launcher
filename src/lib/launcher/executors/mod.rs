mod create_prefix;
mod download_diff;

pub use create_prefix::*;
pub use download_diff::*;

use crate::lib::config;
use crate::lib::wine_prefix::WinePrefix;

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Wine,
    DXVK,
    Prefix
}

#[derive(Debug, Clone)]
pub struct ComponentsChain {
    pub chain: Vec<Component>
}

impl ComponentsChain {
    pub fn get() -> std::io::Result<Self> {
        let config = config::get()?;

        let wine_prefix = WinePrefix::new(&config.game.wine.prefix);



        todo!();
    }
}
