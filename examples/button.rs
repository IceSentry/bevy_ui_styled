//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{styled, theme::TEXT_WHITE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>, With<MyButton>),
    >,
    mut text_query: Query<&mut Text>,
) {
    let (interaction, mut color, children) = interaction_query.single_mut();
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
        Interaction::Clicked => {
            text.sections[0].value = "Press".to_string();
            *color = PRESSED_BUTTON.into();
        }
        Interaction::Hovered => {
            text.sections[0].value = "Hover".to_string();
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            text.sections[0].value = "Button".to_string();
            *color = NORMAL_BUTTON.into();
        }
    }
}

#[derive(Component)]
struct MyButton;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: styled("w-150 h-60 m-auto justify-center items-center"),
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MyButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: TEXT_WHITE,
                },
            ));
        });
}
