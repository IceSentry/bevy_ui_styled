# bevy_ui_styled

Utility function that let's you define a bevy_ui `Style` component with `tailwindcss` inspired syntax.

If you are already familiar with tailwind classes just use them and it will _probably_ work. As long as you only use the layout related classes. Not all features are supported, for example, bevy currently only supports flexbox. If you don't know tailwind but know bevy I'd recommend using the search in the tailwind docs which will give you a class that will _probably_ work. It's not actually tailwind, just based on the same principles so plenty of things might not behave as expected.

Reference: <https://tailwindcss.com>

The basic idea is that each `Style` property has a simple short-hand value that can be used to compose more complex styles. The parameter is simply a space separated string of those short-hand.

## Example

This is the button example in bevy 0.8

```rust
commands
    .spawn(ButtonBundle {
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
        background_color: NORMAL_BUTTON.into(),
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
    .spawn(ButtonBundle {
        // This will return a Style component that is identical to the one above
        style: styled("w-150 h-65 m-auto justify-center items-center"),
        background_color: NORMAL_BUTTON.into(),
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

## Px, Percent, Auto

Some of those utilities support passing a numerical value. Numbers are parse as `f32` so you can pass it any valid `f32`. If you use a fraction, it will compute the value as a percentage and clamp it to 100%.

```rust
styled("m-50"); // a 50px margin
styled("m-1.5"); // a 1.5px margin
styled("m-1/2"); // a 1/2 or 50% margin. Any fraction will be converted to a percentage and clamped to 100%
styled("m-auto"); // a Val::Auto margin
```

**Warning**: In tailwind, decimal values are used to represent `em` values. Since bevy only supports percent and pixels I simply evaluate it as a pixel value. I don't know how bevy interprets a 0.5 pixel.

## Colors

I also created a `colors` module that containes the default colors from tailwind. Unlike tailwind these aren't easily customizable, but you can just use a rust `const CUSTOM_COLOR: Color` to do that. This is just to have some basic color to get you started.

## Extracting Styles

If you don't like repeating the same classes multiple time, you can easily just store the output of the function in a variable and clone it whenever you want to duplicate a style.

Since the styles are just a string parsed at runtime. If you need to share a style globally, you can simply define the string in a `const` and use that `const` as the argument.

```rust
const GLOBAL_STYLE: &str = "w-full h-full justify-center";

fn foo() {
    let bar = styled(GLOBAL_STYLE);
}
```
