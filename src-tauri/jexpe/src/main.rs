use std::collections::HashMap;
use tokio::sync::Mutex;

use pty::{PtyProcess};

mod shell;


pub struct JexpeState {
    ptys: Mutex<HashMap<String, PtyProcess>>,
}

impl JexpeState {
    fn new() -> Self {
        Self {
            ptys: Mutex::new(HashMap::new()),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(JexpeState::new())
        .invoke_handler(tauri::generate_handler![
            shell::commands::get_os_shells,
            shell::commands::spawn_shell,
            shell::commands::write_shell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running jexpe application");
}
