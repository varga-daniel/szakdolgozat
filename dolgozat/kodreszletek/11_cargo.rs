// Menük behozatala a programba

pub struct FileMenu {
    pub file_menu_item: MenuItem,
    pub file_menu: Menu,
    pub new_file_item: MenuItem,
    pub open_item: MenuItem,
    pub save_item: MenuItem,
    pub save_as_item: MenuItem,
    pub close_file_item: MenuItem,
    pub quit_item: MenuItem,
}

// A Header átveszi a HeaderBar szerepét, és inkább ő maga változtatja azt.
// Ezért kap egy Arc referenciát a jelenlegi fájlra (és projektre).
// Illetve megkapja az új menüket.

pub struct Header {
    pub container: HeaderBar,
    pub file_menu: FileMenu,
    pub cargo_menu: CargoMenu,

    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
}

// A Header saját maga változtatja a HeaderBar címét.
// Nem tartalmazhattuk a same_sum boolt a Header-ben, mivel akkor
// a kódszerkesztőnek mutálható referenciát kellett volna adnunk a Header-hez.

pub fn update_titles(&self, same_sum: bool) {
    // ...
}

// Új dialog-ok.
// Csak annyiban különböznek, hogy hogyan hívjuk meg a belső FileChooserDialog-ot.
// A fájlos dialogok kaptak fájlfiltert ".rs" kiterjesztésű fájlokra.

pub struct OpenFolderDialog(FileChooserDialog);
pub struct CreateFolderDialog(FileChooserDialog);

// A state.rs-be bekerült a ProjectMetadata.
// Mivel ezt nem módosítjuk úgy, mint a bent tárolt fájlt (csak átpasszoljuk a cargo parancsoknak),
// ezért itt nem kell annyi mindent ellenőriznünk, csak a nevét és az elérési helyét kérjük.

pub struct ProjectMetadata {
    path: PathBuf,
}

// cargo.rs létrehozva, benne cargo parancsmeghívások.

pub fn check_cargo_project(location: &Path) 
    -> std::io::Result<process::Output> 
{
    process::Command::new("cargo")
            .current_dir(location)
            .args(&["check", "--message-format", "short"])
            .output()
}

// misc.rs nagyban megváltozott, az app.rs-ből néhány metódus ide lett mozgatva,
// illetve kibővült a projektkezelési metódusokkal is.

pub fn open_file(
    editor: &Buffer, 
    current_file: &RwLock<Option<ActiveMetadata>>) {
    // ...
}

pub fn close_file(
    window: &Window,
    editor: &Buffer,
    save_item: &MenuItem,
    current_file: &RwLock<Option<ActiveMetadata>>) {
    // ...
}

pub fn ask_about_unsaved_changes(parent: &Window) -> i32 {
    // ...
}

pub fn open_project(
    parent: &Window, 
    current_project: &RwLock<Option<ProjectMetadata>>) {
    // ...
}

pub fn create_project(
    parent: &Window,
    current_project: &RwLock<Option<ProjectMetadata>>,
    is_binary: bool) {
    // ...
}

pub enum CargoAction {
    Build, Run, Check, Clean, Test,
}

pub fn perform_cargo_action(
    parent: &Window,
    current_project: &RwLock<Option<ProjectMetadata>>,
    action: CargoAction) {
    // ...
}

// Az App megkapta adattagként a Metadata struktúrákat könnyebb kód-átláthatóságért,
// illetve a Header-t becsomagoltuk egy Arc-ba, hogy azt átadhassuk a különféle
// metódusoknak, hogy tudják rákérni a címfrissítésre.

pub struct App {
    pub window: Window,
    pub header: Arc<Header>,
    pub content: Content,

    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    current_project: Arc<RwLock<Option<ProjectMetadata>>>,
}

// Kibővültek a gombkombinációk:
// - F11: teljes képernyő
// - Ctrl+S: Mentés
// - Ctrl+O: Megnyitás
// - Ctrl+W: Jelenlegi fájl bezárása
// - Ctrl+Q: Program bezárása

// A Header cím kap egy csillagot, ha a háttértáron lévő fájl
// eltér a pufferben lévőtől, illetve ha a jelenlegi fájl még nincs mentve.

// Fájlfilter

let filter = FileFilter::new();
filter.add_pattern("*.rs");
filter.set_name("Rust forrasfajlok");

open_dialog.add_filter(&filter);