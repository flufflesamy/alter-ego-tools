use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::ui::procedural::possibility_data::PossibilityData;

mod imp {
    use adw::{EntryRow, SpinRow, SwitchRow};
    use gtk::{Button, glib::subclass::Signal, template_callbacks};
    use std::{cell::RefCell, sync::OnceLock};

    use super::*;

    #[derive(Default, Debug, glib::Properties, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/possibility.ui")]
    #[properties(wrapper_type = super::ProceduralPossibility)]
    pub struct AETProceduralPossibility {
        #[property(get, construct_only)]
        pub possibility_data: RefCell<Option<PossibilityData>>,
        #[template_child]
        poss_remove_btn: TemplateChild<Button>,
        #[template_child]
        poss_name: TemplateChild<EntryRow>,
        #[template_child]
        poss_chance_enabled: TemplateChild<SwitchRow>,
        #[template_child]
        poss_chance: TemplateChild<SpinRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETProceduralPossibility {
        const NAME: &'static str = "AETProceduralPossibility";
        type Type = super::ProceduralPossibility;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AETProceduralPossibility {
        fn constructed(&self) {
            self.parent_constructed();

            // Bind data to controls
            let data = self
                .possibility_data
                .borrow()
                .as_ref()
                .cloned()
                .expect("possibility_data is None");
            let name = self.poss_name.get();
            let chance = self.poss_chance.get();
            let enabled = self.poss_chance_enabled.get();
            data.bind_property("name", &name, "text")
                .bidirectional()
                .build();
            data.bind_property("chance", &chance, "value")
                .bidirectional()
                .build();
            data.bind_property("chance_enabled", &enabled, "active")
                .bidirectional()
                .build();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("remove").build()])
        }
    }

    impl WidgetImpl for AETProceduralPossibility {}

    impl ListBoxRowImpl for AETProceduralPossibility {}

    #[template_callbacks]
    impl AETProceduralPossibility {
        #[template_callback]
        fn on_poss_remove_btn_clicked(&self) {
            // Emit remove signal
            self.obj().emit_by_name::<()>("remove", &[]);
        }
    }
}

glib::wrapper! {
    pub struct ProceduralPossibility(ObjectSubclass<imp::AETProceduralPossibility>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ProceduralPossibility {
    pub fn new(possibility_data: &PossibilityData) -> Self {
        glib::Object::builder()
            .property("possibility-data", possibility_data)
            .build()
    }
}
