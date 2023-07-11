use std::fmt::Debug;

use bevy::{
    prelude::Color,
    ui::{Style, UiRect, Val},
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
    let justify_items = quote_enum(style.justify_items);
    let justify_self = quote_enum(style.justify_self);
    let top = quote_val(style.top);
    let right = quote_val(style.right);
    let bottom = quote_val(style.bottom);
    let left = quote_val(style.left);
    let margin = quote_ui_rect(style.margin);
    let padding = quote_ui_rect(style.padding);
    let border = quote_ui_rect(style.border);
    let flex_grow = quote_f32(style.flex_grow);
    let flex_shrink = quote_f32(style.flex_shrink);
    let flex_basis = quote_val(style.flex_basis);
    let width = quote_val(style.width);
    let min_width = quote_val(style.min_width);
    let max_width = quote_val(style.max_width);
    let height = quote_val(style.height);
    let min_height = quote_val(style.min_height);
    let max_height = quote_val(style.max_height);
    let aspect_ratio = quote_option(style.aspect_ratio);
    // let overflow = quote_enum(style.overflow);
    let column_gap = quote_val(style.column_gap);
    let row_gap = quote_val(style.row_gap);
    // let grid_auto_columns = quote_vec(style.grid_auto_columns);
    // let grid_auto_rows = quote_vec(style.grid_auto_rows);
    // let grid_auto_flow = quote_enum(style.grid_auto_flow);
    // let grid_column = quote_enum(style.grid_column);
    // let grid_row = quote_enum(style.grid_row);
    // let grid_template_columns = quote_vec(style.grid_template_columns);
    // let grid_template_rows = quote_vec(style.grid_template_rows);

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
            justify_items: #justify_items,
            justify_self: #justify_self,
            top: #top,
            right: #right,
            bottom: #bottom,
            left: #left,
            margin: #margin,
            padding: #padding,
            border: #border,
            flex_grow: #flex_grow,
            flex_shrink: #flex_shrink,
            flex_basis: #flex_basis,
            width: #width,
            min_width: #min_width,
            max_width: #max_width,
            height: #height,
            min_height: #min_height,
            max_height: #max_height,
            aspect_ratio: #aspect_ratio,
            // overflow: #overflow,
            column_gap: #column_gap,
            row_gap: #row_gap,
            // grid_auto_columns: #grid_auto_columns,
            // grid_auto_rows: #grid_auto_rows,
            // grid_auto_flow: #grid_auto_flow,
            // grid_column: #grid_column,
            // grid_row: #grid_row,
            // grid_column: bevy::ui::GridPlacement::default(),
            // grid_row: bevy::ui::GridPlacement::default(),
            // grid_template_columns: #grid_template_columns,
            // grid_template_rows: #grid_template_rows,
            ..Default::default()
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

// fn quote_vec(_val: Vec<GridTrack | RepeatedGridTrack>) -> TokenStream {
//     quote!(Vec::<bevy::ui::GridTrack>::new())
// }
