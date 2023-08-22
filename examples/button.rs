use bevy::prelude::*;
use bevy_ui_styled::{styled, StyledPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ui tree
    commands
        .spawn(NodeBundle::default())
        .with_children(|commands| {
            // bevy_ui
            commands
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Button", TextStyle::default()));
                });

            // bevy_ui_styled
            commands
                .spawn(ButtonBundle {
                    style: styled!("w-150 h-65 m-auto justify-center items-center"),
                    background_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Button", TextStyle::default()));
                });
        });
}
