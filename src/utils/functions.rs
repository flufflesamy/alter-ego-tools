use adw::StyleManager;
use anyhow::{Result, anyhow, bail};
use gtk::gdk::{self, prelude::DisplayExt};
use gtk::glib;
use gtk::pango::FontDescription;
use sourceview5::{Buffer, View, prelude::*};

use crate::utils::macros::*;

/// Outputs the input string slice to the clipboard.
pub fn output_clipboard(content: &str) -> Result<()> {
    let clipboard = match gdk::Display::default() {
        Some(display) => display.clipboard(),
        None => bail!("Could not get display."),
    };

    gdk::Clipboard::set_text(&clipboard, content);

    Ok(())
}

/// Changes sourceview5 buffer color to match system theme.
pub fn buffer_color(manager: &StyleManager, buffer: &Buffer) {
    let set_color = |manager: &StyleManager, buffer: &Buffer| {
        // Pick style scheme based on system color scheme
        let scheme_name = if manager.is_dark() {
            "Adwaita-dark"
        } else {
            "Adwaita"
        };

        // Set up the source view with Adwaita style scheme
        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme(scheme_name) {
            buffer.set_style_scheme(Some(scheme));
        } else {
            tracing::debug!("Style scheme {scheme_name} not found");
        }
    };

    set_color(manager, buffer);

    // Connect dark mode notification to update buffer color
    manager.connect_dark_notify(glib::clone!(
        #[weak]
        buffer,
        move |manager| set_color(manager, &buffer)
    ));
}

/// Sets the language of the sourceview5 buffer.
pub fn buffer_language(buffer: &Buffer, language: &str) {
    if let Some(ref language) = sourceview5::LanguageManager::new().language(language) {
        buffer.set_language(Some(language));
    } else {
        tracing::debug!("Language not found");
    }
}

/// Sets the font of the sourceview5 view from a font string.
pub fn set_view_font(view: &View, font: &FontDescription) -> Result<()> {
    // Remove old custom-font class to avoid conflicts
    view.remove_css_class("custom-font");
    view.add_css_class("custom-font");
    let provider = gtk::CssProvider::default();
    let font_string = font.to_str();
    let css = format!(".custom-font {{ font: {font_string}; }}");
    provider.load_from_string(&css);

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        Ok(())
    } else {
        Err(anyhow!("Failed to set view font: display not available"))
    }
}

/// Increments font size by size parameter.
pub fn increment_font_size(font: &mut FontDescription, size: i32) -> &FontDescription {
    font.set_size(font.size() + size);
    font
}

/// Decrements font size by size parameter.
pub fn decrement_font_size(font: &mut FontDescription, size: i32) -> &FontDescription {
    font.set_size(font.size() - size);
    font
}

/// Gets the default monospace font.
pub fn default_font() -> FontDescription {
    let font = adw::StyleManager::default().monospace_font_name();
    FontDescription::from_string(&font)
}
