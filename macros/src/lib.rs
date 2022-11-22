mod quote_utils;

use anyhow::Context;
use bevy::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use quote_utils::{quote_enum, quote_f32, quote_option, quote_size, quote_ui_rect, quote_val};
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

#[proc_macro]
pub fn styled_bundle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { style } = parse_macro_input!(input);

    let style = get_styled(&style.value()).expect("Failed to parse style string");
    let style = get_style_quote(style);

    // TODO return a bundle with hover/pressed/focus/bg-color components, maybe even text stuff

    TokenStream::from(style)
}

/// Reads the given style string and creates a new Style struct corresponding to the string.
#[proc_macro]
pub fn styled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { style } = parse_macro_input!(input);

    let style = get_styled(&style.value()).expect("Failed to parse style string");
    let style = get_style_quote(style);

    TokenStream::from(style)
}

fn get_style_quote(style: Style) -> proc_macro2::TokenStream {
    let display = quote_enum!(style.display);
    let position_type = quote_enum!(style.position_type);
    let direction = quote_enum!(style.direction);
    let flex_direction = quote_enum!(style.flex_direction);
    let flex_wrap = quote_enum!(style.flex_wrap);
    let align_items = quote_enum!(style.align_items);
    let align_self = quote_enum!(style.align_self);
    let align_content = quote_enum!(style.align_content);
    let justify_content = quote_enum!(style.justify_content);
    let position = quote_ui_rect!(style.position);
    let margin = quote_ui_rect!(style.margin);
    let padding = quote_ui_rect!(style.padding);
    let border = quote_ui_rect!(style.border);
    let flex_grow = quote_f32!(style.flex_grow);
    let flex_shrink = quote_f32!(style.flex_shrink);
    let flex_basis = quote_val!(style.flex_basis);
    let size = quote_size!(style.size);
    let min_size = quote_size!(style.min_size);
    let max_size = quote_size!(style.max_size);
    let aspect_ratio = quote_option!(style.aspect_ratio.map(|x| quote_f32!(x)));
    let overflow = quote_enum!(style.overflow);

    quote! {
        bevy::ui::Style {
            display: #display,
            position_type: #position_type,
            direction: #direction,
            flex_direction: #flex_direction,
            flex_wrap: #flex_wrap,
            align_items: #align_items,
            align_self: #align_self,
            align_content: #align_content,
            justify_content: #justify_content,
            position: #position,
            margin: #margin,
            padding: #padding,
            border: #border,
            flex_grow: #flex_grow,
            flex_shrink: #flex_shrink,
            flex_basis: #flex_basis,
            size: #size,
            min_size: #min_size,
            max_size: #max_size,
            aspect_ratio: #aspect_ratio,
            overflow: #overflow,
        }
    }
}

fn get_styled(style: &str) -> anyhow::Result<Style> {
    let mut out = Style::default();
    if style.is_empty() {
        return Ok(out);
    }
    let styles: Vec<&str> = style.split_whitespace().collect();
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

/// Get the value from the string
/// Support auto, raw pixel values and fractional values using `/`
fn get_val(prefix: &str, style: &str) -> anyhow::Result<Val> {
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

#[cfg(test)]
mod tests {
    use super::{get_val, parse_frac, parse_pct, parse_px};
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
        assert_eq!(get_val("", "1/2").unwrap(), Val::Percent(50.0));
        assert_eq!(get_val("", "50").unwrap(), Val::Px(50.0));
        assert_eq!(get_val("", "auto").unwrap(), Val::Auto);
        assert!(get_val("", "hello").is_err());
    }
}
