// SPDX-FileCopyrightText: 2026 Amy Poon <amy@amypoon.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod theme;

use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use anyhow::Result;
use gtk::gio::Settings;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::{self};
pub(crate) use theme::Theme;
use tracing::*;

use crate::config::APP_ID;

macro_rules! toast {
    ($dialog:expr, $msg:literal) => {{
        let toast = adw::Toast::new($msg);
        toast.set_timeout(1);
        $dialog.add_toast(toast);
    }};
}

macro_rules! toast_error {
    ($dialog:expr, $msg:literal, $e:expr) => {{
        toast!($dialog, $msg);
        tracing::error!("$msg: {}", $e);
    }};
}

mod imp {

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/preferences.ui")]
    #[properties(wrapper_type = super::AETPreferencesDialog)]
    pub struct AETPreferencesDialog {
        settings: OnceCell<Settings>,
        #[template_child]
        theme_chooser: TemplateChild<adw::ComboRow>,
        #[template_child]
        font_size_chooser: TemplateChild<adw::SpinRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETPreferencesDialog {
        const NAME: &'static str = "AETPreferencesDialog";
        type Type = super::AETPreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            adw::ButtonRow::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
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
            self.setup_font_size_chooser();
        }
    }

    impl WidgetImpl for AETPreferencesDialog {}
    impl AdwDialogImpl for AETPreferencesDialog {}
    impl PreferencesDialogImpl for AETPreferencesDialog {}

    #[gtk::template_callbacks]
    impl AETPreferencesDialog {
        fn settings(&self) -> &Settings {
            self.settings.get().expect("Could not get settings")
        }

        fn setup_settings(&self) {
            let settings = Settings::new(*APP_ID);

            self.settings.get_or_init(|| settings);
        }

        fn setup_theme_chooser(&self) {
            let chooser = self.theme_chooser.get();
            let settings = self.settings();

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
                        .unwrap_or_else(|e| error!("Could not save theme to settings: {e}"));
                    // Activate theme change action
                    pref.obj()
                        .activate_action("win.load-window-state", None)
                        .unwrap_or_else(|e| error!("Could not set theme: {e}"));
                }
            ));
        }

        fn setup_font_size_chooser(&self) {
            let chooser = self.font_size_chooser.get();
            let settings = self.settings();

            // Initilaize value
            chooser.set_value(settings.int("view-font-size").into());

            chooser.connect_value_notify(clone!(
                #[weak]
                settings,
                #[weak(rename_to=pref)]
                self,
                move |adj| {
                    let value = adj.value();
                    settings
                        .set_int("view-font-size", value.round() as i32)
                        .unwrap_or_else(|e| error!("Could not save font size to settings: {e}"));
                    pref.obj()
                        .activate_action("win.update-view-font", None)
                        .unwrap_or_else(|e| error!("Could not update view font: {e}"));
                }
            ));
        }

        fn reset_window_state(&self) -> Result<()> {
            let settings = self.settings();
            settings.reset("window-width");
            settings.reset("window-height");
            settings.reset("is-maximized");

            self.obj().activate_action("win.load-window-state", None)?;
            Ok(())
        }

        fn reset_all_settings(&self) -> Result<()> {
            let settings = self.settings();

            // Theme
            settings.reset("theme");
            self.theme_chooser.get().set_selected(0);

            // Font Size
            settings.reset("view-font-size");
            self.font_size_chooser
                .get()
                .set_value(settings.int("view-font-size").into());

            self.obj().activate_action("win.load-window-state", None)?;
            self.obj().activate_action("win.update-view-font", None)?;
            Ok(())
        }

        #[template_callback]
        fn on_reset_window_state_btn_pressed(&self) {
            match self.reset_window_state() {
                Ok(_) => toast!(self.obj(), "Reset Complete"),
                Err(e) => toast_error!(self.obj(), "Could not reset window state", e),
            }
        }

        #[template_callback]
        fn on_reset_all_btn_pressed(&self) {
            match self.reset_all_settings() {
                Ok(_) => toast!(self.obj(), "Settings Reset"),
                Err(e) => toast_error!(self.obj(), "Could not reset settings", e),
            }
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
