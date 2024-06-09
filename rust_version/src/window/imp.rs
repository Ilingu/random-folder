use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::widget::{
    CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl,
};
use gtk::subclass::window::WindowImpl;
use gtk::{glib, prelude::*};
use gtk::{
    glib::subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    Button, CompositeTemplate, TemplateChild,
};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/templates/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Connect to "clicked" signal of `button`
        self.button.connect_clicked(move |button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        });
    }
}

// necessary implementation
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
