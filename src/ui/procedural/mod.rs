mod possibility;
mod possibility_data;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::ListStore;
use gtk::glib::Object;
use gtk::{NoSelection, Widget, gdk, gio, glib};
use std::cell::RefCell;

use crate::tools::procedural::*;
use crate::ui::procedural::possibility::ProceduralPossibility;
use crate::ui::procedural::possibility_data::*;

mod imp {
    use adw::{ButtonRow, EntryRow, ExpanderRow, SpinRow};
    use gtk::{Button, ListBox};

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/procedural.ui")]
    pub struct AETContentProcedural {
        pub possibilities: RefCell<Option<ListStore>>,
        #[template_child]
        proc_poss_add_btn: TemplateChild<Button>,
        #[template_child]
        proc_clear_btn: TemplateChild<Button>,
        #[template_child]
        proc_name: TemplateChild<EntryRow>,
        #[template_child]
        proc_chance: TemplateChild<SpinRow>,
        #[template_child]
        proc_stat: TemplateChild<ExpanderRow>,
        #[template_child]
        pub poss_list_box: TemplateChild<ListBox>,
        #[template_child]
        poss_generate_btn: TemplateChild<ButtonRow>,
        #[template_child]
        names_generate_btn: TemplateChild<ButtonRow>,
        #[template_child]
        containing_phrases_generate_btn: TemplateChild<ButtonRow>,
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

    impl ObjectImpl for AETContentProcedural {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = &self.obj();

            obj.possibilities();
            obj.setup_possibilities();
        }
    }

    impl WidgetImpl for AETContentProcedural {}

    impl BreakpointBinImpl for AETContentProcedural {}

    #[gtk::template_callbacks]
    impl AETContentProcedural {
        #[template_callback]
        fn on_proc_clear_btn_clicked(&self) {}
        #[template_callback]
        fn on_poss_add_btn_clicked(&self) {}
        #[template_callback]
        fn on_names_generate_btn_activated(&self) {}
        #[template_callback]
        fn on_poss_generate_btn_activated(&self) {}
        #[template_callback]
        fn on_containing_phrases_generate_btn_activated(&self) {}
    }
}

glib::wrapper! {
    pub struct ContentProcedural(ObjectSubclass<imp::AETContentProcedural>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ContentProcedural {
    fn possibilities(&self) -> ListStore {
        // Get state
        self.imp()
            .possibilities
            .borrow()
            .clone()
            .expect("Could not get current possibilities")
    }

    fn setup_possibilities(&self) {
        // Create new model
        let model = ListStore::new::<PossibilityData>();

        // Get state and set model
        self.imp().possibilities.replace(Some(model));

        // Bind model to listbox
        self.imp()
            .poss_list_box
            .bind_model(Some(&self.possibilities()), |item| {
                ProceduralPossibility::new(
                    item.downcast_ref::<PossibilityData>()
                        .expect("PossibilityData is of wrong type"),
                )
                .upcast::<gtk::Widget>()
            });
    }
}
