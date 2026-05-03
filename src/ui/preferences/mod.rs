mod theme;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::OnceCell;
use tracing::*;

use crate::config::APP_ID;

pub(crate) use theme::Theme;

mod imp {

    use gtk::glib::{
        clone,
        subclass::{self},
    };

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/preferences.ui")]
    #[properties(wrapper_type = super::AETPreferencesDialog)]
    pub struct AETPreferencesDialog {
        settings: OnceCell<gio::Settings>,
        #[template_child]
        theme_chooser: TemplateChild<adw::ComboRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETPreferencesDialog {
        const NAME: &'static str = "AETPreferencesDialog";
        type Type = super::AETPreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETPreferencesDialog {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup_settings();
            self.setup_theme_chooser();
        }
    }

    impl WidgetImpl for AETPreferencesDialog {}
    impl AdwDialogImpl for AETPreferencesDialog {}
    impl PreferencesDialogImpl for AETPreferencesDialog {}

    impl AETPreferencesDialog {
        fn setup_settings(&self) {
            let settings = gio::Settings::new(*APP_ID);

            self.settings.get_or_init(|| settings);
        }

        fn setup_theme_chooser(&self) {
            let chooser = self.theme_chooser.get();
            let settings = self.settings.get().expect("Could not get settings");

            // Get setting and set chooser
            let theme: Theme = settings.string("theme").as_str().into();
            chooser.set_selected(theme.into());

            chooser.connect_selected_notify(clone!(
                #[weak]
                settings,
                #[weak(rename_to=pref)]
                self,
                move |chooser| {
                    let theme: Theme = chooser.selected().into();
                    // Save selected theme to settings
                    settings
                        .set_string("theme", &theme.to_string())
                        .unwrap_or_else(|_| error!("Could not save theme to settings"));
                    // Activate theme change action
                    pref.obj()
                        .activate_action("win.set-color-scheme", Some(&theme.to_variant()))
                        .unwrap_or_else(|e| error!("Could not set theme {e}"));
                }
            ));
        }
    }
}

glib::wrapper! {
    pub struct AETPreferencesDialog(ObjectSubclass<imp::AETPreferencesDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AETPreferencesDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
