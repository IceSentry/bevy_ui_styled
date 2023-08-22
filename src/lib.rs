#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub mod colors;
pub use bevy_ui_styled_macros::styled;

#[derive(Component)]
pub struct BaseStyle {
    pub style: Style,
    pub color: Option<Color>,
}

#[derive(Component)]
pub struct HoveredStyle {
    pub style: Style,
    pub color: Option<Color>,
}

#[derive(Component)]
pub struct PressedStyle {
    pub style: Style,
    pub color: Option<Color>,
}

pub struct StyledPlugin;
impl Plugin for StyledPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_interactable_style);
    }
}

// TODO handle TextStyle
// TODO figure out how to only change style properties that are different and not default
#[allow(clippy::type_complexity)]
fn update_interactable_style(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut Style,
            &BaseStyle,
            Option<&HoveredStyle>,
            Option<&PressedStyle>,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bg_color, mut style, base, hovered, pressed) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                if let Some(pressed) = pressed {
                    if let Some(color) = pressed.color {
                        *bg_color = BackgroundColor(color);
                    }
                    *style = pressed.style.clone();
                }
            }
            Interaction::Hovered => {
                if let Some(hovered) = hovered {
                    if let Some(color) = hovered.color {
                        *bg_color = BackgroundColor(color);
                    }
                    *style = hovered.style.clone();
                }
            }
            Interaction::None => {
                if let Some(color) = base.color {
                    *bg_color = BackgroundColor(color);
                }
                *style = base.style.clone();
            }
        }
    }
}
