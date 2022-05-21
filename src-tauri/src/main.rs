use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

struct AppState {
    src_file: Mutex<Option<String>>,
}

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
        .invoke_handler(tauri::generate_handler![pick_file])
        .manage(AppState::new());

    b.run(tauri::generate_context!())
        .expect("error while running tauri application");
    // let app = b.build(tauri::generate_context!()).unwrap();
    // app.run(|_app_handle, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         api.prevent_exit();
    //     }
    //     // tauri::RunEvent::WindowEvent {label, event, ..} =>{
    //     //     match label.into() {
    //     //         "pick_file" =>{
    //     //             println!("pick file");
    //     //             let dialog = tauri::api::dialog::FileDialogBuilder::default();
    //     //             dialog.pick_file(|file_path| {
    //     //                 println!("{:?}", file_path);
    //     //                 // tauri::api::ipc
    //     //             })
    //     //         }
    //     //     }
    //     // }
    //     _ => {}
    // });
}

#[tauri::command]
fn pick_file(state: State<AppState>, app_handle: tauri::AppHandle) {
    //state.src_file = None.clone();

    let dialog = tauri::api::dialog::FileDialogBuilder::default();
    dialog.pick_file(move |file_path| {
        println!("{:?}", file_path);
        app_handle
            .emit_all("draw_src", file_path.clone())
            .expect("mensaje no mandado");
        // state.src_file = file_path;
        *(app_handle
            .state::<AppState>()
            .src_file
            .lock()
            .as_deref_mut()
            .unwrap()) = Some(file_path.unwrap().to_str().unwrap().into());
    });
}
