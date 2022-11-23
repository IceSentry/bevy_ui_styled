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
pub struct ClickedStyle {
    pub style: Style,
    pub color: Option<Color>,
}

pub struct StyledPlugin;
impl Plugin for StyledPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_interactable_style);
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
            Option<&ClickedStyle>,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bg_color, mut style, base, hovered, clicked) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                if let Some(clicked) = clicked {
                    if let Some(color) = clicked.color {
                        *bg_color = BackgroundColor(color);
                    }
                    // *style = compare_style(&style, &clicked.style);
                    *style = clicked.style.clone();
                }
            }
            Interaction::Hovered => {
                if let Some(hovered) = hovered {
                    if let Some(color) = hovered.color {
                        *bg_color = BackgroundColor(color);
                    }
                    // *style = compare_style(&style, &hovered.style);
                    *style = hovered.style.clone();
                }
            }
            Interaction::None => {
                if let Some(color) = base.color {
                    *bg_color = BackgroundColor(color);
                }
                // *style = compare_style(&style, &base.style);
                *style = base.style.clone();
            }
        }
    }
}
