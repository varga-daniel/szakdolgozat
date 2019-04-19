// UI MODULE
// Az Appot épp most tesszük bele

mod app;
mod source_view;

pub use self::app::App;
pub use self::source_view::Content;

// APP
// Content épp most adódik

use super::Content;
use gtk;
use gtk::*;

pub struct App {
    pub window: Window,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("A GTK inicializacioja megbukott!");
            process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let content = Content::new();

        window.set_title("Rozsda IDE");
        window.set_default_size(800, 600);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, content }
    }
}

// Header hozzáadása

use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub save: Button,
    pub save_as: Button,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title("Rozsda IDE");
        container.set_show_close_button(true);

        let open = Button::new_with_mnemonic("Megnyitas");
        let save = Button::new_with_mnemonic("Mentes");
        let save_as = Button::new_with_mnemonic("Mentes Maskent");

        container.pack_start(&open);
        container.pack_end(&save_as);
        container.pack_end(&save);

        Header { container, open, save, save_as }
    }
}

// App, a header hozzáadása után

use super::{Content, Header}; // <---
use gtk;
use gtk::*;

// ...

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new(); // <---
        let content = Content::new();

        window.set_titlebar(&header.container); // <---
        window.set_title("Rozsda IDE");
        window.set_default_size(800, 600);

// ui.rs Header után

mod app;
mod source_view;
mod header; // <---

pub use self::app::App;
pub use self::source_view::Content;
pub use self::header::Header; // <---

// Sourceview eleje

use gtk::*;
use pango::*;
use sourceview::*;

pub struct Source {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
}

pub struct Content {
    pub container: Paned,
    pub source: Source,
}

impl Source {
    pub fn new() -> Source {
        let buff = Buffer::new(gtk::NONE_TEXT_TAG_TABLE);
        let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(
            gtk::NONE_ADJUSTMENT, NONE_ADJUSTMENT);

        container.add(&view);

        Source::configure(&view, &buff);

        Source {container, buff, view}
    }

    fn configure(view: &View, buff: &Buffer) {
        WidgetExt::override_font(view, 
            &FontDescription::from_string("monospace"));

        LanguageManager::new()
            .get_language("rust")
            .map(|rust| buff.set_language(&rust));

        // ...

        view.set_input_hints(InputHints::SPELLCHECK | 
            InputHints::WORD_COMPLETION);
    }
}

// Content hozzáadása az apphoz

        window.add(&content.container);