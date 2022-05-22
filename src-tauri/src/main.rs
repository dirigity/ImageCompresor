use image::{io::Reader as ImageReader, DynamicImage};
use std::fs;
use std::sync::Mutex;
use tauri::Manager;

enum Tiles {
    SolidSquare,
    SolidCircle,
}

impl Tiles {
    fn getPixel(&self, x: f64, y: f64) -> bool {
      

        match self {
            Tiles::SolidSquare => true,
            Tiles::SolidCircle => x.powi(2) + y.powi(2) < 1.,
        }
    }
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct Tile {
    tile_code: Tiles,
    color: Color,
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
    rotation: f32,
}

impl Tile{
    fn getPixel(&self, gloval_x: f64, gloval_y: f64)->bool{
          let x = (gloval_x - self.center_x) / self.width;
        let y = (gloval_y - self.center_y);
    }
}

struct EncodedImg {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    background: Color,
}

impl EncodedImg {
    fn getPixel(&self, x: usize, y: usize) -> Result<Color, ()> {
        if x < 0 || x > self.width || y < 0 || y > self.height {
            Err(())
        } else {
            Ok(match self.tiles.iter().find(|t| t.getPixel(x, y)) {
                None => self.background,
                Some(c) => c,
            })
        }
    }
}

struct AppState {
    src_file: Mutex<Option<DynamicImage>>,
}

const WORKSPACE_PATH: &str = "./encoder_workspace/";
const SRC_PATH: &str = "./encoder_workspace/src.png";
const RES_PATH: &str = "./encoder_workspace/res.png";

impl AppState {
    fn new() -> AppState {
        AppState {
            src_file: Mutex::new(None),
        }
    }
}

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
fn main() {
    println!("main starts");

    // let state = AppState::new();

    let b = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pick_file, encode])
        .manage(AppState::new());

    b.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn pick_file(app_handle: tauri::AppHandle) {
    let dialog = tauri::api::dialog::FileDialogBuilder::default();
    dialog.pick_file(move |file_path| match file_path {
        None => {
            println!("no file pocken");
        }
        Some(path) => {
            println!("dialog return: {:?}", path);
            let img = ImageReader::open(path.clone()).unwrap().decode().unwrap();
            println!("image readed");

            fs::remove_dir_all(WORKSPACE_PATH).unwrap();
            fs::create_dir_all(WORKSPACE_PATH).unwrap();
            println!("workspace reseted");

            let save_ret = img.save(SRC_PATH);
            println!("saving return: {:?}", save_ret);

            *(app_handle
                .state::<AppState>()
                .src_file
                .lock()
                .as_deref_mut()
                .unwrap()) = Some(img);

            app_handle
                .emit_all("draw_src", SRC_PATH)
                .expect("mensaje draw_src no mandado");
        }
    });
}

#[tauri::command]
fn encode(app_handle: tauri::AppHandle) {
    update_dst(&app_handle);
}

fn update_dst(app_handle: &tauri::AppHandle) {
    app_handle
        .emit_all("draw_res", RES_PATH.clone())
        .expect("mensaje draw_res no mandado");
}
