#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub mod colors;
pub use bevy_ui_styled_macros::styled;

#[derive(Bundle)]
pub struct StyledBundle {
    pub style: Style,
    pub background_color: BackgroundColor,
}
