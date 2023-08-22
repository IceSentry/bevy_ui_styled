use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{colors::*, styled};

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
        font_size: 16.0,
        color: WHITE,
        ..default()
    };

    // Ui tree
    commands
        .spawn(NodeBundle::default())
        .insert(styled!("w-full h-full flex-col"))
        .insert(BackgroundColor(SLATE_900))
        .with_children(|c| {
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full m-10 flex-col p-10"))
                .insert(BackgroundColor(SLATE_700))
                .with_children(|c| {
                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-full flex-row justify-between"))
                        .with_children(|c| {
                            for color in [
                                SKY_50, SKY_100, SKY_200, SKY_200, SKY_300, SKY_400, SKY_500,
                                SKY_600, SKY_700, SKY_800, SKY_900,
                            ] {
                                c.spawn(NodeBundle::default())
                                    .insert(styled!("aspect-square h-full"))
                                    .insert(BackgroundColor(color));
                            }
                        });

                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-18 w-full flex-row items-center justify-between"))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section("sky-50", text_style.clone()));
                            c.spawn(TextBundle::from_section("sky-900", text_style.clone()));
                        });
                });
            c.spawn(NodeBundle::default())
                .insert(styled!("h-full m-10 flex-col p-10"))
                .insert(BackgroundColor(SLATE_700))
                .with_children(|c| {
                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-full flex-row justify-between"))
                        .with_children(|c| {
                            for color in [
                                BLUE_50, BLUE_100, BLUE_200, BLUE_200, BLUE_300, BLUE_400,
                                BLUE_500, BLUE_600, BLUE_700, BLUE_800, BLUE_900,
                            ] {
                                c.spawn(NodeBundle::default())
                                    .insert(styled!("aspect-square h-full"))
                                    .insert(BackgroundColor(color));
                            }
                        });

                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-18 w-full flex-row items-center justify-between"))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section("blue-50", text_style.clone()));
                            c.spawn(TextBundle::from_section("blue-900", text_style.clone()));
                        });
                });

            c.spawn(NodeBundle::default())
                .insert(styled!("h-full m-10 flex-col p-10"))
                .insert(BackgroundColor(SLATE_700))
                .with_children(|c| {
                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-full flex-row justify-between"))
                        .with_children(|c| {
                            for color in [
                                INDIGO_50, INDIGO_100, INDIGO_200, INDIGO_200, INDIGO_300,
                                INDIGO_400, INDIGO_500, INDIGO_600, INDIGO_700, INDIGO_800,
                                INDIGO_900,
                            ] {
                                c.spawn(NodeBundle::default())
                                    .insert(styled!("aspect-square h-full"))
                                    .insert(BackgroundColor(color));
                            }
                        });

                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-18 w-full flex-row items-center justify-between"))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section("indigo-50", text_style.clone()));
                            c.spawn(TextBundle::from_section("indigo-900", text_style.clone()));
                        });
                });

            c.spawn(NodeBundle::default())
                .insert(styled!("h-full m-10 flex-col p-10"))
                .insert(BackgroundColor(SLATE_700))
                .with_children(|c| {
                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-full flex-row justify-between"))
                        .with_children(|c| {
                            for color in [
                                VIOLET_50, VIOLET_100, VIOLET_200, VIOLET_200, VIOLET_300,
                                VIOLET_400, VIOLET_500, VIOLET_600, VIOLET_700, VIOLET_800,
                                VIOLET_900,
                            ] {
                                c.spawn(NodeBundle::default())
                                    .insert(styled!("aspect-square h-full"))
                                    .insert(BackgroundColor(color));
                            }
                        });

                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-18 w-full flex-row items-center justify-between"))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section("violet-50", text_style.clone()));
                            c.spawn(TextBundle::from_section("violet-900", text_style.clone()));
                        });
                });

            c.spawn(NodeBundle::default())
                .insert(styled!("h-full m-10 flex-col p-10"))
                .insert(BackgroundColor(SLATE_700))
                .with_children(|c| {
                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-full flex-row justify-between"))
                        .with_children(|c| {
                            for color in [
                                ORANGE_50, ORANGE_100, ORANGE_200, ORANGE_200, ORANGE_300,
                                ORANGE_400, ORANGE_500, ORANGE_600, ORANGE_700, ORANGE_800,
                                ORANGE_900,
                            ] {
                                c.spawn(NodeBundle::default())
                                    .insert(styled!("aspect-square h-full"))
                                    .insert(BackgroundColor(color));
                            }
                        });

                    c.spawn(NodeBundle::default())
                        .insert(styled!("h-18 w-full flex-row items-center justify-between"))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section("orange-50", text_style.clone()));
                            c.spawn(TextBundle::from_section("orange-900", text_style.clone()));
                        });
                });
        });
}
