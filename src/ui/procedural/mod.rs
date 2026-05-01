mod possibility;
mod possibility_data;

// Re-exports for parent module
pub(crate) use possibility::ProceduralPossibility;
pub(crate) use possibility_data::PossibilityData;

use std::cell::RefCell;

use anyhow::{Result, bail};
use tracing::*;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ButtonRow, ComboRow, EntryRow, SpinRow, SwitchRow};
use gtk::{
    Button, ListBox, StringObject,
    gio::ListStore,
    glib::{self, clone, closure_local},
};
use sourceview5::prelude::*;

use crate::tools::procedural::*;
use crate::utils::macros::ok_or;
use crate::utils::output_clipboard;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/procedural.ui")]
    pub struct AETContentProcedural {
        possibilities: RefCell<Option<ListStore>>,
        #[template_child]
        proc_poss_add_btn: TemplateChild<ButtonRow>,
        #[template_child]
        proc_clear_btn: TemplateChild<Button>,
        #[template_child]
        proc_name: TemplateChild<EntryRow>,
        #[template_child]
        proc_chance_enabled: TemplateChild<SwitchRow>,
        #[template_child]
        proc_chance: TemplateChild<SpinRow>,
        #[template_child]
        proc_stat: TemplateChild<ComboRow>,
        #[template_child]
        poss_list_box: TemplateChild<ListBox>,
        #[template_child]
        poss_pattern: TemplateChild<EntryRow>,
        #[template_child]
        poss_flag: TemplateChild<ComboRow>,
        #[template_child]
        poss_generate_btn: TemplateChild<ButtonRow>,
        #[template_child]
        names_generate_btn: TemplateChild<ButtonRow>,
        #[template_child]
        containing_phrases_generate_btn: TemplateChild<ButtonRow>,
        #[template_child]
        source_view: TemplateChild<sourceview5::View>,
        #[template_child]
        source_buffer: TemplateChild<sourceview5::Buffer>,
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
            self.setup_source_view();
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

        fn setup_source_view(&self) {
            let buffer = self.source_buffer.get();

            // Pick style scheme based on system color scheme
            let scheme_name = if adw::StyleManager::default().is_dark() {
                "Adwaita-dark"
            } else {
                "Adwaita"
            };

            // Set up the source view with Adwaita style scheme
            if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme(scheme_name) {
                buffer.set_style_scheme(Some(&scheme));
            } else {
                debug!("Style scheme not found");
            }

            // Set up language to XML
            if let Some(ref language) = sourceview5::LanguageManager::new().language("xml") {
                buffer.set_language(Some(&language));
            } else {
                debug!("Language not found");
            }
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

            let stat = ok_or!(
                self.proc_stat
                    .selected_item()
                    .and_downcast::<StringObject>(),
                "Could not get selected stat"
            )?
            .string();

            if let Some(stat) = Stat::from_name(stat.as_str()) {
                builder.stat(stat);
            }

            let possibilities = self.get_poss();
            for item in possibilities.into_iter() {
                let poss = item.map(|o| o)?;
                let poss = ok_or!(
                    poss.downcast_ref::<PossibilityData>(),
                    "Could not get PossibilityData"
                )?;

                let name = poss.name();
                let name = ok_or!(name.as_deref(), "Could not get possibility name")?;
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

        fn generate_procedural(&self) -> Result<String> {
            let proc = self.build_procedural()?;
            Ok(proc.generate_procedural_string())
        }

        fn generate_possible_names(&self) -> Result<String> {
            let proc = self.build_procedural()?;
            proc.generate_possible_names(self.get_possible_flag()?)
        }

        fn generate_possible_phrases(&self) -> Result<String> {
            let proc = self.build_procedural()?;

            let pattern = self.poss_pattern.text();
            if pattern.is_empty() {
                bail!("A pattern must be provided")
            };

            proc.generate_possible_containing_phrases(pattern.as_str(), self.get_possible_flag()?)
        }

        fn get_possible_flag(&self) -> Result<PossibleFlag> {
            let flag = ok_or!(
                self.poss_flag
                    .selected_item()
                    .and_downcast::<StringObject>(),
                "Could not get selected flag"
            )?;

            Ok(PossibleFlag::from(flag.string().as_str()))
        }

        #[template_callback]
        fn on_proc_clear_btn_clicked(&self) {
            let model = self.get_poss();
            model.remove_all();
            self.proc_name.set_text("");
            self.proc_chance_enabled.set_active(false);
            self.proc_chance.set_value(100.0);
            self.proc_stat.set_selected(0);
            self.poss_pattern.set_text("");
            self.poss_flag.set_selected(0);
        }
        #[template_callback]
        fn on_poss_add_btn_activated(&self) {
            let poss = PossibilityData::default();
            let model = self.get_poss();
            model.append(&poss);
        }
        #[template_callback]
        fn on_poss_generate_btn_activated(&self) {
            let procedural = self.generate_procedural();
            match procedural {
                Ok(procedural) => {
                    self.source_buffer.set_text(&procedural);
                }
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_names_generate_btn_activated(&self) {
            let names = self.generate_possible_names();
            match names {
                Ok(names) => {
                    self.source_buffer.set_text(&names);
                }
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_containing_phrases_generate_btn_activated(&self) {
            let phrases = self.generate_possible_phrases();

            match phrases {
                Ok(p) => self.source_buffer.set_text(&p),
                Err(e) => {
                    self.show_toast(&e.to_string());
                }
            }
        }
        #[template_callback]
        fn on_poss_copy_btn_clicked(&self) {
            let buffer = self.source_buffer.get();
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            match output_clipboard(text.as_str()) {
                Ok(_) => self.show_toast("Copied"),
                Err(_) => self.show_toast("Could not copy to clipboard"),
            }
        }
    }
}

glib::wrapper! {
    pub struct ContentProcedural(ObjectSubclass<imp::AETContentProcedural>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ContentProcedural {}
