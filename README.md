# bevy_ui_styled

Utility function that let's you define a bevy_ui `Style` component with `tailwindcss` inspired syntax.

If you are already familiar with tailwind classes just use them and it will _probably_ work. As long as you only use the layout related classes. Not all features are supported, for example, bevy currently only supports flexbox. If you don't know tailwind but know bevy I'd recommend using the search in the tailwind docs which will give you a class that will _probably_ work. It's not actually tailwind, just based on the same principles so plenty of things might not behave as expected.

Reference: <https://tailwindcss.com>

The basic idea is that each `Style` property has a simple short-hand value that can be used to compose more complex styles. The parameter is simply a space separated string of those short-hand.

## Example

This is the button example in bevy 0.8

```rust
use bevy::prelude::*;

fn system(mut commands: Commands, asset_server: AssetServer) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
```

The same example using bevy_ui_styled

```rust
use bevy::prelude::*;
use bevy_ui_styled::styled;

fn system(mut commands: Commands, asset_server: AssetServer) {
    commands
        .spawn(ButtonBundle {
            // This will return a Style component that is identical to the one above
            style: styled!("w-150 h-65 m-auto justify-center items-center"),
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
```

## Px, Percent, Auto

Some of those utilities support passing a numerical value. Numbers are parse as `f32` so you can pass it any valid `f32`. If you use a fraction, it will compute the value as a percentage and clamp it to 100%.

```rust
use bevy_ui_styled::styled;

styled!("m-50"); // a 50px margin
styled!("m-1.5"); // a 1.5px margin
styled!("m-1/2"); // a 1/2 margin. Any fraction will be converted to a percentage and clamped to 100%
styled!("m-50%"); // a 50% margin
styled!("m-auto"); // a Val::Auto margin
```

**Warning**: In tailwind, decimal values are used to represent `em` values. Since bevy only supports percent and pixels I simply evaluate it as a pixel value. I don't know how bevy interprets a 0.5 pixel.

## Colors

`bevy_ui_styled` has a `colors` module that contains the default colors from tailwind. Unlike tailwind these aren't easily customizable, but you can use `const CUSTOM_COLOR: Color` if you want custom colors. The goal of this module is to have some basic colors to get you started.

## Bevy Version Support

| bevy_ui_styled | bevy |
| -------------- | ---- |
| 0.4            | 0.11 |
| 0.3            | 0.10 |
| 0.2            | 0.9  |
