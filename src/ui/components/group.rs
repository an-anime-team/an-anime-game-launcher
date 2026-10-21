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

use std::path::PathBuf;

use relm4::prelude::*;
use adw::prelude::*;

use super::ComponentsListMsg;
use super::ComponentVersionMsg;

pub struct ComponentGroup {
    pub title: String,
    pub show_recommended_only: bool,

    pub versions: Vec<AsyncController<super::ComponentVersion>>
}

#[derive(Debug)]
pub enum ComponentGroupMsg {
    ShowRecommendedOnly(bool),
    CallOnDownloaded,
    CallOnDeleted
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ComponentGroup {
    type Init = (super::ComponentsListGroup, PathBuf);
    type Input = ComponentGroupMsg;
    type Output = super::list::ComponentsListMsg;

    view! {
        group = adw::ExpanderRow {
            set_title: &model.title
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = ComponentGroup {
            title: init.0.title,
            show_recommended_only: true,

            versions: init.0.versions
                .into_iter()
                .map(|version| {
                    super::ComponentVersion::builder()
                        .launch((version, init.1.clone()))
                        .forward(sender.input_sender(), std::convert::identity)
                })
                .collect()
        };

        let widgets = view_output!();

        for version in &model.versions {
            widgets.group.add_row(version.widget());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            ComponentGroupMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for version in &self.versions {
                    version.sender().send(ComponentVersionMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }

            #[allow(unused_must_use)]
            ComponentGroupMsg::CallOnDownloaded => {
                sender.output(ComponentsListMsg::CallOnDownloaded);
            }

            #[allow(unused_must_use)]
            ComponentGroupMsg::CallOnDeleted => {
                sender.output(ComponentsListMsg::CallOnDeleted);
            }
        }
    }
}
