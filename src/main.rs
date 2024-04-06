#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod dom;

use gtk::{
    glib::clone, prelude::*, Application, ApplicationWindow, Entry,
    ScrolledWindow,
};
use html5ever::tendril::{ByteTendril, TendrilSink};
use std::{rc::Rc, sync::mpsc};

fn main() -> gtk::glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.johan_mi.gtk_web")
        .build();

    app.connect_activate(activate);

    app.run()
}

fn activate(app: &Application) {
    let view = Rc::new(ScrolledWindow::builder().expand(true).build());

    let url_bar = Entry::new();
    url_bar.connect_key_press_event(
        clone!(@strong view => move |url_bar, event| {
            if event.keyval().to_unicode() != Some('\r') {
                return gtk::glib::Propagation::Proceed;
            }
            open(&url_bar.text(), &view);
            gtk::glib::Propagation::Stop
        }),
    );

    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hello, World!")
        .child(
            &gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .child(&url_bar)
                .child(&*view)
                .build(),
        )
        .build();

    win.show_all();
}

fn open(url: &str, view: &ScrolledWindow) {
    let parts = mpsc::channel::<Box<[u8]>>();

    std::thread::scope(|scope| {
        scope.spawn(|| {
            let mut easy = curl::easy::Easy::new();
            easy.url(url).unwrap();
            easy.write_function(move |bytes| {
                parts.0.send(bytes.into()).unwrap();
                Ok(bytes.len())
            })
            .unwrap();
            easy.perform().unwrap();
        });
    });

    let document = html5ever::parse_document(
        dom::Sink::new(),
        html5ever::ParseOpts::default(),
    )
    .from_utf8()
    .from_iter(parts.1.into_iter().map(|it| ByteTendril::from(&*it)));

    let content = document.render();
    view.set_child(Some(&content));
    content.show_all();
}
