use adw::StyleManager;
use anyhow::{Result, anyhow, bail};
use gtk::gdk::{self, prelude::DisplayExt};
use gtk::glib;
use gtk::pango::FontDescription;
use sourceview5::{Buffer, View, prelude::*};

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
    let font_size = font.size().to_string();
    let font_family = font.family().unwrap_or("Adwaita Mono".into());
    let css = format!(".custom-font {{ font-family: {font_family}; font-size: {font_size}pt; }}");
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

pub fn generate_font(font_family: &str, font_size: i32) -> FontDescription {
    let mut font = FontDescription::new();
    font.set_family(font_family);
    font.set_size(font_size);
    font
}
