use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Object};

mod imp {
    use super::*;
    use gtk::glib::Properties;
    use std::cell::{Cell, RefCell};

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::PossibilityData)]
    pub struct PossibilityData {
        #[property(get, set)]
        name: RefCell<Option<String>>,
        #[property(get, set, minimum = 0.0, maximum = 100.0)]
        chance: Cell<f64>,
        #[property(get, set)]
        chance_enabled: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PossibilityData {
        const NAME: &'static str = "AETPossibilityData";
        type Type = super::PossibilityData;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PossibilityData {}
}

glib::wrapper! {
    pub struct PossibilityData(ObjectSubclass<imp::PossibilityData>);
}

impl PossibilityData {
    pub fn new(name: String, chance: f64, chance_enabled: bool) -> Self {
        Object::builder()
            .property("name", name)
            .property("chance", chance)
            .property("chance_enabled", chance_enabled)
            .build()
    }
}

impl Default for PossibilityData {
    fn default() -> Self {
        Self::new(String::new(), 100.0, false)
    }
}
