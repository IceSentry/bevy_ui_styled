use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{
    colors::{SLATE_500, WHITE},
    styled,
};
use bevy_ui_styled_macros::styled_bundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        font_size: 30.0,
        color: WHITE,
        ..default()
    };

    // Ui tree
    commands
        .spawn(NodeBundle::default())
        .insert(styled_bundle!(
            "w-90% h-90% m-auto p-15 justify-between flex-col bg-blue"
        ))
        .with_children(|c| {
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-96% mb-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-96%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-80% mb-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-80%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-72% my-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-72%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-64% my-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-64%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-56% my-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-56%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-52% my-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-52%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert((
                    styled!("h-full w-48% mt-4 justify-center items-center"),
                    BackgroundColor(SLATE_500),
                ))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-48%", text_style.clone()));
                });
        });
}
