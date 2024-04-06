#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod dom;

use gtk::{prelude::*, Application, ApplicationWindow, ScrolledWindow};
use html5ever::tendril::TendrilSink;

fn main() -> gtk::glib::ExitCode {
    let document = html5ever::parse_document(
        dom::Sink::new(),
        html5ever::ParseOpts::default(),
    )
    .from_utf8()
    .from_file("example.html")
    .unwrap();

    let app = Application::builder()
        .application_id("com.github.johan_mi.gtk_web")
        .build();

    app.connect_activate(move |app| {
        let content = document.render();

        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .child(&ScrolledWindow::builder().child(&content).build())
            .build();

        win.show_all();
    });

    app.run()
}
