use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use std::cell::OnceCell;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/preferences.ui")]
    #[properties(wrapper_type = super::AETPreferencesDialog)]
    pub struct AETPreferencesDialog {
        #[property(get, construct_only)]
        pub settings: OnceCell<gtk::Settings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETPreferencesDialog {
        const NAME: &'static str = "AETPreferencesDialog";
        type Type = super::AETPreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETPreferencesDialog {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for AETPreferencesDialog {}
    impl AdwDialogImpl for AETPreferencesDialog {}
    impl PreferencesDialogImpl for AETPreferencesDialog {}
}

glib::wrapper! {
    pub struct AETPreferencesDialog(ObjectSubclass<imp::AETPreferencesDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AETPreferencesDialog {
    pub fn new(settings: gtk::Settings) -> Self {
        glib::Object::builder()
            .property("settings", settings)
            .build()
    }
}
