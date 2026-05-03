use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::AEToolsApp;
use crate::config::{APP_ID, PROFILE};

use crate::ui::content::Content;
use crate::ui::preferences::Theme;
use crate::ui::sidebar::Sidebar;

mod imp {
    use std::cell::OnceCell;

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
            obj.set_settings();
            self.init_sidebar();
        }
    }

    impl WidgetImpl for AETApplicationWindow {}

    impl WindowImpl for AETApplicationWindow {}

    impl ApplicationWindowImpl for AETApplicationWindow {}

    impl AdwApplicationWindowImpl for AETApplicationWindow {}

    impl AETApplicationWindow {
        pub fn settings(&self) -> &gio::Settings {
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
            .parameter_type(Some(&i32::static_variant_type()))
            .activate(move |win: &Self, _, param| {
                let size = param.map_or(0, |s| s.get::<i32>().unwrap_or(0));
                win.imp().content.increment_font_size(size);
            })
            .build();

        let decrement_view_font_size = gio::ActionEntry::builder("decrement-view-font-size")
            .parameter_type(Some(&i32::static_variant_type()))
            .activate(move |win: &Self, _, param| {
                let size = param.map_or(0, |s| s.get::<i32>().unwrap_or(0));
                win.imp().content.decrement_font_size(size);
            })
            .build();

        let change_view_font = gio::ActionEntry::builder("change-view-font")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |win: &Self, _, param| {
                let size = param.map_or(String::new(), |s| {
                    s.get::<String>().unwrap_or(String::new())
                });
                win.imp().content.change_view_font(&size);
            })
            .build();

        self.add_action_entries([
            show_toast,
            sidebar_activated,
            set_color_scheme,
            increment_view_font_size,
            decrement_view_font_size,
            change_view_font,
        ]);
    }

    fn set_settings(&self) {
        let settings = self.imp().settings();

        settings.bind("window-width", self, "default-width").build();
        settings
            .bind("window-height", self, "default-height")
            .build();
        settings.bind("is-maximized", self, "maximized").build();

        // Get setting and set chooser
        let theme: Theme = settings.string("theme").as_str().into();
        adw::StyleManager::default().set_color_scheme(theme.into());
    }
}
