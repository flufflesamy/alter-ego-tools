use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use tracing::*;

use crate::tools::description::str_to_description;
use crate::utils::macros::*;
use crate::utils::*;

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
        fn setup_source_view(&self) {
            let manager = adw::StyleManager::default();
            let input_buffer = self.input_buffer.get();
            let output_buffer = self.output_buffer.get();

            buffer_color(&manager, &input_buffer);
            buffer_color(&manager, &output_buffer);

            buffer_language(&input_buffer, "markdown");
            buffer_language(&output_buffer, "xml");
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
                    toast!(self.obj(), "Generated description.");
                }
                Err(e) => {
                    toast_error!(self.obj(), "Cannot generate description", e);
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

            match output_clipboard(&output_text) {
                Ok(()) => toast!(self.obj(), "Output copied to clipboard."),
                Err(e) => toast_error!(self.obj(), "Could not copy output to clipboard", e),
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
                            match res {
                                Ok(clipboard) => {
                                    let string = clipboard.unwrap().to_string();
                                    desc.clipboard_to_input_done(string);
                                }
                                Err(e) => {
                                    toast_error!(desc.obj(), "Could not read clipboard", e);
                                }
                            }
                        }
                    ),
                );
            } else {
                toast!(self.obj(), "Error: Could not read clipboard.");
            }
        }

        fn clipboard_to_input_done(&self, text: String) {
            let input_buffer = &self.input_text.buffer();
            input_buffer.set_text(&text);
            toast!(self.obj(), "Pasted text to input.");
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
