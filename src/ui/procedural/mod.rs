mod possibility;
mod possibility_data;

use adw::prelude::*;
use adw::subclass::prelude::*;
use anyhow::Result;
use gtk::gio::ListStore;
use gtk::glib::Object;
use gtk::{NoSelection, Widget, gdk, gio, glib};
use std::cell::RefCell;
use tracing::error;

use crate::tools::procedural::*;
use crate::ui::procedural::possibility::ProceduralPossibility;
use crate::ui::procedural::possibility_data::*;

mod imp {
    use adw::{ButtonRow, EntryRow, ExpanderRow, SpinRow};
    use anyhow::anyhow;
    use gtk::{Button, ListBox};

    use crate::utils::output_clipboard;

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
        pub(super) fn show_toast(&self, message: &str) {
            self.obj()
                .activate_action("win.show-toast", Some(&message.to_variant()))
                .map_or_else(|e| error!("Could not show toast: {e}."), |_| ());
        }

        fn build_procedural(&self) -> Result<Procedural> {
            let mut builder = Procedural::builder();
            let name = self.proc_name.text();
            if !name.is_empty() {
                builder.name(name.as_str());
            }
            builder.chance(self.proc_chance.value());

            let possibilities = self
                .possibilities
                .borrow()
                .clone()
                .ok_or(anyhow!("Cannot get possibilities."))?;

            for item in possibilities.into_iter() {
                let poss = item.map(|o| o)?;
                let poss = poss
                    .downcast_ref::<PossibilityData>()
                    .ok_or(anyhow!("Could not get PossibilityData"))?;
                builder.possibility(poss.name().as_deref(), Some(poss.chance()));
            }

            builder.build()
        }

        #[template_callback]
        fn on_proc_clear_btn_clicked(&self) {}
        #[template_callback]
        fn on_poss_add_btn_clicked(&self) {}
        #[template_callback]
        fn on_names_generate_btn_activated(&self) {
            let proc = self.build_procedural();

            match proc {
                Ok(proc) => {
                    if let Err(e) = output_clipboard(&proc.generate_procedural_string()) {
                        self.show_toast(&e.to_string());
                    } else {
                        self.show_toast("Copied procedural to clipboard.");
                    }
                }
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_poss_generate_btn_activated(&self) {
            let proc = self.build_procedural();

            match proc {
                Ok(proc) => {
                    // proc.generate_possible_names();
                    match proc.generate_possible_names(PossibleFlag::None) {
                        Ok(names) => {
                            if let Err(e) = output_clipboard(&names) {
                                self.show_toast(&e.to_string());
                            } else {
                                self.show_toast("Copied possible names to clipboard.");
                            }
                        }
                        Err(e) => {
                            self.show_toast(&e.to_string());
                        }
                    }
                }
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_containing_phrases_generate_btn_activated(&self) {
            let proc = self.build_procedural();

            todo!()
        }
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
