// State

use std::path::{Path, PathBuf};
use tiny_keccak::keccak512;

pub struct ActiveMetadata {
    path: PathBuf,
    sum: [u8; 64],
}

impl ActiveMetadata {
    pub fn new(path: PathBuf, data: &[u8]) -> ActiveMetadata {
        ActiveMetadata { path, sum: keccak512(data), }
    }

    pub fn get_path(&self) -> &Path {&self.path}

    pub fn get_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|p| p.to_path_buf())
    }

    pub fn is_same_as(&self, data: &[u8]) -> bool {
        &keccak512(data)[..] == &self.sum[..]
    }

    pub fn set_sum(&mut self, data: &[u8]) {
        self.sum = keccak512(data);
    }
}

// Dialogok

use gtk::*;
use std::path::PathBuf;

pub struct OpenDialog(FileChooserDialog);
pub struct SaveDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new(
            Some("Megnyitas"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        open_dialog.add_button("Megse", ResponseType::Cancel.into());
        open_dialog.add_button("Megnyit", ResponseType::Ok.into());

        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

// Save

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) 
    -> io::Result<SaveAction> 
{
    if let Some(path) = path {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&new_path)?;
        file.write_all(data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}

pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        let result = if save_as {
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), 
                text.as_bytes())
        };

        match result {
            Ok(SaveAction::New(file)) => {
                set_title(&headerbar, file.get_path());
                if let Some(parent) = file.get_dir() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                };

                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }
            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = 
                    *current_file.write().unwrap() 
                {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }
            _ => (),
        }
    }
}

// If let és árnyékolás magyarázat

match path {
    Some(letezik) => {
        let kicsomagolt_letezik = letezik.unwrap();
        // ...
    },
    _ => (),
}

let Some(letezik) = path {
    // ...
}

let Some(path) = path {
    // ...
}