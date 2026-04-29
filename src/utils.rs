use anyhow::{Result, bail};
use gtk::{
    gdk::{self, prelude::DisplayExt},
    gio, glib,
};

pub fn output_clipboard(content: &str) -> Result<()> {
    let clipboard = match gdk::Display::default() {
        Some(display) => display.clipboard(),
        None => bail!("Could not get display."),
    };

    gdk::Clipboard::set_text(&clipboard, content);

    Ok(())
}
