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
                    // *style = compare_style(&clicked.style, &style);
                }
            }
            Interaction::Hovered => {
                if let Some(hovered) = hovered {
                    if let Some(color) = hovered.color {
                        *bg_color = BackgroundColor(color);
                    }
                    // *style = compare_style(&hovered.style, &style);
                }
            }
            Interaction::None => {
                if let Some(color) = base.color {
                    *bg_color = BackgroundColor(color);
                }
                // *style = compare_style(&base.style, &style);
            }
        }
    }
}

// macro_rules! update_param {
//     ($out: expr, $b: expr, $default: expr, $param: ident) => {{
//         if $out.$param != $b.$param {
//             if $default.$param != $b.$param {
//                 $out.$param = $b.$param;
//             }
//         }
//     }};
// }

// fn compare_style(a: &Style, b: &Style) -> Style {
//     let default = Style::default();
//     let mut out = a.clone();

//     update_param!(out, b, default, display);
//     update_param!(out, b, default, position_type);
//     update_param!(out, b, default, direction);
//     update_param!(out, b, default, flex_direction);
//     update_param!(out, b, default, flex_wrap);
//     update_param!(out, b, default, align_items);
//     update_param!(out, b, default, align_self);
//     update_param!(out, b, default, align_content);
//     update_param!(out, b, default, justify_content);
//     update_param!(out, b, default, position);
//     update_param!(out, b, default, margin);
//     update_param!(out, b, default, padding);
//     update_param!(out, b, default, border);
//     update_param!(out, b, default, flex_grow);
//     update_param!(out, b, default, flex_shrink);
//     update_param!(out, b, default, flex_basis);
//     update_param!(out, b, default, size);
//     update_param!(out, b, default, min_size);
//     update_param!(out, b, default, max_size);
//     update_param!(out, b, default, aspect_ratio);
//     update_param!(out, b, default, overflow);

//     out
// }
