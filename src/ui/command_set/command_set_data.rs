// SPDX-FileCopyrightText: 2026 Amy Poon <amy@amypoon.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Object, Properties};

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::CommandSetData)]
    pub struct CommandSetData {
        #[property(get, set)]
        data: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CommandSetData {
        const NAME: &'static str = "AETCommandSetData";
        type Type = super::CommandSetData;
    }

    #[glib::derived_properties]
    impl ObjectImpl for CommandSetData {}
}

glib::wrapper! {
    pub struct CommandSetData(ObjectSubclass<imp::CommandSetData>);
}

impl CommandSetData {
    pub fn new(data: String) -> Self {
        Object::builder().property("data", data).build()
    }
}

impl Default for CommandSetData {
    fn default() -> Self {
        Self::new(String::new())
    }
}
