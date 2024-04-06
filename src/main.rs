#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod sink;

use gtk::{
    prelude::*, Align, Application, ApplicationWindow, Label, Orientation,
    ScrolledWindow,
};
use html5ever::tendril::TendrilSink;

fn main() -> gtk::glib::ExitCode {
    let sink = html5ever::parse_document(
        sink::Sink::default(),
        html5ever::ParseOpts::default(),
    )
    .from_utf8()
    .from_file("example.html")
    .unwrap();

    let app = Application::builder()
        .application_id("com.github.johan_mi.gtk_web")
        .build();

    app.connect_activate(move |app| {
        let mut content = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Start);

        for name in sink.names.values() {
            content = content.child(&Label::new(Some(&*name.local)));
        }

        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .child(&ScrolledWindow::builder().child(&content.build()).build())
            .build();

        win.show_all();
    });

    app.run()
}
