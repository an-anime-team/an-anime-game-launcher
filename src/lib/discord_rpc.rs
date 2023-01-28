use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, SendError};

use discord_rich_presence::{
    activity::*,
    DiscordIpc,
    DiscordIpcClient
};

use super::config::prelude::DiscordRpc as DiscordRpcConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcUpdates {
    /// Establish RPC connection
    Connect,

    /// Terminate RPC connection. Panics if not connected
    Disconnect,

    /// Update RPC activity
    UpdateActivity {
        title: String,
        subtitle: String,
        image: String
    },

    /// Clear RPC activity
    ClearActivity
}

pub struct DiscordRpc {
    _thread: JoinHandle<()>,
    sender: Sender<RpcUpdates>
}

impl DiscordRpc {
    pub fn new(mut config: DiscordRpcConfig) -> Self {
        let (sender, receiver) = mpsc::channel();

        Self {
            _thread: std::thread::spawn(move || {
                let mut client = DiscordIpcClient::new(&config.app_id.to_string())
                    .expect("Failed to register discord ipc client");

                let mut connected = false;

                while let Ok(update) = receiver.recv() {
                    match update {
                        RpcUpdates::Connect => {
                            if !connected {
                                connected = true;

                                client.connect().expect("Failed to connect to discord");

                                client.set_activity(Self::get_activity(&config))
                                    .expect("Failed to update discord rpc activity");
                            }
                        }

                        RpcUpdates::Disconnect => {
                            if connected {
                                connected = false;

                                client.close().expect("Failed to disconnect from discord");
                            }
                        }

                        RpcUpdates::UpdateActivity { title, subtitle, image } => {
                            config.title = title;
                            config.subtitle = subtitle;
                            config.image = image;

                            if connected {
                                client.set_activity(Self::get_activity(&config))
                                    .expect("Failed to update discord rpc activity");
                            }
                        }

                        RpcUpdates::ClearActivity => {
                            if connected {
                                client.clear_activity().expect("Failed to clear discord rpc activity");
                            }
                        }
                    }
                }
            }),
            sender
        }
    }

    pub fn get_activity(config: &DiscordRpcConfig) -> Activity {
        Activity::new()
            .details(&config.title)
            .state(&config.subtitle)
            .assets(Assets::new().large_image(&config.image))
    }

    pub fn update(&self, update: RpcUpdates) -> Result<(), SendError<RpcUpdates>> {
        self.sender.send(update)
    }
}
