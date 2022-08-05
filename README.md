# bevy_ui_styled

Utility function that let's you define a bevy_ui `Style` component with tailwindcss inspired string.

If you are already familiar with tailwind classes just use them and it will probably work. As long as you only use the layout related classes. Not all features are supported, for example, bevy currently only supporst flexbox.

Reference: <https://tailwindcss.com>

## Example

This is the button example in bevy 0.8

```rust
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
```

The same example using bevy_ui_styled

```rust
commands
    .spawn_bundle(ButtonBundle {
        // This will return a Style component that is identical to the one above
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
