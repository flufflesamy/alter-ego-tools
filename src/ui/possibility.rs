use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};

mod imp {
    use gtk::template_callbacks;

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/possibility.ui")]
    pub struct AETProceduralPossibility {}

    #[glib::object_subclass]
    impl ObjectSubclass for AETProceduralPossibility {
        const NAME: &'static str = "AETProceduralPossibility";
        type Type = super::ProceduralPossibility;
        type ParentType = adw::PreferencesGroup;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETProceduralPossibility {}

    impl WidgetImpl for AETProceduralPossibility {}

    impl PreferencesGroupImpl for AETProceduralPossibility {}

    #[template_callbacks]
    impl AETProceduralPossibility {
        #[template_callback]
        fn on_pos_remove_btn_clicked() {}
    }
}

glib::wrapper! {
    pub struct ProceduralPossibility(ObjectSubclass<imp::AETProceduralPossibility>)
        @extends gtk::Widget, adw::PreferencesGroup,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ProceduralPossibility {
    pub fn new(&self) {}
}
