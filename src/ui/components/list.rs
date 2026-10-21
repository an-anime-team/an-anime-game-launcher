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

use relm4::prelude::*;
use adw::prelude::*;

use super::ComponentGroupMsg;

#[derive(Debug, Clone)]
pub struct ComponentsListInit<T> {
    pub pattern: super::ComponentsListPattern,
    pub on_downloaded: Option<T>,
    pub on_deleted: Option<T>
}

pub struct ComponentsList<T> {
    pub show_recommended_only: bool,
    pub init: ComponentsListInit<T>,

    pub groups: Vec<AsyncController<super::ComponentGroup>>
}

#[derive(Debug)]
pub enum ComponentsListMsg {
    ShowRecommendedOnly(bool),
    CallOnDownloaded,
    CallOnDeleted
}

#[relm4::component(async, pub)]
impl<T: std::fmt::Debug + Clone + 'static> SimpleAsyncComponent for ComponentsList<T> {
    type Init = ComponentsListInit<T>;
    type Input = ComponentsListMsg;
    type Output = T;

    view! {
        group = adw::PreferencesGroup {}
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let init_copy = init.clone();

        let model = ComponentsList {
            show_recommended_only: true,
            init: init_copy,

            groups: init.pattern.groups
                .into_iter()
                .map(|group| {
                    super::ComponentGroup::builder()
                        .launch((group, init.pattern.download_folder.clone()))
                        .forward(sender.input_sender(), std::convert::identity)
                })
                .collect()
        };

        let widgets = view_output!();

        for group in &model.groups {
            widgets.group.add(group.widget());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            ComponentsListMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for group in &self.groups {
                    group.sender().send(ComponentGroupMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }

            #[allow(unused_must_use)]
            ComponentsListMsg::CallOnDownloaded => if let Some(on_downloaded) = &self.init.on_downloaded {
                sender.output(on_downloaded.to_owned());
            }

            #[allow(unused_must_use)]
            ComponentsListMsg::CallOnDeleted => if let Some(on_deleted) = &self.init.on_deleted {
                sender.output(on_deleted.to_owned());
            }
        }
    }
}
