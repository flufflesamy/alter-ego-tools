use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::tools::procedural::*;

mod imp {
    use adw::{EntryRow, ExpanderRow, PreferencesPage, SpinRow};
    use gtk::Button;

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/procedural.ui")]
    pub struct AETContentProcedural {
        #[template_child]
        proc_page: TemplateChild<PreferencesPage>,
        #[template_child]
        proc_poss_add_btn: TemplateChild<Button>,
        #[template_child]
        proc_name: TemplateChild<EntryRow>,
        #[template_child]
        proc_chance: TemplateChild<SpinRow>,
        #[template_child]
        proc_stat: TemplateChild<ExpanderRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETContentProcedural {
        const NAME: &'static str = "AETContentProcedural";
        type Type = super::ContentProcedural;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETContentProcedural {}

    impl WidgetImpl for AETContentProcedural {}

    impl BreakpointBinImpl for AETContentProcedural {}

    #[gtk::template_callbacks]
    impl AETContentProcedural {
        #[template_callback]
        fn on_pos_add_btn_clicked(&self) {}
    }
}

glib::wrapper! {
    pub struct ContentProcedural(ObjectSubclass<imp::AETContentProcedural>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
