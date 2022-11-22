use bevy::prelude::*;
use bevy_ui_styled::styled;

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
