extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Box, Button, Orientation, TextView};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let box1 = Box::new(Orientation::Vertical, 8);
        window.add(&box1);

        let text = TextView::new();
        box1.pack_start(&text, true, true, 8);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        box1.pack_start(&button, true, true, 8);

        window.show_all();
    });

    application.run(&[]);
}
