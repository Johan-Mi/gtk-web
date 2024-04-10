#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod dom;

use gtk::{
    gio::ActionEntry, glib::clone, prelude::*, Application, ApplicationWindow,
    Entry, InfoBar, ScrolledWindow,
};
use html5ever::tendril::{ByteTendril, TendrilSink};
use std::{cell::RefCell, error::Error, rc::Rc, sync::mpsc};
use url::Url;

fn main() -> gtk::glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.johan_mi.gtk_web")
        .build();

    app.connect_activate(activate);

    app.run()
}

fn activate(app: &Application) {
    let view = ScrolledWindow::builder()
        .expand(true)
        .propagate_natural_width(true)
        .build();

    let info_bar = InfoBar::builder()
        .message_type(gtk::MessageType::Error)
        .build();

    let url_bar = Entry::new();

    let browser = Rc::new(Browser {
        view,
        info_bar,
        url_bar,
        current_url: RefCell::default(),
        frame: std::env::var_os("FRAME").is_some(),
    });

    browser.url_bar.connect_key_press_event(
        clone!(@strong browser => move |url_bar, event| {
            if event.keyval().to_unicode() != Some('\r') {
                return gtk::glib::Propagation::Proceed;
            }
            browser.open(&url_bar.text(), true);
            gtk::glib::Propagation::Stop
        }),
    );

    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("GTK-Web")
        .child(
            &gtk::Grid::builder()
                .orientation(gtk::Orientation::Vertical)
                .child(&browser.url_bar)
                .child(&browser.view)
                .child(&browser.info_bar)
                .build(),
        )
        .build();

    win.connect_key_press_event(clone!(@strong browser => move |_, event| {
        if event.keyval().to_unicode() != Some('\x1b') {
            return gtk::glib::Propagation::Proceed;
        }
        browser.info_bar.hide();
        gtk::glib::Propagation::Stop
    }));

    win.add_action_entries([ActionEntry::builder("select-ui-bar")
        .activate(
            clone!(@strong browser => move |_: &ApplicationWindow, _, _| {
                browser.url_bar.grab_focus();
            }),
        )
        .build()]);
    app.set_accels_for_action("win.select-ui-bar", &["<Ctrl>L"]);

    win.show_all();
    browser.info_bar.hide();
}

struct Browser {
    view: ScrolledWindow,
    info_bar: InfoBar,
    url_bar: Entry,
    current_url: RefCell<Option<Url>>,
    frame: bool,
}

impl Browser {
    fn open(self: &Rc<Self>, url: &str, absolute: bool) {
        if let Err(err) = self.open_impl(url, absolute) {
            for child in self.info_bar.children() {
                self.info_bar.remove(&child);
            }
            self.info_bar
                .set_child(Some(&gtk::Label::new(Some(&err.to_string()))));
            self.info_bar.show_all();
        }
    }

    fn open_impl(
        self: &Rc<Self>,
        url: &str,
        absolute: bool,
    ) -> Result<(), Box<dyn Error>> {
        let joined_url;
        let url = if absolute {
            url
        } else if let Some(current) = &*self.current_url.borrow() {
            joined_url = current.join(url)?.to_string();
            &*joined_url
        } else {
            url
        };

        let parts = mpsc::channel::<Box<[u8]>>();

        let url = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let mut easy = curl::easy::Easy::new();
                    easy.url(url).map_err(|it| it.to_string())?;
                    easy.write_function(move |bytes| {
                        parts.0.send(bytes.into()).unwrap();
                        Ok(bytes.len())
                    })
                    .map_err(|it| it.to_string())?;
                    easy.follow_location(true).map_err(|it| it.to_string())?;
                    easy.perform().map_err(|it| it.to_string())?;
                    Ok::<_, String>(String::from(
                        easy.effective_url()
                            .map_err(|it| it.to_string())?
                            .unwrap(),
                    ))
                })
                .join()
        })
        .unwrap()?;

        let document = html5ever::parse_document(
            dom::Sink::new(),
            html5ever::ParseOpts::default(),
        )
        .from_utf8()
        .from_iter(parts.1.into_iter().map(|it| ByteTendril::from(&*it)));

        let content = document.render(self);
        if let Some(child) = self.view.child() {
            self.view.remove(&child);
        }
        self.view.set_child(Some(&content));
        content.show_all();

        self.url_bar.set_text(&url);
        self.current_url.replace(Url::parse(&url).ok());

        Ok(())
    }
}
