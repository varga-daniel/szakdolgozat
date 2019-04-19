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