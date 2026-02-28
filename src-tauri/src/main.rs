mod protocol;
mod router;
mod runners;
mod utils;

use protocol::handler::handle_protocol;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            println!("VibeCode Router started");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![handle_protocol])
        .run(tauri::generate_context!())
        .expect("error running vibecode router");
}
