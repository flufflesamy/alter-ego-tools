use adw::StyleManager;
use anyhow::{Result, bail};
use gtk::gdk::{self, prelude::DisplayExt};
use gtk::glib;
use sourceview5::{Buffer, prelude::*};

pub fn output_clipboard(content: &str) -> Result<()> {
    let clipboard = match gdk::Display::default() {
        Some(display) => display.clipboard(),
        None => bail!("Could not get display."),
    };

    gdk::Clipboard::set_text(&clipboard, content);

    Ok(())
}

/// Changes sourceview5 buffer color to match system theme
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

/// Sets the language of the sourceview5 buffer
pub fn buffer_language(buffer: &Buffer, language: &str) {
    if let Some(ref language) = sourceview5::LanguageManager::new().language(language) {
        buffer.set_language(Some(language));
    } else {
        tracing::debug!("Language not found");
    }
}
