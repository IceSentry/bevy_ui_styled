use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{
    colors::{SLATE_500, SLATE_900, WHITE},
    styled,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: WHITE,
    };

    // Ui tree
    commands
        .spawn(NodeBundle::default())
        .insert(styled!("w-90% h-90% m-auto p-15 justify-between flex-col"))
        .insert(BackgroundColor(SLATE_900))
        .with_children(|c| {
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-96% mb-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-96%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-80% my-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-80%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-72% my-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-72%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-64% my-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-64%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-56% my-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-56%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-52% my-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-52%", text_style.clone()));
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full w-48% mt-4 justify-center items-center"))
                .insert(BackgroundColor(SLATE_500))
                .with_children(|c| {
                    c.spawn(TextBundle::from_section("w-48%", text_style.clone()));
                });
        });
}
