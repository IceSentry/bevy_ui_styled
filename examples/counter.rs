//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{colors::WHITE, styled, StyledPlugin};
use bevy_ui_styled_macros::styled_bundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(StyledPlugin)
        .add_startup_system(setup)
        .add_system(on_button_interact)
        .add_system(on_count_changed)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Count(i32);

#[derive(Component)]
struct Increment;

#[derive(Component)]
struct Decrement;

#[allow(clippy::type_complexity)]
fn on_button_interact(
    mut interaction_query: Query<
        (&Interaction, Option<&Increment>, Option<&Decrement>),
        (Changed<Interaction>, With<Button>),
    >,
    mut count_query: Query<&mut Count>,
) {
    for (interaction, increment, decrement) in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            let mut count = count_query.single_mut();
            if increment.is_some() {
                count.0 += 1;
            } else if decrement.is_some() {
                count.0 -= 1;
            }
        }
    }
}

fn on_count_changed(mut count_query: Query<(&mut Text, &Count), Changed<Count>>) {
    for (mut text, count) in &mut count_query {
        text.sections[0].value = format!("Count: {}", count.0);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ui tree
    commands
        .spawn(NodeBundle {
            style: styled!("w-full h-full"),
            ..default()
        })
        .with_children(|c| {
            let text_style = TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: WHITE,
            };

            c.spawn((ButtonBundle::default(), Decrement))
                .insert(styled_bundle!(
                    "w-full h-full m-auto justify-center items-center bg-green hover:bg-red clicked:bg-blue"
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("-", text_style.clone()));
                });

            c.spawn(NodeBundle {
                style: styled!("w-1/3 shrink-0 justify-center items-center"),
                ..default()
            })
            .with_children(|c| {
                c.spawn((
                    TextBundle::from_section("Count: ", text_style.clone()),
                    Count(0),
                ));
            });

            c.spawn((ButtonBundle::default(), Increment))
                .insert(styled_bundle!(
                    "w-full h-full m-auto justify-center items-center bg-green hover:bg-red clicked:bg-blue hover:h-50%"
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("+", text_style.clone()));
                });
        });
}
