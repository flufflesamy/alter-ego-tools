use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{NavigationPage, Toast, ToastOverlay, ViewStack, ViewStackPage};
use gtk::glib;
use tracing::warn;

use crate::ui::description::Description;
use crate::ui::procedural::ContentProcedural;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/content.ui")]
    #[properties(wrapper_type = super::Content)]
    pub struct AETContent {
        #[template_child]
        pub stack: TemplateChild<ViewStack>,
        #[template_child]
        description: TemplateChild<Description>,
        #[template_child]
        procedural: TemplateChild<ContentProcedural>,
        #[template_child]
        toast_overlay: TemplateChild<ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETContent {
        const NAME: &'static str = "AETContent";
        type Type = super::Content;
        type ParentType = NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AETContent {
        fn constructed(&self) {
            self.parent_constructed();
            self.set_title();
        }
    }

    impl WidgetImpl for AETContent {}

    impl NavigationPageImpl for AETContent {}

    #[gtk::template_callbacks]
    impl AETContent {
        fn current_page(&self) -> Option<ViewStackPage> {
            self.stack
                .visible_child()
                .map(|widget| self.stack.page(&widget))
        }

        fn current_page_title(&self) -> String {
            if let Some(page) = self.current_page() {
                page.title().map_or_else(
                    || {
                        warn!("Could not get page title, page title not found.");
                        "".to_string()
                    },
                    |t| t.to_string(),
                )
            } else {
                warn!("Could not get page title, current page not found.");
                "".to_string()
            }
        }

        fn set_title(&self) {
            let obj = self.obj();
            let title = self.current_page_title();
            obj.set_title(&title);
        }

        pub(super) fn show_toast(&self, msg: &str) {
            let toast = Toast::builder().title(msg).timeout(1).build();
            self.toast_overlay.add_toast(toast);
        }

        #[template_callback]
        fn on_visible_child_notify(&self) {
            self.set_title();
        }
    }
}

glib::wrapper! {
    pub struct Content(ObjectSubclass<imp::AETContent>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Content {
    pub(crate) fn show_toast(&self, msg: &str) {
        self.imp().show_toast(msg);
    }

    pub fn increment_font_size(&self, size: i32) {
        todo!()
    }

    pub fn decrement_font_size(&self, size: i32) {
        todo!()
    }

    pub fn change_view_font(&self, font_string: &str) {
        todo!()
    }
}
