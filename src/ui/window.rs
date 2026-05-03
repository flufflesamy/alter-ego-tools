use adw::prelude::*;
use adw::subclass::prelude::*;
use anyhow::Result;
use gtk::{gio, glib};

use crate::application::AEToolsApp;
use crate::config::{APP_ID, PROFILE};

use crate::ui::content::Content;
use crate::ui::preferences::Theme;
use crate::ui::sidebar::Sidebar;

mod imp {
    use std::cell::OnceCell;

    use crate::utils::{generate_font, set_view_font};

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/window.ui")]
    pub struct AETApplicationWindow {
        settings: OnceCell<gio::Settings>,
        #[template_child]
        pub(super) split_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        pub(super) sidebar: TemplateChild<Sidebar>,
        #[template_child]
        pub(super) content: TemplateChild<Content>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETApplicationWindow {
        const NAME: &'static str = "AETApplicationWindow";
        type Type = super::AETApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            // klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            if *PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            self.setup_settings();
            obj.setup_actions();
            self.load_window_state();
            self.init_sidebar();
            self.update_view_font();
        }
    }

    impl WidgetImpl for AETApplicationWindow {}

    impl WindowImpl for AETApplicationWindow {
        // Save window state on close
        fn close_request(&self) -> glib::Propagation {
            if let Err(err) = self.save_window_state() {
                tracing::warn!("Failed to save window state: {}", err);
            }

            // Pass the close request to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for AETApplicationWindow {}

    impl AdwApplicationWindowImpl for AETApplicationWindow {}

    impl AETApplicationWindow {
        pub(super) fn settings(&self) -> &gio::Settings {
            self.settings
                .get()
                .expect("`settings` should be set in `setup_settings`.")
        }

        fn setup_settings(&self) {
            let settings = gio::Settings::new(*APP_ID);
            self.settings
                .set(settings)
                .expect("`settings` should not be set before calling `setup_settings`.");
        }

        fn init_sidebar(&self) {
            let sidebar = self.sidebar.get().imp().sidebar.get();
            let stack = self.content.get().imp().stack.get();

            sidebar.set_stack(Some(&stack));
        }

        pub(super) fn load_window_state(&self) {
            let settings = self.settings();

            // Set window size
            let width = settings.int("window-width");
            let height = settings.int("window-height");
            self.obj().set_default_size(width, height);

            // Set maximized state
            let maximized = settings.boolean("is-maximized");
            self.obj().set_maximized(maximized);

            // Set window theme from settings
            let theme: Theme = settings.string("theme").as_str().into();
            adw::StyleManager::default().set_color_scheme(theme.into());
        }

        pub(super) fn update_view_font(&self) {
            let settings = self.settings();
            let description = self.content.get().imp().description.get();
            let procedural = self.content.get().imp().procedural.get();
            let views = [
                description.imp().input_text.get(),
                description.imp().output_text.get(),
                procedural.imp().source_view.get(),
            ];

            // Set view font
            let font_family = settings.string("view-font-family");
            let font_size = settings.int("view-font-size");
            let font = generate_font(&font_family, font_size);
            for view in views {
                set_view_font(&view, &font).unwrap();
            }
        }

        fn save_window_state(&self) -> Result<()> {
            let settings = self.settings();
            settings.set_int("window-width", self.obj().width())?;
            settings.set_int("window-height", self.obj().height())?;
            settings.set_boolean("is-maximized", self.obj().is_maximized())?;
            Ok(())
        }
    }
}

glib::wrapper! {
    pub struct AETApplicationWindow(ObjectSubclass<imp::AETApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AETApplicationWindow {
    pub fn new(app: &AEToolsApp) -> Self {
        // Create new window
        glib::Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let show_toast = gio::ActionEntry::builder("show-toast")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |win: &Self, _, param| {
                let msg = param.map_or(String::new(), |m| {
                    m.get::<String>().map_or(String::new(), |m| m)
                });
                win.imp().content.get().show_toast(&msg);
            })
            .build();

        let sidebar_activated = gio::ActionEntry::builder("sidebar-activated")
            .activate(move |win: &Self, _, _| {
                win.imp().split_view.set_show_content(true);
            })
            .build();

        let set_color_scheme = gio::ActionEntry::builder("set-color-scheme")
            .parameter_type(Some(&u32::static_variant_type()))
            .activate(move |_, _, param| {
                let manager = adw::StyleManager::default();
                let theme: Theme = param.map_or(0, |t| t.get::<u32>().unwrap_or(0)).into();
                manager.set_color_scheme(theme.into());
            })
            .build();

        let increment_view_font_size = gio::ActionEntry::builder("increment-view-font-size")
            .activate(move |win: &Self, _, _| {
                let settings = win.imp().settings();
                let font_size = settings.int("view-font-size");
                let out = if font_size >= 100 { 100 } else { font_size + 1 };
                settings
                    .set_int("view-font-size", out)
                    .expect("Failed to increment font size");
                win.imp().update_view_font();
            })
            .build();

        let decrement_view_font_size = gio::ActionEntry::builder("decrement-view-font-size")
            .activate(move |win: &Self, _, _| {
                let settings = win.imp().settings();
                let font_size = settings.int("view-font-size");
                let out = if font_size <= 1 { 1 } else { font_size - 1 };
                settings
                    .set_int("view-font-size", out)
                    .expect("Failed to increment font size");
                win.imp().update_view_font();
            })
            .build();

        let reset_view_font_size = gio::ActionEntry::builder("reset-view-font-size")
            .activate(move |win: &Self, _, _| {
                let settings = win.imp().settings();
                settings.reset("view-font-size");
                win.imp().update_view_font();
            })
            .build();

        // let change_view_font = gio::ActionEntry::builder("change-view-font")
        //     .parameter_type(Some(&String::static_variant_type()))
        //     .activate(move |win: &Self, _, param| {
        //         let font_string = param.map_or(String::new(), |s| {
        //             s.get::<String>().unwrap_or(String::new())
        //         });
        //         let font = FontDescription::from_string(&font_string);
        //         win.imp().content.change_view_font(&font);
        //     })
        //     .build();

        let update_view_font = gio::ActionEntry::builder("update-view-font")
            .activate(move |win: &Self, _, _| {
                win.imp().update_view_font();
            })
            .build();

        let load_window_state = gio::ActionEntry::builder("load-window-state")
            .activate(move |win: &Self, _, _| {
                win.imp().load_window_state();
            })
            .build();

        self.add_action_entries([
            show_toast,
            sidebar_activated,
            set_color_scheme,
            increment_view_font_size,
            decrement_view_font_size,
            // change_view_font,
            load_window_state,
            update_view_font,
            reset_view_font_size,
        ]);
    }
}
