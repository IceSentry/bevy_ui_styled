# bevy_ui_styled

Utility function that let's you define a bevy_ui `Style` component with tailwindcss inspired string.

Reference: <https://tailwindcss.com>

## example

```rust
// This is the button example in bevy 0.8
commands
    .spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Button",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    });

// When using styled
commands
    .spawn_bundle(ButtonBundle {
        style: styled("w-150 h-60 m-auto justify-center items-center"),
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Button",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    });
```
