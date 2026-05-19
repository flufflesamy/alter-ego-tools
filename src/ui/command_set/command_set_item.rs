// SPDX-FileCopyrightText: 2026 Amy Poon <amy@amypoon.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::sync::OnceLock;

use adw::EntryRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::subclass::Signal;
use gtk::glib::{derived_properties, object_subclass};
use gtk::{glib, template_callbacks};

use crate::ui::command_set::CommandSetData;

mod imp {

    use super::*;

    #[derive(Default, Debug, glib::Properties, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/command_set_item.ui")]
    #[properties(wrapper_type = super::ContentCommandSetItem)]
    pub struct AETContentCommandSetItem {
        #[property(get, construct_only)]
        pub data: RefCell<Option<CommandSetData>>,
        #[property(get, construct_only)]
        title: RefCell<Option<String>>,
        #[template_child]
        text: TemplateChild<EntryRow>,
    }

    #[object_subclass]
    impl ObjectSubclass for AETContentCommandSetItem {
        const NAME: &'static str = "AETContentCommandSetItem";
        type Type = super::ContentCommandSetItem;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[derived_properties]
    impl ObjectImpl for AETContentCommandSetItem {
        fn constructed(&self) {
            self.parent_constructed();

            // Bind data to controls
            let data = self.data.borrow().as_ref().cloned().expect("data is None");
            let text = self.text.get();
            data.bind_property("data", &text, "text")
                .bidirectional()
                .build();

            // Set title
            let title = self.title.borrow().as_ref().cloned().unwrap_or_default();
            text.set_title(&title);
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("remove").build()])
        }
    }

    impl WidgetImpl for AETContentCommandSetItem {}

    impl ListBoxRowImpl for AETContentCommandSetItem {}

    #[template_callbacks]
    impl AETContentCommandSetItem {
        #[template_callback]
        fn on_remove_btn_clicked(&self) {
            // Emit remove signal
            self.obj().emit_by_name::<()>("remove", &[]);
        }
    }
}

glib::wrapper! {
    pub struct ContentCommandSetItem(ObjectSubclass<imp::AETContentCommandSetItem>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ContentCommandSetItem {
    pub fn new(data: &CommandSetData, title: &str) -> Self {
        glib::Object::builder()
            .property("data", data)
            .property("title", title)
            .build()
    }
}
