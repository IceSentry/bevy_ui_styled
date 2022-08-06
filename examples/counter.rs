//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{
    colors::{SLATE_500, SLATE_600, SLATE_700, TRANSPARENT, WHITE},
    styled,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(on_button_interact)
        .add_system(on_count_changed)
        .run();
}

const BUTTON_COLOR: Color = SLATE_600;

#[derive(Component, Deref, DerefMut)]
struct Count(i32);

#[derive(Component)]
struct Increment;

#[derive(Component)]
struct Decrement;

#[allow(clippy::type_complexity)]
fn on_button_interact(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiColor,
            Option<&Increment>,
            Option<&Decrement>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut count_query: Query<&mut Count>,
) {
    for (interaction, mut color, increment, decrement) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = SLATE_500.into();

                let mut count = count_query.single_mut();
                if increment.is_some() {
                    count.0 += 1;
                } else if decrement.is_some() {
                    count.0 -= 1;
                }
            }
            Interaction::Hovered => {
                *color = SLATE_700.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
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
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: styled("w-full h-full"),
            color: TRANSPARENT.into(),
            ..default()
        })
        .with_children(|c| {
            let text_style = TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: WHITE,
            };

            let btn = styled("w-full h-full m-auto justify-center items-center");

            c.spawn_bundle(ButtonBundle {
                style: btn.clone(),
                color: BUTTON_COLOR.into(),
                ..default()
            })
            .insert(Decrement)
            .with_children(|c| {
                c.spawn_bundle(TextBundle::from_section("-", text_style.clone()));
            });

            c.spawn_bundle(NodeBundle {
                style: styled("w-1/3 shrink-0 justify-center items-center"),
                color: TRANSPARENT.into(),
                ..default()
            })
            .with_children(|c| {
                c.spawn_bundle(TextBundle::from_section("Count: ", text_style.clone()))
                    .insert(Count(0));
            });

            c.spawn_bundle(ButtonBundle {
                style: btn,
                color: BUTTON_COLOR.into(),
                ..default()
            })
            .insert(Increment)
            .with_children(|c| {
                c.spawn_bundle(TextBundle::from_section("+", text_style.clone()));
            });
        });
}
