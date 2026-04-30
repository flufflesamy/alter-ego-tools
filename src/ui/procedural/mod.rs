mod possibility;
mod possibility_data;

use adw::prelude::*;
use adw::subclass::prelude::*;
use anyhow::Result;
use gtk::gio::ListStore;
use gtk::glib;
use std::cell::RefCell;
use tracing::error;

use crate::tools::procedural::*;
use crate::ui::procedural::possibility::ProceduralPossibility;
use crate::ui::procedural::possibility_data::*;

mod imp {
    use adw::{ButtonRow, EntryRow, ExpanderRow, SpinRow, SwitchRow};
    use anyhow::anyhow;
    use gtk::{
        Button, ListBox,
        glib::{clone, closure, closure_local},
    };

    use crate::utils::output_clipboard;

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/procedural.ui")]
    pub struct AETContentProcedural {
        possibilities: RefCell<Option<ListStore>>,
        #[template_child]
        proc_poss_add_btn: TemplateChild<Button>,
        #[template_child]
        proc_clear_btn: TemplateChild<Button>,
        #[template_child]
        proc_name: TemplateChild<EntryRow>,
        #[template_child]
        proc_chance_enabled: TemplateChild<SwitchRow>,
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

            self.setup_possibilities();
        }
    }

    impl WidgetImpl for AETContentProcedural {}

    impl BreakpointBinImpl for AETContentProcedural {}

    #[gtk::template_callbacks]
    impl AETContentProcedural {
        fn show_toast(&self, message: &str) {
            self.obj()
                .activate_action("win.show-toast", Some(&message.to_variant()))
                .map_or_else(|e| error!("Could not show toast: {e}."), |_| ());
        }

        fn get_poss(&self) -> ListStore {
            // Get state
            self.possibilities
                .borrow()
                .clone()
                .expect("Could not get current possibilities")
        }

        fn setup_possibilities(&self) {
            // Create new model
            let model = ListStore::new::<PossibilityData>();

            // Get state and set model
            self.possibilities.replace(Some(model));

            // Borrow a fresh model for binding
            let model = self.get_poss();

            // Bind model to listbox
            self.poss_list_box.bind_model(
                Some(&self.get_poss()),
                clone!(
                    #[strong]
                    model,
                    move |item| {
                        // Create possibility widget from PossibilityData
                        let poss = ProceduralPossibility::new(
                            &item
                                .downcast_ref::<PossibilityData>()
                                .expect("Model is of wrong type."),
                        );

                        // Connect remove signal to remove item from model
                        poss.connect_closure(
                            "remove",
                            false,
                            closure_local!(
                                #[weak]
                                model,
                                #[weak]
                                item,
                                move |_poss: ProceduralPossibility| {
                                    model
                                        .remove(model.find(&item).expect("Item not found in model"))
                                }
                            ),
                        );

                        poss.upcast::<gtk::Widget>()
                    }
                ),
            );
        }

        fn build_procedural(&self) -> Result<Procedural> {
            let mut builder = Procedural::builder();
            let name = self.proc_name.text();
            let chance_enabled = self.proc_chance_enabled.is_active();
            // If name is empty, name is None
            if !name.is_empty() {
                builder.name(name.as_str());
            }
            // If chance is enabled, set chance, else None
            if chance_enabled {
                builder.chance(self.proc_chance.value());
            }

            let possibilities = self.get_poss();
            for item in possibilities.into_iter() {
                let poss = item.map(|o| o)?;
                let poss = poss
                    .downcast_ref::<PossibilityData>()
                    .ok_or(anyhow!("Could not get PossibilityData"))?;

                let name = poss.name();
                let name = name
                    .as_deref()
                    .ok_or(anyhow!("Could not get possibility name"))?;
                let name = if name.is_empty() { None } else { Some(name) };

                let chance = if poss.chance_enabled() {
                    Some(poss.chance())
                } else {
                    None
                };

                builder.possibility(name, chance);
            }

            builder.build()
        }

        #[template_callback]
        fn on_proc_clear_btn_clicked(&self) {
            let model = self.get_poss();
            model.remove_all();
        }
        #[template_callback]
        fn on_poss_add_btn_clicked(&self) {
            let poss = PossibilityData::default();
            let model = self.get_poss();
            model.append(&poss);
        }
        #[template_callback]
        fn on_poss_generate_btn_activated(&self) {
            let proc = self.build_procedural();

            match proc {
                Ok(proc) => {
                    if let Err(e) = output_clipboard(&proc.generate_procedural_string()) {
                        self.show_toast(&e.to_string());
                        error!("Could not copy procedural string: {e}");
                    } else {
                        self.show_toast("Copied");
                    }
                }
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_names_generate_btn_activated(&self) {
            let proc = self.build_procedural();

            match proc {
                Ok(proc) => {
                    // proc.generate_possible_names();
                    match proc.generate_possible_names(PossibleFlag::None) {
                        Ok(names) => {
                            if let Err(e) = output_clipboard(&names) {
                                self.show_toast(&e.to_string());
                            } else {
                                self.show_toast("Copied");
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

impl ContentProcedural {}
