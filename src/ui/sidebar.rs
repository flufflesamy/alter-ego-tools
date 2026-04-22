use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use gtk::template_callbacks;
    use tracing::error;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/sidebar.ui")]
    pub struct AETSidebar {
        #[template_child]
        pub sidebar: TemplateChild<adw::ViewSwitcherSidebar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETSidebar {
        const NAME: &'static str = "AETSidebar";
        type Type = super::Sidebar;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETSidebar {
        fn constructed(&self) {
            self.parent_constructed();
            // let obj = self.obj();
        }
    }

    impl WidgetImpl for AETSidebar {}

    impl NavigationPageImpl for AETSidebar {}

    #[template_callbacks]
    impl AETSidebar {
        #[template_callback]
        fn on_sidebar_activated(&self) {
            self.obj()
                .activate_action("win.sidebar-activated", None)
                .map_or_else(
                    |e| error!("Could not activate sidebar_activated action: {e}."),
                    |_| (),
                );
        }
    }
}

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::AETSidebar>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Sidebar {}
