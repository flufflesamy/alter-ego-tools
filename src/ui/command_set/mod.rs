// SPDX-FileCopyrightText: 2026 Amy Poon <amy@amypoon.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod command_set_data;
mod command_set_item;

use std::cell::RefCell;

use adw::EntryRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use anyhow::Result;
pub(crate) use command_set_data::CommandSetData;
pub(crate) use command_set_item::ContentCommandSetItem;
use gtk::gio::ListStore;
use gtk::glib::{clone, closure_local};
use gtk::{ListBox, glib};

use crate::tools::command_set::*;
use crate::utils::macros::*;
use crate::utils::*;

mod imp {

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/command_set.ui")]
    pub struct AETContentCommandSet {
        outcomes: RefCell<Option<ListStore>>,
        solved_commands: RefCell<Option<ListStore>>,
        unsolved_commands: RefCell<Option<ListStore>>,
        #[template_child]
        capture_string: TemplateChild<EntryRow>,
        #[template_child]
        outcomes_list: TemplateChild<ListBox>,
        #[template_child]
        solved_commands_list: TemplateChild<ListBox>,
        #[template_child]
        unsolved_commands_list: TemplateChild<ListBox>,
        #[template_child]
        pub(crate) source_view: TemplateChild<sourceview5::View>,
        #[template_child]
        source_buffer: TemplateChild<sourceview5::Buffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETContentCommandSet {
        const NAME: &'static str = "AETContentCommandSet";
        type Type = super::ContentCommandSet;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETContentCommandSet {
        fn constructed(&self) {
            self.parent_constructed();

            // Setup listboxes
            self.setup_list(&self.outcomes, &self.outcomes_list, "Outcome".to_owned());
            self.setup_list(
                &self.solved_commands,
                &self.solved_commands_list,
                "Solved Command".to_owned(),
            );
            self.setup_list(
                &self.unsolved_commands,
                &self.unsolved_commands_list,
                "Unsolved Command".to_owned(),
            );

            self.setup_source_view();
        }
    }

    impl WidgetImpl for AETContentCommandSet {}
    impl BreakpointBinImpl for AETContentCommandSet {}

    #[gtk::template_callbacks]
    impl AETContentCommandSet {
        fn get_list(&self, list: &RefCell<Option<ListStore>>) -> ListStore {
            list.borrow().clone().expect("Can't get list")
        }

        fn setup_list(
            &self,
            list: &RefCell<Option<ListStore>>,
            list_box: &TemplateChild<ListBox>,
            name: String,
        ) {
            // Create new model
            let model = ListStore::new::<CommandSetData>();

            // Set listbox model
            list.replace(Some(model));

            // Borrow listbox model
            let model = self.get_list(list);

            // Bind model to listbox
            list_box.bind_model(
                Some(&model),
                clone!(
                    #[strong]
                    model,
                    #[strong]
                    name,
                    move |data| {
                        // Create item widget from data
                        let item = ContentCommandSetItem::new(
                            data.downcast_ref::<CommandSetData>()
                                .expect("Model is of wrong type."),
                            &name,
                        );

                        // Connect remove signal to remove model item
                        item.connect_closure(
                            "remove",
                            false,
                            closure_local!(
                                #[weak]
                                model,
                                #[weak]
                                data,
                                move |_item: ContentCommandSetItem| {
                                    model
                                        .remove(model.find(&data).expect("Item not found in model"))
                                }
                            ),
                        );

                        item.upcast::<gtk::Widget>()
                    }
                ),
            );
        }

        fn setup_source_view(&self) {
            let manager = adw::StyleManager::default();
            let buffer = self.source_buffer.get();

            // Set buffer to match system theme
            buffer_color(&manager, &buffer);

            // TODO: Write custom lang definition for command sets
            buffer_language(&buffer, "possnames");
        }

        fn build_command_set(&self) -> Result<CommandSet> {
            let mut builder = CommandSet::builder();

            builder.capture_string(self.capture_string.text().into());

            let outcomes = self.get_list(&self.outcomes);
            let solved_commands = self.get_list(&self.solved_commands);
            let unsolved_commands = self.get_list(&self.unsolved_commands);

            for outcome in outcomes.into_iter() {
                let object = outcome?;
                let data = ok_or!(
                    object.downcast_ref::<CommandSetData>(),
                    "Could not get CommandSetData"
                )?
                .data();

                builder.outcome(ok_or!(data, "Could not get outcome")?);
            }

            for solved in solved_commands.into_iter() {
                let object = solved?;
                let data = ok_or!(
                    object.downcast_ref::<CommandSetData>(),
                    "Could not get CommandSetData"
                )?
                .data();

                builder.solved_command(ok_or!(data, "Could not get solved command")?);
            }

            for unsolved in unsolved_commands.into_iter() {
                let object = unsolved?;
                let data = ok_or!(
                    object.downcast_ref::<CommandSetData>(),
                    "Could not get CommandSetData"
                )?
                .data();

                builder.unsolved_command(ok_or!(data, "Could not get unsolved command")?);
            }

            builder.build()
        }

        fn generate(&self) -> Result<String> {
            let command_set = self.build_command_set()?;
            command_set.generate()
        }

        #[template_callback]
        fn on_clear_btn_clicked(&self) {
            // Remove items from list boxes
            let outcomes = self.get_list(&self.outcomes);
            let solved_commands = self.get_list(&self.solved_commands);
            let unsolved_commands = self.get_list(&self.unsolved_commands);
            outcomes.remove_all();
            solved_commands.remove_all();
            unsolved_commands.remove_all();

            // Clear capture string
            self.capture_string.set_text("");
        }

        #[template_callback]
        fn on_outcomes_add_btn_activated(&self) {
            let data = CommandSetData::default();
            let model = self.get_list(&self.outcomes);
            model.append(&data);
        }

        #[template_callback]
        fn on_solved_commands_add_btn_activated(&self) {
            let data = CommandSetData::default();
            let model = self.get_list(&self.solved_commands);
            model.append(&data);
        }

        #[template_callback]
        fn on_unsolved_commands_add_btn_activated(&self) {
            let data = CommandSetData::default();
            let model = self.get_list(&self.unsolved_commands);
            model.append(&data);
        }

        #[template_callback]
        fn on_generate_btn_activated(&self) {
            let generated = self.generate();
            match generated {
                Ok(text) => self.source_buffer.set_text(&text),
                Err(e) => {
                    toast_error!(self.obj(), "Error:", e)
                }
            }
        }

        #[template_callback]
        fn on_copy_btn_clicked(&self) {
            let buffer = self.source_buffer.get();
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            match output_clipboard(&text) {
                Ok(_) => toast!(self.obj(), "Copied"),
                Err(e) => toast_error!(self.obj(), "Could not copy to clipboard:", e),
            }
        }
    }
}

glib::wrapper! {
    pub struct ContentCommandSet(ObjectSubclass<imp::AETContentCommandSet>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
