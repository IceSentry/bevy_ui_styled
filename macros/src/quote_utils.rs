pub(crate) fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

macro_rules! quote_enum {
    ($prop: expr) => {{
        let type_name = $crate::quote_utils::type_of(&$prop);
        let type_name = type_name.split("::").last().unwrap();
        let type_name: proc_macro2::TokenStream = type_name.parse().unwrap();
        let value: proc_macro2::TokenStream = format!("{:?}", $prop).parse().unwrap();
        quote!(bevy::ui::#type_name::#value)
    }};
}

macro_rules! quote_option {
    ($value: expr) => {{
        match $value {
            Some(value) => quote!(Some(#value)),
            None => quote!(None),
        }
    }};
}

macro_rules! quote_f32 {
    ($value: expr) => {{
        let value: proc_macro2::TokenStream = format!("{}", $value).parse().unwrap();
        quote!(#value as f32)
    }};
}

macro_rules! quote_val {
    ($value: expr) => {{
        let value: proc_macro2::TokenStream = format!("{:?}", $value).parse().unwrap();
        quote!(bevy::ui::Val::#value)
    }};
}

macro_rules! quote_ui_rect {
    ($value: expr) => {{
        let left = quote_val!($value.left);
        let right = quote_val!($value.right);
        let top = quote_val!($value.top);
        let bottom = quote_val!($value.bottom);
        quote!(bevy::ui::UiRect {
            left: #left,
            right: #right,
            top: #top,
            bottom: #bottom,
        })
    }};
}

macro_rules! quote_size {
    ($value: expr) => {{
        let width = quote_val!($value.width);
        let height = quote_val!($value.height);
        quote!(bevy::ui::Size {
            width: #width,
            height: #height,
        })
    }};
}

pub(crate) use quote_enum;
pub(crate) use quote_f32;
pub(crate) use quote_option;
pub(crate) use quote_size;
pub(crate) use quote_ui_rect;
pub(crate) use quote_val;
