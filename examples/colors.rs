use bevy::{prelude::*, winit::WinitSettings};
use bevy_ui_styled::{colors::*, styled};

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
        font_size: 16.0,
        color: WHITE,
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
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_50));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_100));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_200));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_300));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_400));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_500));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_600));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_700));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_800));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(SKY_900));
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
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_50));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_100));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_200));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_300));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_400));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_500));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_600));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_700));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_800));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(BLUE_900));
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
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_50));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_100));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_200));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_300));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_400));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_500));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_600));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_700));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_800));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(INDIGO_900));
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
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_50));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_100));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_200));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_300));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_400));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_500));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_600));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_700));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_800));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(VIOLET_900));
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
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_50));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_100));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_200));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_300));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_400));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_500));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_600));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_700));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_800));
                            c.spawn(NodeBundle::default())
                                .insert(styled!("aspect-square"))
                                .insert(BackgroundColor(ORANGE_900));
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
