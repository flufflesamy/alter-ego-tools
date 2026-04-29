mod application;
mod config;
mod tools;
mod ui;
mod utils;

use gettextrs::{LocaleCategory, gettext};
use gtk::{gio, glib};

use self::application::AEToolsApp;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() -> glib::ExitCode {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(*GETTEXT_PACKAGE, *LOCALEDIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(*GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Alter Ego Tools"));

    let res = gio::Resource::load(*RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = AEToolsApp::default();
    app.run()
}
#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
