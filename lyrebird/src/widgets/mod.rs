// SPDX-License-Identifier: BSD-3-Clause

use crate::theme::Theme;

pub mod tabBar;
pub mod trackProgress;

pub type Renderer = iced::Renderer;
pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
