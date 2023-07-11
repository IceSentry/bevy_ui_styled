mod quote_utils;

use anyhow::Context;
use bevy::prelude::*;
use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use quote_utils::{quote_color_rgba, quote_option, quote_style};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, LitStr,
};

// TODO
// StyledPlugin
// system to handle changing color based on hover or focus
// handle fonts automatically (FontRef)
// support runtime values
// try to support format strings instead of just raw strings
// bg-color

struct Input {
    style: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let style: LitStr = input.parse()?;
        Ok(Input { style })
    }
}

/// Reads the given styled string and creates a bundle with the components used by the StyledPlugin
/// for things like hover/pressed states, background color and text styling
#[proc_macro]
pub fn styled_bundle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { style } = parse_macro_input!(input);

    let result = parse_styled(&style.value()).expect("Failed to parse styled string");

    let mut tokens = vec![];

    // Base
    let style = quote_style(result.base_style);
    tokens.push(style.clone());

    let bg_color = quote_option(result.base_bg_color.map(|color| {
        let color = quote_color_rgba(color);
        tokens.push(quote!(bevy::ui::BackgroundColor(#color)));
        color
    }));

    let base_style = quote!(bevy_ui_styled::BaseStyle {
        style: #style,
        color: #bg_color,
    });
    tokens.push(base_style);

    // Hovered
    let hovered_style = quote_style(result.hovered_style);
    let hovered_bg_color = quote_option(result.hovered_bg_color.map(quote_color_rgba));
    let hovered_style = quote!(bevy_ui_styled::HoveredStyle {
        style: #hovered_style,
        color: #hovered_bg_color,
    });
    tokens.push(hovered_style);

    // Pressed
    let pressed_style = quote_style(result.pressed_style);
    let pressed_bg_color = quote_option(result.pressed_bg_color.map(quote_color_rgba));
    let pressed_style = quote!(bevy_ui_styled::PressedStyle {
        style: #pressed_style,
        color: #pressed_bg_color,
    });
    tokens.push(pressed_style);

    let mut stream = proc_macro2::TokenStream::new();
    stream.append_separated(tokens, quote!(,));
    TokenStream::from(quote!((#stream)))
}

/// Reads the given styled string and creates a new Style struct corresponding to the string.
#[proc_macro]
pub fn styled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { style } = parse_macro_input!(input);

    let result = parse_styled(&style.value()).expect("Failed to parse styled string");
    let style = quote_style(result.base_style);

    TokenStream::from(style)
}

#[derive(Default)]
struct ParseResult {
    base_style: Style,
    base_bg_color: Option<Color>,
    hovered_style: Style,
    hovered_bg_color: Option<Color>,
    pressed_style: Style,
    pressed_bg_color: Option<Color>,
}

fn parse_styled(style: &str) -> anyhow::Result<ParseResult> {
    let mut result = ParseResult::default();
    if style.is_empty() {
        return Ok(result);
    }

    let styles: Vec<&str> = style.split_whitespace().collect();
    let mut hover_styles = vec![];
    let mut pressed_styles = vec![];
    for style in styles {
        match style {
            style if style.starts_with("hover:") => {
                hover_styles.push(style.replace("hover:", ""));
            }
            style if style.starts_with("pressed:") => {
                pressed_styles.push(style.replace("pressed:", ""));
            }
            // no modifier
            _ => parse_style_element(style, &mut result.base_style, &mut result.base_bg_color)?,
        }
    }

    // For hover and pressed state, we need to make sure the style is also the base style not just default values
    result.hovered_style = result.base_style.clone();
    result.hovered_bg_color = result.base_bg_color;
    for style in hover_styles {
        parse_style_element(
            &style,
            &mut result.hovered_style,
            &mut result.hovered_bg_color,
        )?;
    }

    result.pressed_style = result.base_style.clone();
    result.pressed_bg_color = result.base_bg_color;
    for style in pressed_styles {
        parse_style_element(
            &style,
            &mut result.pressed_style,
            &mut result.pressed_bg_color,
        )?;
    }

    Ok(result)
}

fn parse_style_element(
    style: &str,
    out: &mut Style,
    bg_color: &mut Option<Color>,
) -> anyhow::Result<()> {
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
                "overflow-hidden" => Overflow::clip(),
                "overflow-visible" => Overflow::visible(),
                _ => unimplemented!("{style}"),
            }
        }
        style if style.starts_with("aspect-") => {
            out.aspect_ratio = match style {
                "aspect-auto" => None,
                "aspect-square" => Some(1.0),
                "aspect-video" => Some(16.0 / 9.0),
                _ => unimplemented!("{style}"),
            }
        }
        //margin
        style if style.starts_with("mx-") => {
            let val = parse_val("mx-", style)?;
            out.margin.left = val;
            out.margin.right = val;
        }
        style if style.starts_with("my-") => {
            let val = parse_val("my-", style)?;
            out.margin.top = val;
            out.margin.bottom = val;
        }
        style if style.starts_with("mt-") => {
            out.margin.top = parse_val("mt-", style)?;
        }
        style if style.starts_with("mr-") => {
            out.margin.right = parse_val("mr-", style)?;
        }
        style if style.starts_with("mb-") => {
            out.margin.bottom = parse_val("mb-", style)?;
        }
        style if style.starts_with("ml-") => {
            out.margin.left = parse_val("ml-", style)?;
        }
        style if style.starts_with("m-") => {
            out.margin = UiRect::all(parse_val("m-", style)?);
        }
        // padding
        style if style.starts_with("px-") => {
            let val = parse_val("px-", style)?;
            out.padding.left = val;
            out.padding.right = val;
        }
        style if style.starts_with("py-") => {
            let val = parse_val("py-", style)?;
            out.padding.top = val;
            out.padding.bottom = val;
        }
        style if style.starts_with("pt-") => {
            out.padding.top = parse_val("pt-", style)?;
        }
        style if style.starts_with("pr-") => {
            out.padding.right = parse_val("pr-", style)?;
        }
        style if style.starts_with("pb-") => {
            out.padding.bottom = parse_val("pb-", style)?;
        }
        style if style.starts_with("pl-") => {
            out.padding.left = parse_val("pl-", style)?;
        }
        style if style.starts_with("p-") => {
            out.padding = UiRect::all(parse_val("p-", style)?);
        }
        // width
        "w-auto" => {
            out.width = Val::Auto;
        }
        "w-full" => {
            out.width = Val::Percent(100.0);
        }
        style if style.starts_with("w-") => {
            out.width = parse_val("w-", style)?;
        }
        // max-width
        "max-w-auto" => {
            out.max_width = Val::Auto;
        }
        "max-w-full" => {
            out.max_width = Val::Percent(100.0);
        }
        style if style.starts_with("max-w-") => {
            out.max_width = parse_val("max-w-", style)?;
        }
        // height
        "h-auto" => {
            out.height = Val::Auto;
        }
        "h-full" => {
            out.height = Val::Percent(100.0);
        }
        style if style.starts_with("h-") => {
            out.height = parse_val("h-", style)?;
        }
        // max-height
        "max-h-auto" => {
            out.max_height = Val::Auto;
        }
        "max-h-full" => {
            out.max_height = Val::Percent(100.0);
        }
        style if style.starts_with("max-h-") => {
            out.max_height = parse_val("max-h-", style)?;
        }
        // display
        "hidden" => {
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
        // grow
        "grow" => {
            out.flex_grow = 1.0;
        }
        "grow-0" => {
            out.flex_grow = 0.0;
        }
        // shrink
        "shrink" => {
            out.flex_shrink = 1.0;
        }
        "shrink-0" => {
            out.flex_shrink = 0.0;
        }
        // basis
        style if style.starts_with("basis-") => {
            out.flex_basis = parse_val("basis-", style)?;
        }
        // position
        style if style.starts_with("top-") => {
            out.top = parse_val("top-", style)?;
        }
        style if style.starts_with("right-") => {
            out.right = parse_val("right-", style)?;
        }
        style if style.starts_with("bottom-") => {
            out.bottom = parse_val("bottom-", style)?;
        }
        style if style.starts_with("left-") => {
            out.left = parse_val("left-", style)?;
        }
        style if style.starts_with("inset-y-") => {
            let val = parse_val("inset-y-", style)?;
            out.left = val;
            out.right = val;
        }
        style if style.starts_with("inset-x-") => {
            let val = parse_val("inset-x-", style)?;
            out.left = val;
            out.right = val;
        }
        style if style.starts_with("inset-") => {
            let parsed = UiRect::all(parse_val("inset-", style)?);
            out.top = parsed.top;
            out.right = parsed.right;
            out.bottom = parsed.bottom;
            out.left = parsed.left;
        }
        style if style.starts_with("bg-") => {
            bg_color.replace(parse_color_name(&style.replace("bg-", "")));
        }
        _ => unimplemented!("{style}"),
    };
    Ok(())
}

/// Get the value from the string
/// Support auto, raw pixel values and fractional values using `/`
fn parse_val(prefix: &str, style: &str) -> anyhow::Result<Val> {
    let style = &style.replace(prefix, "");
    Ok(if style.ends_with("auto") {
        Val::Auto
    } else if style.contains('/') {
        Val::Percent(parse_frac(style)?)
    } else if style.contains('%') {
        Val::Percent(parse_pct(style)?)
    } else {
        Val::Px(parse_px(style)?)
    })
}

/// Parse raw pixel values
/// Returns the computed value in f32
fn parse_px(style: &str) -> anyhow::Result<f32> {
    style
        .parse::<f32>()
        .context(format!("Failed to parse px value: {style}"))
}

/// Parse raw pixel values
/// Returns the computed value in f32
fn parse_pct(style: &str) -> anyhow::Result<f32> {
    style
        .replace('%', "")
        .parse::<f32>()
        .context(format!("Failed to parse % value: {style}"))
}

/// Parse fractional values
/// Returns the computed value in f32
fn parse_frac(style: &str) -> anyhow::Result<f32> {
    let vals: Vec<&str> = style.split('/').collect();
    let v0 = vals[0]
        .parse::<f32>()
        .context(format!("Failed to parse percent value left: {style}"))?;
    let v1 = vals[1]
        .parse::<f32>()
        .context(format!("Failed to parse percent value right: {style}"))?;

    Ok(((v0 / v1) * 100.0).clamp(0.0, 100.0))
}

fn parse_color_name(color: &str) -> Color {
    match color {
        color if color.starts_with("transparent") => Color::NONE,
        color if color.starts_with("red") => Color::RED,
        color if color.starts_with("green") => Color::GREEN,
        color if color.starts_with("blue") => Color::BLUE,
        _ => unimplemented!("Unknown colors: {color}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_frac, parse_pct, parse_px, parse_val};
    use bevy::ui::Val;

    #[test]
    fn test_parse_px() {
        assert_eq!(parse_px("0").unwrap(), 0.0);
        assert_eq!(parse_px("50").unwrap(), 50.0);
        assert_eq!(parse_px("50.0").unwrap(), 50.0);
    }

    #[test]
    fn test_parse_pct() {
        assert_eq!(parse_pct("0%").unwrap(), 0.0);
        assert_eq!(parse_pct("50%").unwrap(), 50.0);
        assert_eq!(parse_pct("50.0%").unwrap(), 50.0);
    }

    #[test]
    fn test_parse_percent() {
        assert_eq!(parse_frac("1/2").unwrap(), 50.0);
        assert_eq!(parse_frac("1.0/2.0").unwrap(), 50.0);
        assert_eq!(parse_frac("1/1").unwrap(), 100.0);
        assert_eq!(parse_frac("2/1").unwrap(), 100.0);
    }

    #[test]
    fn test_get_val() {
        assert_eq!(parse_val("", "1/2").unwrap(), Val::Percent(50.0));
        assert_eq!(parse_val("", "50").unwrap(), Val::Px(50.0));
        assert_eq!(parse_val("", "auto").unwrap(), Val::Auto);
        assert!(parse_val("", "hello").is_err());
    }
}
