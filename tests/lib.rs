use bevy::prelude::*;
use bevy_ui_styled::styled;
use bevy_ui_styled_macros::styled_bundle;

#[test]
fn test_style() {
    assert_eq!(
        styled!("flex-col-reverse justify-center"),
        Style {
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("flex-col-reverse shrink-0 ml-15 mt-5 mb-5"),
        Style {
            flex_direction: FlexDirection::ColumnReverse,
            flex_shrink: 0.,
            margin: UiRect {
                left: Val::Px(15.),
                top: Val::Px(5.),
                bottom: Val::Px(5.),
                ..Default::default()
            },
            ..Default::default()
        }
    );
}

#[test]
fn test_empty_default() {
    assert_eq!(
        styled!(""),
        Style {
            ..Default::default()
        }
    );
}

#[test]
fn test_display() {
    assert_eq!(
        styled!("hidden"),
        Style {
            display: Display::None,
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("flex"),
        Style {
            display: Display::Flex,
            ..Default::default()
        }
    );
}

#[test]
fn test_margin() {
    assert_eq!(
        styled!("m-0"),
        Style {
            margin: UiRect::all(Val::Px(0.0)),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("mx-0"),
        Style {
            margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("my-0.0"),
        Style {
            margin: UiRect {
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    );
    assert_eq!(
        styled!("mx-1 my-2"),
        Style {
            margin: UiRect {
                top: Val::Px(2.0),
                bottom: Val::Px(2.0),
                left: Val::Px(1.0),
                right: Val::Px(1.0)
            },
            ..Default::default()
        }
    );
}

#[test]
fn test_aspect_ratio() {
    assert_eq!(
        styled!("aspect-auto"),
        Style {
            aspect_ratio: None,
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("aspect-square"),
        Style {
            aspect_ratio: Some(1.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("aspect-video"),
        Style {
            aspect_ratio: Some(16.0 / 9.0),
            ..Default::default()
        }
    );
}

#[test]
fn test_macro_bundle() {
    let (style, base_style, hovered_style, pressed_style) = styled_bundle!("flex");
    assert_eq!(
        style,
        Style {
            display: Display::Flex,
            ..Default::default()
        },
    );

    assert_eq!(style, base_style.style);
    assert_eq!(base_style.color, None);

    assert_eq!(style, hovered_style.style);
    assert_eq!(hovered_style.color, None);

    assert_eq!(style, pressed_style.style);
    assert_eq!(pressed_style.color, None);

    let (style, bg_color, base_style, hovered_style, pressed_style) =
        styled_bundle!("bg-red hover:bg-blue");

    assert_eq!(style, base_style.style);
    assert_eq!(base_style.color, Some(Color::RED));
    assert_eq!(bg_color.0, Color::RED);

    assert_eq!(style, hovered_style.style);
    assert_eq!(hovered_style.color, Some(Color::BLUE));

    assert_eq!(style, pressed_style.style);
    assert_eq!(pressed_style.color, Some(Color::RED));
}

#[test]
fn test_flexbox() {
    assert_eq!(
        styled!("basis-0"),
        Style {
            flex_basis: Val::Px(0.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("basis-500"),
        Style {
            flex_basis: Val::Px(500.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("gap-x-0"),
        Style {
            column_gap: Val::Px(0.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("gap-x-10"),
        Style {
            column_gap: Val::Px(10.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("gap-y-0"),
        Style {
            row_gap: Val::Px(0.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("gap-y-55"),
        Style {
            row_gap: Val::Px(55.0),
            ..Default::default()
        }
    );

    assert_eq!(
        styled!("gap-55"),
        Style {
            column_gap: Val::Px(55.0),
            row_gap: Val::Px(55.0),
            ..Default::default()
        }
    );
}
