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