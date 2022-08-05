//! Reference: <https://tailwindcss.com/docs/customizing-colors>

use bevy::prelude::Color;

pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
pub const TRANSPARENT: Color = Color::NONE;

pub const AMBER_50: Color = Color::rgb(1.00, 0.98, 0.92);
pub const AMBER_100: Color = Color::rgb(1.00, 0.95, 0.78);
pub const AMBER_200: Color = Color::rgb(0.99, 0.90, 0.54);
pub const AMBER_300: Color = Color::rgb(0.99, 0.83, 0.30);
pub const AMBER_400: Color = Color::rgb(0.98, 0.75, 0.14);
pub const AMBER_500: Color = Color::rgb(0.96, 0.62, 0.04);
pub const AMBER_600: Color = Color::rgb(0.85, 0.47, 0.02);
pub const AMBER_700: Color = Color::rgb(0.71, 0.33, 0.04);
pub const AMBER_800: Color = Color::rgb(0.57, 0.25, 0.05);
pub const AMBER_900: Color = Color::rgb(0.47, 0.21, 0.06);

pub const BLUE_50: Color = Color::rgb(0.94, 0.96, 1.00);
pub const BLUE_100: Color = Color::rgb(0.86, 0.92, 1.00);
pub const BLUE_200: Color = Color::rgb(0.75, 0.86, 1.00);
pub const BLUE_300: Color = Color::rgb(0.58, 0.77, 0.99);
pub const BLUE_400: Color = Color::rgb(0.38, 0.65, 0.98);
pub const BLUE_500: Color = Color::rgb(0.23, 0.51, 0.96);
pub const BLUE_600: Color = Color::rgb(0.15, 0.39, 0.92);
pub const BLUE_700: Color = Color::rgb(0.11, 0.31, 0.85);
pub const BLUE_800: Color = Color::rgb(0.12, 0.25, 0.69);
pub const BLUE_900: Color = Color::rgb(0.12, 0.23, 0.54);

pub const CYAN_50: Color = Color::rgb(0.93, 1.00, 1.00);
pub const CYAN_100: Color = Color::rgb(0.81, 0.98, 1.00);
pub const CYAN_200: Color = Color::rgb(0.65, 0.95, 0.99);
pub const CYAN_300: Color = Color::rgb(0.40, 0.91, 0.98);
pub const CYAN_400: Color = Color::rgb(0.13, 0.83, 0.93);
pub const CYAN_500: Color = Color::rgb(0.02, 0.71, 0.83);
pub const CYAN_600: Color = Color::rgb(0.03, 0.57, 0.70);
pub const CYAN_700: Color = Color::rgb(0.05, 0.45, 0.56);
pub const CYAN_800: Color = Color::rgb(0.08, 0.37, 0.46);
pub const CYAN_900: Color = Color::rgb(0.09, 0.31, 0.39);

pub const EMERALD_50: Color = Color::rgb(0.93, 0.99, 0.96);
pub const EMERALD_100: Color = Color::rgb(0.82, 0.98, 0.90);
pub const EMERALD_200: Color = Color::rgb(0.65, 0.95, 0.82);
pub const EMERALD_300: Color = Color::rgb(0.43, 0.91, 0.72);
pub const EMERALD_400: Color = Color::rgb(0.20, 0.83, 0.60);
pub const EMERALD_500: Color = Color::rgb(0.06, 0.73, 0.51);
pub const EMERALD_600: Color = Color::rgb(0.02, 0.59, 0.41);
pub const EMERALD_700: Color = Color::rgb(0.02, 0.47, 0.34);
pub const EMERALD_800: Color = Color::rgb(0.02, 0.37, 0.27);
pub const EMERALD_900: Color = Color::rgb(0.02, 0.31, 0.23);

pub const FUCHSIA_50: Color = Color::rgb(0.99, 0.96, 1.00);
pub const FUCHSIA_100: Color = Color::rgb(0.98, 0.91, 1.00);
pub const FUCHSIA_200: Color = Color::rgb(0.96, 0.82, 1.00);
pub const FUCHSIA_300: Color = Color::rgb(0.94, 0.67, 0.99);
pub const FUCHSIA_400: Color = Color::rgb(0.91, 0.47, 0.98);
pub const FUCHSIA_500: Color = Color::rgb(0.85, 0.27, 0.94);
pub const FUCHSIA_600: Color = Color::rgb(0.75, 0.15, 0.83);
pub const FUCHSIA_700: Color = Color::rgb(0.64, 0.11, 0.69);
pub const FUCHSIA_800: Color = Color::rgb(0.53, 0.10, 0.56);
pub const FUCHSIA_900: Color = Color::rgb(0.44, 0.10, 0.46);

pub const GRAY_50: Color = Color::rgb(0.98, 0.98, 0.98);
pub const GRAY_100: Color = Color::rgb(0.95, 0.96, 0.96);
pub const GRAY_200: Color = Color::rgb(0.90, 0.91, 0.92);
pub const GRAY_300: Color = Color::rgb(0.82, 0.84, 0.86);
pub const GRAY_400: Color = Color::rgb(0.61, 0.64, 0.69);
pub const GRAY_500: Color = Color::rgb(0.42, 0.45, 0.50);
pub const GRAY_600: Color = Color::rgb(0.29, 0.33, 0.39);
pub const GRAY_700: Color = Color::rgb(0.22, 0.25, 0.32);
pub const GRAY_800: Color = Color::rgb(0.12, 0.16, 0.22);
pub const GRAY_900: Color = Color::rgb(0.07, 0.09, 0.15);

pub const GREEN_50: Color = Color::rgb(0.94, 0.99, 0.96);
pub const GREEN_100: Color = Color::rgb(0.86, 0.99, 0.91);
pub const GREEN_200: Color = Color::rgb(0.73, 0.97, 0.82);
pub const GREEN_300: Color = Color::rgb(0.53, 0.94, 0.67);
pub const GREEN_400: Color = Color::rgb(0.29, 0.87, 0.50);
pub const GREEN_500: Color = Color::rgb(0.13, 0.77, 0.37);
pub const GREEN_600: Color = Color::rgb(0.09, 0.64, 0.29);
pub const GREEN_700: Color = Color::rgb(0.08, 0.50, 0.24);
pub const GREEN_800: Color = Color::rgb(0.09, 0.40, 0.20);
pub const GREEN_900: Color = Color::rgb(0.08, 0.33, 0.18);

pub const INDIGO_50: Color = Color::rgb(0.93, 0.95, 1.00);
pub const INDIGO_100: Color = Color::rgb(0.88, 0.91, 1.00);
pub const INDIGO_200: Color = Color::rgb(0.78, 0.82, 1.00);
pub const INDIGO_300: Color = Color::rgb(0.65, 0.71, 0.99);
pub const INDIGO_400: Color = Color::rgb(0.51, 0.55, 0.97);
pub const INDIGO_500: Color = Color::rgb(0.39, 0.40, 0.95);
pub const INDIGO_600: Color = Color::rgb(0.31, 0.27, 0.90);
pub const INDIGO_700: Color = Color::rgb(0.26, 0.22, 0.79);
pub const INDIGO_800: Color = Color::rgb(0.22, 0.19, 0.64);
pub const INDIGO_900: Color = Color::rgb(0.19, 0.18, 0.51);

pub const LIME_50: Color = Color::rgb(0.97, 1.00, 0.91);
pub const LIME_100: Color = Color::rgb(0.93, 0.99, 0.80);
pub const LIME_200: Color = Color::rgb(0.85, 0.98, 0.62);
pub const LIME_300: Color = Color::rgb(0.75, 0.95, 0.39);
pub const LIME_400: Color = Color::rgb(0.64, 0.90, 0.21);
pub const LIME_500: Color = Color::rgb(0.52, 0.80, 0.09);
pub const LIME_600: Color = Color::rgb(0.40, 0.64, 0.05);
pub const LIME_700: Color = Color::rgb(0.30, 0.49, 0.06);
pub const LIME_800: Color = Color::rgb(0.25, 0.38, 0.07);
pub const LIME_900: Color = Color::rgb(0.21, 0.33, 0.08);

pub const NEUTRAL_50: Color = Color::rgb(0.98, 0.98, 0.98);
pub const NEUTRAL_100: Color = Color::rgb(0.96, 0.96, 0.96);
pub const NEUTRAL_200: Color = Color::rgb(0.90, 0.90, 0.90);
pub const NEUTRAL_300: Color = Color::rgb(0.83, 0.83, 0.83);
pub const NEUTRAL_400: Color = Color::rgb(0.64, 0.64, 0.64);
pub const NEUTRAL_500: Color = Color::rgb(0.45, 0.45, 0.45);
pub const NEUTRAL_600: Color = Color::rgb(0.32, 0.32, 0.32);
pub const NEUTRAL_700: Color = Color::rgb(0.25, 0.25, 0.25);
pub const NEUTRAL_800: Color = Color::rgb(0.15, 0.15, 0.15);
pub const NEUTRAL_900: Color = Color::rgb(0.09, 0.09, 0.09);

pub const ORANGE_50: Color = Color::rgb(1.00, 0.97, 0.93);
pub const ORANGE_100: Color = Color::rgb(1.00, 0.93, 0.84);
pub const ORANGE_200: Color = Color::rgb(1.00, 0.84, 0.67);
pub const ORANGE_300: Color = Color::rgb(0.99, 0.73, 0.45);
pub const ORANGE_400: Color = Color::rgb(0.98, 0.57, 0.24);
pub const ORANGE_500: Color = Color::rgb(0.98, 0.45, 0.09);
pub const ORANGE_600: Color = Color::rgb(0.92, 0.35, 0.05);
pub const ORANGE_700: Color = Color::rgb(0.76, 0.25, 0.05);
pub const ORANGE_800: Color = Color::rgb(0.60, 0.20, 0.07);
pub const ORANGE_900: Color = Color::rgb(0.49, 0.18, 0.07);

pub const PINK_50: Color = Color::rgb(0.99, 0.95, 0.97);
pub const PINK_100: Color = Color::rgb(0.99, 0.91, 0.95);
pub const PINK_200: Color = Color::rgb(0.98, 0.81, 0.91);
pub const PINK_300: Color = Color::rgb(0.98, 0.66, 0.83);
pub const PINK_400: Color = Color::rgb(0.96, 0.45, 0.71);
pub const PINK_500: Color = Color::rgb(0.93, 0.28, 0.60);
pub const PINK_600: Color = Color::rgb(0.86, 0.15, 0.47);
pub const PINK_700: Color = Color::rgb(0.75, 0.09, 0.36);
pub const PINK_800: Color = Color::rgb(0.62, 0.09, 0.30);
pub const PINK_900: Color = Color::rgb(0.51, 0.09, 0.26);

pub const PURPLE_50: Color = Color::rgb(0.98, 0.96, 1.00);
pub const PURPLE_100: Color = Color::rgb(0.95, 0.91, 1.00);
pub const PURPLE_200: Color = Color::rgb(0.91, 0.84, 1.00);
pub const PURPLE_300: Color = Color::rgb(0.85, 0.71, 1.00);
pub const PURPLE_400: Color = Color::rgb(0.75, 0.52, 0.99);
pub const PURPLE_500: Color = Color::rgb(0.66, 0.33, 0.97);
pub const PURPLE_600: Color = Color::rgb(0.58, 0.20, 0.92);
pub const PURPLE_700: Color = Color::rgb(0.49, 0.13, 0.81);
pub const PURPLE_800: Color = Color::rgb(0.42, 0.13, 0.66);
pub const PURPLE_900: Color = Color::rgb(0.35, 0.11, 0.53);

pub const RED_50: Color = Color::rgb(1.00, 0.95, 0.95);
pub const RED_100: Color = Color::rgb(1.00, 0.89, 0.89);
pub const RED_200: Color = Color::rgb(1.00, 0.79, 0.79);
pub const RED_300: Color = Color::rgb(0.99, 0.65, 0.65);
pub const RED_400: Color = Color::rgb(0.97, 0.44, 0.44);
pub const RED_500: Color = Color::rgb(0.94, 0.27, 0.27);
pub const RED_600: Color = Color::rgb(0.86, 0.15, 0.15);
pub const RED_700: Color = Color::rgb(0.73, 0.11, 0.11);
pub const RED_800: Color = Color::rgb(0.60, 0.11, 0.11);
pub const RED_900: Color = Color::rgb(0.50, 0.11, 0.11);

pub const ROSE_50: Color = Color::rgb(1.00, 0.95, 0.95);
pub const ROSE_100: Color = Color::rgb(1.00, 0.89, 0.90);
pub const ROSE_200: Color = Color::rgb(1.00, 0.80, 0.83);
pub const ROSE_300: Color = Color::rgb(0.99, 0.64, 0.69);
pub const ROSE_400: Color = Color::rgb(0.98, 0.44, 0.52);
pub const ROSE_500: Color = Color::rgb(0.96, 0.25, 0.37);
pub const ROSE_600: Color = Color::rgb(0.88, 0.11, 0.28);
pub const ROSE_700: Color = Color::rgb(0.75, 0.07, 0.24);
pub const ROSE_800: Color = Color::rgb(0.62, 0.07, 0.22);
pub const ROSE_900: Color = Color::rgb(0.53, 0.07, 0.22);

pub const SKY_50: Color = Color::rgb(0.94, 0.98, 1.00);
pub const SKY_100: Color = Color::rgb(0.88, 0.95, 1.00);
pub const SKY_200: Color = Color::rgb(0.73, 0.90, 0.99);
pub const SKY_300: Color = Color::rgb(0.49, 0.83, 0.99);
pub const SKY_400: Color = Color::rgb(0.22, 0.74, 0.97);
pub const SKY_500: Color = Color::rgb(0.05, 0.65, 0.91);
pub const SKY_600: Color = Color::rgb(0.01, 0.52, 0.78);
pub const SKY_700: Color = Color::rgb(0.01, 0.41, 0.63);
pub const SKY_800: Color = Color::rgb(0.03, 0.35, 0.52);
pub const SKY_900: Color = Color::rgb(0.05, 0.29, 0.43);

pub const SLATE_50: Color = Color::rgb(0.97, 0.98, 0.99);
pub const SLATE_100: Color = Color::rgb(0.95, 0.96, 0.98);
pub const SLATE_200: Color = Color::rgb(0.89, 0.91, 0.94);
pub const SLATE_300: Color = Color::rgb(0.80, 0.84, 0.88);
pub const SLATE_400: Color = Color::rgb(0.58, 0.64, 0.72);
pub const SLATE_500: Color = Color::rgb(0.39, 0.45, 0.55);
pub const SLATE_600: Color = Color::rgb(0.28, 0.33, 0.41);
pub const SLATE_700: Color = Color::rgb(0.20, 0.25, 0.33);
pub const SLATE_800: Color = Color::rgb(0.12, 0.16, 0.23);
pub const SLATE_900: Color = Color::rgb(0.06, 0.09, 0.16);

pub const STONE_50: Color = Color::rgb(0.98, 0.98, 0.98);
pub const STONE_100: Color = Color::rgb(0.96, 0.96, 0.96);
pub const STONE_200: Color = Color::rgb(0.91, 0.90, 0.89);
pub const STONE_300: Color = Color::rgb(0.84, 0.83, 0.82);
pub const STONE_400: Color = Color::rgb(0.66, 0.64, 0.62);
pub const STONE_500: Color = Color::rgb(0.47, 0.44, 0.42);
pub const STONE_600: Color = Color::rgb(0.34, 0.33, 0.31);
pub const STONE_700: Color = Color::rgb(0.27, 0.25, 0.24);
pub const STONE_800: Color = Color::rgb(0.16, 0.15, 0.14);
pub const STONE_900: Color = Color::rgb(0.11, 0.10, 0.09);

pub const TEAL_50: Color = Color::rgb(0.94, 0.99, 0.98);
pub const TEAL_100: Color = Color::rgb(0.80, 0.98, 0.95);
pub const TEAL_200: Color = Color::rgb(0.60, 0.96, 0.89);
pub const TEAL_300: Color = Color::rgb(0.37, 0.92, 0.83);
pub const TEAL_400: Color = Color::rgb(0.18, 0.83, 0.75);
pub const TEAL_500: Color = Color::rgb(0.08, 0.72, 0.65);
pub const TEAL_600: Color = Color::rgb(0.05, 0.58, 0.53);
pub const TEAL_700: Color = Color::rgb(0.06, 0.46, 0.43);
pub const TEAL_800: Color = Color::rgb(0.07, 0.37, 0.35);
pub const TEAL_900: Color = Color::rgb(0.07, 0.31, 0.29);

pub const VIOLET_50: Color = Color::rgb(0.96, 0.95, 1.00);
pub const VIOLET_100: Color = Color::rgb(0.93, 0.91, 1.00);
pub const VIOLET_200: Color = Color::rgb(0.87, 0.84, 1.00);
pub const VIOLET_300: Color = Color::rgb(0.77, 0.71, 0.99);
pub const VIOLET_400: Color = Color::rgb(0.65, 0.55, 0.98);
pub const VIOLET_500: Color = Color::rgb(0.55, 0.36, 0.96);
pub const VIOLET_600: Color = Color::rgb(0.49, 0.23, 0.93);
pub const VIOLET_700: Color = Color::rgb(0.43, 0.16, 0.85);
pub const VIOLET_800: Color = Color::rgb(0.36, 0.13, 0.71);
pub const VIOLET_900: Color = Color::rgb(0.30, 0.11, 0.58);

pub const YELLOW_50: Color = Color::rgb(1.00, 0.99, 0.91);
pub const YELLOW_100: Color = Color::rgb(1.00, 0.98, 0.76);
pub const YELLOW_200: Color = Color::rgb(1.00, 0.94, 0.54);
pub const YELLOW_300: Color = Color::rgb(0.99, 0.88, 0.28);
pub const YELLOW_400: Color = Color::rgb(0.98, 0.80, 0.08);
pub const YELLOW_500: Color = Color::rgb(0.92, 0.70, 0.03);
pub const YELLOW_600: Color = Color::rgb(0.79, 0.54, 0.02);
pub const YELLOW_700: Color = Color::rgb(0.63, 0.38, 0.03);
pub const YELLOW_800: Color = Color::rgb(0.52, 0.30, 0.05);
pub const YELLOW_900: Color = Color::rgb(0.44, 0.25, 0.07);

pub const ZINC_50: Color = Color::rgb(0.98, 0.98, 0.98);
pub const ZINC_100: Color = Color::rgb(0.96, 0.96, 0.96);
pub const ZINC_200: Color = Color::rgb(0.89, 0.89, 0.91);
pub const ZINC_300: Color = Color::rgb(0.83, 0.83, 0.85);
pub const ZINC_400: Color = Color::rgb(0.63, 0.63, 0.67);
pub const ZINC_500: Color = Color::rgb(0.44, 0.44, 0.48);
pub const ZINC_600: Color = Color::rgb(0.32, 0.32, 0.36);
pub const ZINC_700: Color = Color::rgb(0.25, 0.25, 0.27);
pub const ZINC_800: Color = Color::rgb(0.15, 0.15, 0.16);
pub const ZINC_900: Color = Color::rgb(0.09, 0.09, 0.11);
