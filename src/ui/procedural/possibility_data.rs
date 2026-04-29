use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Object};

// #[derive(Default, Debug)]
// pub struct PossibilityData {
//     pub name: String,
//     pub chance: f64,
// }

mod imp {
    use super::*;
    use gtk::glib::Properties;
    use std::cell::{Cell, RefCell};

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::PossibilityData)]
    pub struct PossibilityData {
        // #[property(name = "name", get, set, type = String, member = name)]
        // #[property(name = "chance", get, set, type = f64, member = chance)]
        // pub data: RefCell<PossibilityData>,
        #[property(get, set)]
        name: RefCell<Option<String>>,
        #[property(get, set, minimum = 0.0, maximum = 100.0)]
        chance: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PossibilityData {
        const NAME: &'static str = "AETPossibilityObject";
        type Type = super::PossibilityData;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PossibilityData {}
}

glib::wrapper! {
    pub struct PossibilityData(ObjectSubclass<imp::PossibilityData>);
}

impl PossibilityData {
    pub fn new(name: String, chance: f64) -> Self {
        Object::builder()
            .property("name", name)
            .property("chance", chance)
            .build()
    }
}
