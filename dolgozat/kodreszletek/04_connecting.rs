// Mentés event

let editor = self.content.source.buff.clone();
let headerbar = self.header.container.clone();
let save_button = save_button.clone();
actual_button.connect_clicked(move |_| {
    save(&editor, &headerbar, &save_button, &current_file, save_as)
});

// Szerkesztő tartalma megváltozott

let save_button = save_button.clone();
self.content.source.buff.connect_changed(move |editor| {
    if let Some(source_code) = get_buffer(&editor) {
        if let Some(ref current_file) = 
            *current_file.read().unwrap() 
        {
            let has_same_sum = current_file.is_same_as(
                &source_code.as_bytes());
            save_button.set_sensitive(!has_same_sum);
        }
    }
});

// key_events
// open_file

// ConnectedApp

pub struct ConnectedApp(App);

impl ConnectedApp {
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}

// ConnectedApp létrehozása

impl App {
    // ...
    pub fn connect_events(self) -> ConnectedApp {
        let current_file = Arc::new(RwLock::new(None));
        let fullscreen = Arc::new(AtomicBool::new(false));

        let save = &self.header.save;
        let save_as = &self.header.save_as;

        self.editor_changed(current_file.clone(), &save);
        self.open_file(current_file.clone());
        self.save_event(&save, &save, current_file.clone(), false);
        self.save_event(&save, &save_as, current_file.clone(), true);
        self.key_events(current_file, fullscreen);

        ConnectedApp(self)
    }
}

// A main függvény

pub mod cargo;
pub mod state;
pub mod ui;

use ui::App;

fn main() {
    App::new().connect_events().then_execute();
}
