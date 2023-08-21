use std::fmt::Debug;

use bevy::{
    prelude::Color,
    ui::{Size, Style, UiRect, Val},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) fn quote_color_rgba(color: Color) -> TokenStream {
    let [r, g, b, a] = color.as_rgba_f32();
    quote!(bevy::prelude::Color::rgba(#r, #g, #b, #a))
}

pub(crate) fn quote_style(style: Style) -> TokenStream {
    let display = quote_enum(style.display);
    let position_type = quote_enum(style.position_type);
    let direction = quote_enum(style.direction);
    let flex_direction = quote_enum(style.flex_direction);
    let flex_wrap = quote_enum(style.flex_wrap);
    let align_items = quote_enum(style.align_items);
    let align_self = quote_enum(style.align_self);
    let align_content = quote_enum(style.align_content);
    let justify_content = quote_enum(style.justify_content);
    let position = quote_ui_rect(style.position);
    let margin = quote_ui_rect(style.margin);
    let padding = quote_ui_rect(style.padding);
    let border = quote_ui_rect(style.border);
    let flex_grow = quote_f32(style.flex_grow);
    let flex_shrink = quote_f32(style.flex_shrink);
    let flex_basis = quote_val(style.flex_basis);
    let size = quote_size(style.size);
    let min_size = quote_size(style.min_size);
    let max_size = quote_size(style.max_size);
    let aspect_ratio = quote_option(style.aspect_ratio);
    let overflow = quote_enum(style.overflow);
    let gap = quote_size(style.gap);

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
            gap: #gap,
        }
    }
}

pub fn quote_option<T: ToTokens>(opt: Option<T>) -> TokenStream {
    match opt {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}

fn quote_enum<T: Debug>(value: T) -> TokenStream {
    let type_name = std::any::type_name::<T>().to_string();
    let type_name = type_name.split("::").last().unwrap();
    let type_name: proc_macro2::TokenStream = type_name.parse().unwrap();
    let value: proc_macro2::TokenStream = format!("{:?}", value).parse().unwrap();
    quote!(bevy::ui::#type_name::#value)
}

fn quote_f32(value: f32) -> TokenStream {
    quote!(#value)
}

fn quote_val(val: Val) -> TokenStream {
    let value: proc_macro2::TokenStream = format!("{:?}", val).parse().unwrap();
    quote!(bevy::ui::Val::#value)
}

fn quote_ui_rect(rect: UiRect) -> TokenStream {
    let left = quote_val(rect.left);
    let right = quote_val(rect.right);
    let top = quote_val(rect.top);
    let bottom = quote_val(rect.bottom);
    quote!(bevy::ui::UiRect {
        left: #left,
        right: #right,
        top: #top,
        bottom: #bottom,
    })
}

fn quote_size(size: Size) -> TokenStream {
    let width = quote_val(size.width);
    let height = quote_val(size.height);
    quote!(bevy::ui::Size {
        width: #width,
        height: #height,
    })
}
