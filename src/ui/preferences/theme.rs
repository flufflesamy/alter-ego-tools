use adw::ColorScheme;
use adw::prelude::*;
use gtk::glib;
use gtk::glib::Variant;
use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "AETTheme")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        };

        write!(f, "{s}")
    }
}

impl From<&str> for Theme {
    fn from(value: &str) -> Self {
        match value {
            "Light" => Theme::Light,
            "Dark" => Theme::Dark,
            _ => Theme::System,
        }
    }
}

impl From<u32> for Theme {
    fn from(value: u32) -> Self {
        match value {
            0 => Theme::System,
            1 => Theme::Light,
            2 => Theme::Dark,
            _ => Theme::System,
        }
    }
}

impl From<Theme> for u32 {
    fn from(value: Theme) -> Self {
        match value {
            Theme::System => 0,
            Theme::Light => 1,
            Theme::Dark => 2,
        }
    }
}

impl ToVariant for Theme {
    fn to_variant(&self) -> Variant {
        let num: u32 = (*self).into();
        num.to_variant()
    }
}

impl From<Theme> for ColorScheme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::System => ColorScheme::Default,
            Theme::Light => ColorScheme::ForceLight,
            Theme::Dark => ColorScheme::ForceDark,
        }
    }
}
