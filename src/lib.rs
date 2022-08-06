use anyhow::Context;
use bevy::prelude::*;

pub mod colors;

pub fn styled(style: &str) -> Style {
    get_styled(style).expect("Failed to parse styled string")
}

pub fn get_styled(style: &str) -> anyhow::Result<Style> {
    let styles: Vec<&str> = style.split(' ').collect();
    let mut out = Style::default();
    for style in styles {
        match style {
            style if style.starts_with("flex-") => {
                match style {
                    // direction
                    "flex-col" => out.flex_direction = FlexDirection::Column,
                    "flex-col-reverse" => out.flex_direction = FlexDirection::ColumnReverse,
                    "flex-row" => out.flex_direction = FlexDirection::Row,
                    "flex-row-reverse" => out.flex_direction = FlexDirection::RowReverse,
                    // wrap
                    "flex-wrap" => out.flex_wrap = FlexWrap::Wrap,
                    "flex-wrap-reverse" => out.flex_wrap = FlexWrap::WrapReverse,
                    "flex-nowrap" => out.flex_wrap = FlexWrap::NoWrap,
                    _ => unimplemented!("{style}"),
                }
            }
            style if style.starts_with("justify-") => {
                out.justify_content = match style {
                    "justify-start" => JustifyContent::FlexStart,
                    "justify-end" => JustifyContent::FlexEnd,
                    "justify-center" => JustifyContent::Center,
                    "justify-between" => JustifyContent::SpaceBetween,
                    "justify-around" => JustifyContent::SpaceAround,
                    "justify-evenly" => JustifyContent::SpaceEvenly,
                    _ => unimplemented!("{style}"),
                }
            }
            style if style.starts_with("items-") => {
                out.align_items = match style {
                    "items-start" => AlignItems::FlexStart,
                    "items-end" => AlignItems::FlexEnd,
                    "items-center" => AlignItems::Center,
                    "items-baseline" => AlignItems::Baseline,
                    "items-stretch" => AlignItems::Stretch,
                    _ => unimplemented!("{style}"),
                }
            }
            style if style.starts_with("overflow-") => {
                out.overflow = match style {
                    "overflow-hidden" => Overflow::Hidden,
                    "overflow-visible" => Overflow::Visible,
                    _ => unimplemented!("{style}"),
                }
            }
            //margin
            style if style.starts_with("mx-") => {
                let val = get_val("mx-", style)?;
                out.margin.left = val;
                out.margin.right = val;
            }
            style if style.starts_with("my-") => {
                let val = get_val("my-", style)?;
                out.margin.top = val;
                out.margin.bottom = val;
            }
            style if style.starts_with("mt-") => {
                out.margin.top = get_val("mt-", style)?;
            }
            style if style.starts_with("mr-") => {
                out.margin.right = get_val("mr-", style)?;
            }
            style if style.starts_with("mb-") => {
                out.margin.bottom = get_val("mb-", style)?;
            }
            style if style.starts_with("ml-") => {
                out.margin.left = get_val("ml-", style)?;
            }
            style if style.starts_with("m-") => {
                out.margin = UiRect::all(get_val("m-", style)?);
            }
            // padding
            style if style.starts_with("px-") => {
                let val = get_val("px-", style)?;
                out.padding.left = val;
                out.padding.right = val;
            }
            style if style.starts_with("py-") => {
                let val = get_val("py-", style)?;
                out.padding.top = val;
                out.padding.bottom = val;
            }
            style if style.starts_with("pt-") => {
                out.padding.top = get_val("pt-", style)?;
            }
            style if style.starts_with("pr-") => {
                out.padding.right = get_val("pr-", style)?;
            }
            style if style.starts_with("pb-") => {
                out.padding.bottom = get_val("pb-", style)?;
            }
            style if style.starts_with("pl-") => {
                out.padding.left = get_val("pl-", style)?;
            }
            style if style.starts_with("p-") => {
                out.padding = UiRect::all(get_val("p-", style)?);
            }
            // width
            "w-auto" => {
                out.size.width = Val::Auto;
            }
            "w-full" => {
                out.size.width = Val::Percent(100.0);
            }
            style if style.starts_with("w-") => {
                out.size.width = get_val("w-", style)?;
            }
            // max-width
            "max-w-auto" => {
                out.max_size.width = Val::Auto;
            }
            "max-w-full" => {
                out.max_size.width = Val::Percent(100.0);
            }
            style if style.starts_with("max-w-") => {
                out.max_size.width = get_val("max-w-", style)?;
            }
            // height
            "h-auto" => {
                out.size.height = Val::Auto;
            }
            "h-full" => {
                out.size.height = Val::Percent(100.0);
            }
            style if style.starts_with("h-") => {
                out.size.height = get_val("h-", style)?;
            }
            // max-height
            "max-h-auto" => {
                out.max_size.height = Val::Auto;
            }
            "max-h-full" => {
                out.max_size.height = Val::Percent(100.0);
            }
            style if style.starts_with("max-h-") => {
                out.max_size.height = get_val("max-h-", style)?;
            }
            // display
            style if style.starts_with("hidden") => {
                out.display = Display::None;
            }
            "flex" => {
                out.display = Display::Flex;
            }
            // Position type
            "absolute" => {
                out.position_type = PositionType::Absolute;
            }
            "relative" => {
                out.position_type = PositionType::Relative;
            }
            // shrink
            "shrink" => {
                out.flex_shrink = 1.0;
            }
            "shrink-0" => {
                out.flex_shrink = 0.0;
            }
            // position
            style if style.starts_with("top-") => {
                out.position.top = get_val("top-", style)?;
            }
            style if style.starts_with("right-") => {
                out.position.right = get_val("right-", style)?;
            }
            style if style.starts_with("bottom-") => {
                out.position.bottom = get_val("bottom-", style)?;
            }
            style if style.starts_with("left-") => {
                out.position.left = get_val("left-", style)?;
            }
            style if style.starts_with("inset-y-") => {
                let val = get_val("inset-y-", style)?;
                out.position.left = val;
                out.position.right = val;
            }
            style if style.starts_with("inset-x-") => {
                let val = get_val("inset-x-", style)?;
                out.position.left = val;
                out.position.right = val;
            }
            style if style.starts_with("inset-") => {
                out.position = UiRect::all(get_val("inset-", style)?);
            }
            _ => unimplemented!("{style}"),
        }
    }
    Ok(out)
}

fn get_val(replace: &str, style: &str) -> anyhow::Result<Val> {
    Ok(if style.ends_with("auto") {
        Val::Auto
    } else if style.contains('/') {
        Val::Percent(parse_frac(replace, style)?)
    } else {
        Val::Px(parse_px(replace, style)?)
    })
}

fn parse_px(replace: &str, style: &str) -> anyhow::Result<f32> {
    style
        .replace(replace, "")
        .parse::<f32>()
        .context(format!("Failed to parse px value: {style}"))
}

fn parse_frac(replace: &str, style: &str) -> anyhow::Result<f32> {
    let style = style.replace(replace, "");
    let vals: Vec<&str> = style.split('/').collect();
    let v0 = vals[0]
        .parse::<f32>()
        .context(format!("Failed to parse percent value left: {style}"))?;
    let v1 = vals[1]
        .parse::<f32>()
        .context(format!("Failed to parse percent value right: {style}"))?;

    Ok(((v0 / v1) * 100.0).clamp(0.0, 100.0))
}

#[cfg(test)]
mod tests {
    use bevy::ui::{FlexDirection, JustifyContent, Style, UiRect, Val};

    use super::{get_val, parse_frac, parse_px, styled};

    #[test]
    fn test_parse_px() {
        assert_eq!(parse_px("", "0").unwrap(), 0.0);
        assert_eq!(parse_px("", "50").unwrap(), 50.0);
        assert_eq!(parse_px("", "50.0").unwrap(), 50.0);
    }

    #[test]
    fn test_parse_percent() {
        assert_eq!(parse_frac("", "1/2").unwrap(), 50.0);
        assert_eq!(parse_frac("", "1.0/2.0").unwrap(), 50.0);
        assert_eq!(parse_frac("", "1/1").unwrap(), 100.0);
        assert_eq!(parse_frac("", "2/1").unwrap(), 100.0);
    }

    #[test]
    fn test_get_val() {
        assert_eq!(get_val("", "1/2").unwrap(), Val::Percent(50.0));
        assert_eq!(get_val("", "50").unwrap(), Val::Px(50.0));
        assert_eq!(get_val("", "auto").unwrap(), Val::Auto);
        assert!(get_val("", "hello").is_err());
    }

    #[test]
    fn test_style() {
        assert_eq!(
            styled("flex-col-reverse justify-center"),
            Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            }
        );

        assert_eq!(
            styled("flex-col-reverse shrink-0 ml-15 mt-5 mb-5"),
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
        )
    }

    #[test]
    fn test_margin() {
        assert_eq!(
            styled("m-0"),
            Style {
                margin: UiRect::all(Val::Px(0.0)),
                ..Default::default()
            }
        );

        assert_eq!(
            styled("mx-0"),
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
            styled("my-0.0"),
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
            styled("mx-1 my-2"),
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
}
