use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use sourceview5::prelude::*;
use tracing::*;

use crate::tools::description::str_to_description;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/flufflesamy/AlterEgoTools/ui/description.ui")]
    pub struct AETContentDescription {
        #[template_child]
        pub(super) input_text: TemplateChild<sourceview5::View>,
        #[template_child]
        pub(super) output_text: TemplateChild<sourceview5::View>,
        #[template_child]
        input_buffer: TemplateChild<sourceview5::Buffer>,
        #[template_child]
        output_buffer: TemplateChild<sourceview5::Buffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AETContentDescription {
        const NAME: &'static str = "AETContentDescription";
        type Type = super::Description;
        type ParentType = adw::BreakpointBin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AETContentDescription {
        fn constructed(&self) {
            self.parent_constructed();
            self.setup_source_view();
        }
    }

    impl WidgetImpl for AETContentDescription {}

    impl BreakpointBinImpl for AETContentDescription {}

    #[gtk::template_callbacks]
    impl AETContentDescription {
        pub(super) fn show_toast(&self, message: &str) {
            self.obj()
                .activate_action("win.show-toast", Some(&message.to_variant()))
                .map_or_else(|e| error!("Could not show toast: {e}."), |_| ());
        }

        fn setup_source_view(&self) {
            let input_buffer = self.input_buffer.get();
            let output_buffer = self.output_buffer.get();

            // Pick style scheme based on system color scheme
            let scheme_name = if adw::StyleManager::default().is_dark() {
                "Adwaita-dark"
            } else {
                "Adwaita"
            };

            // Set up the source view with Adwaita style scheme
            if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme(scheme_name) {
                input_buffer.set_style_scheme(Some(scheme));
                output_buffer.set_style_scheme(Some(scheme));
            } else {
                debug!("Style scheme not found");
            }

            let language_mananger = sourceview5::LanguageManager::new();

            // Set up input language to markdown
            if let Some(ref language) = language_mananger.language("markdown") {
                input_buffer.set_language(Some(language));
            } else {
                debug!("Language not found");
            }

            // Set up  language to XML
            if let Some(ref language) = language_mananger.language("xml") {
                output_buffer.set_language(Some(language));
            } else {
                debug!("Language not found");
            }
        }

        fn generate_description(&self) {
            let input_buffer = self.input_text.buffer();
            let output_buffer = self.output_text.buffer();
            let input_text = input_buffer
                .text(&input_buffer.start_iter(), &input_buffer.end_iter(), false)
                .to_string();

            let converted = str_to_description(&input_text);
            match converted {
                Ok(res) => {
                    output_buffer.set_text(&res);
                    self.show_toast("Generated description.");
                }
                Err(e) => {
                    error!("Cannot generate description: {e}.");
                    self.show_toast("Error: Cannot generate description.");
                }
            }
        }

        fn output_to_clipboard(&self) {
            let output_buffer = self.output_text.buffer();
            let output_text = output_buffer
                .text(
                    &output_buffer.start_iter(),
                    &output_buffer.end_iter(),
                    false,
                )
                .to_string();
            let display = gdk::Display::default();

            if let Some(display) = display {
                let clipboard = gdk::Display::clipboard(&display);
                gdk::Clipboard::set_text(&clipboard, &output_text);
                self.show_toast("Output copied to clipboard.");
            } else {
                self.show_toast("Error: Could not copy output to clipboard.");
            }
        }

        fn clipboard_to_input(&self) {
            let display = gdk::Display::default();

            if let Some(display) = display {
                let clipboard = gdk::Display::clipboard(&display);
                gdk::Clipboard::read_text_async(
                    &clipboard,
                    gio::Cancellable::NONE,
                    glib::clone!(
                        #[weak(rename_to = desc)]
                        self,
                        move |res| {
                            if let Ok(clipboard) = res {
                                let string = clipboard.unwrap().to_string();
                                desc.clipboard_to_input_done(string);
                            } else {
                                desc.show_toast("Error: Could not read clipboard.");
                            }
                        }
                    ),
                );
            } else {
                self.show_toast("Error: Could not read clipboard.");
            }
        }

        fn clipboard_to_input_done(&self, text: String) {
            let input_buffer = &self.input_text.buffer();
            input_buffer.set_text(&text);
            self.show_toast("Pasted text to input.");
        }

        #[template_callback]
        fn on_import_button_clicked(&self) {
            self.clipboard_to_input();
        }
        #[template_callback]
        fn on_generate_button_clicked(&self) {
            self.generate_description();
        }
        #[template_callback]
        fn on_copy_button_clicked(&self) {
            self.output_to_clipboard();
        }
    }
}

glib::wrapper! {
    pub struct Description(ObjectSubclass<imp::AETContentDescription>)
        @extends gtk::Widget, adw::BreakpointBin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Description {}
