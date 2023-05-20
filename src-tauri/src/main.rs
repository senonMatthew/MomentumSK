#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size, PhysicalSize, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayMenuItem, SystemTrayEvent, Window, RunEvent, WindowEvent};
use MomentumSK::{functsys::{ read_blocklist, get_process_info, add_blockentry, del_blockentry, get_processes }, config::{ write_config_data, get_config_data, read_hyperfocus, read_forceexit }, scan::{persistent_foreground, persistent_scan}, scanner::{foreground_scanner, background_scanner}, sysfunc::{fresh_install, check_identifier}};

#[tokio::main]
async fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let visibility = CustomMenuItem::new("visibility".to_string(), "Hide");
  let info = CustomMenuItem::new("info".to_string(), "You may double click this icon to open the window.").disabled();
  let tray_menu = SystemTrayMenu::new()
    .add_item(info)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit)
    .add_item(visibility);


  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      Ok(())
  })
    .invoke_handler(tauri::generate_handler![print_blocklist, update_bools, listen_for_toggle_change, print_process, get_process, save_entry, remove_entry, start_timer, new_start])
    .plugin(tauri_plugin_positioner::init())
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "visibility" => {
            let window = app.get_window("main").unwrap();
            if window.hide().is_err() {
              window.show().unwrap();
            } else {
              window.hide().unwrap();
            }
          }
          _ => {}
        }
      }
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      RunEvent::ExitRequested { api, .. } => {
          api.prevent_exit();
      }
      RunEvent::WindowEvent {
          label,
          event: WindowEvent::CloseRequested { api, .. },
          ..
      } => {
          let window = _app_handle.get_window("main").unwrap();
          api.prevent_close();
          window.hide().unwrap();
      }
      _ => {}
  });

}


#[tauri::command(async)]
async fn print_blocklist() -> Result<Vec<String>, ()> {
  let list = read_blocklist();
  Ok(list, )
} 

#[tauri::command(async)]
async fn update_bools(categ: &str, params: &str) -> Result<bool, ()> {
  let read_data = get_config_data(categ, params);
  Ok(read_data, )
}

#[tauri::command]
fn listen_for_toggle_change(categ: &str, params: &str, boolean: bool) {
  write_config_data(categ, params, boolean);
  println!("Changed {} to {}.", params, boolean);
}

#[tauri::command(async)]
async fn get_process() -> Result<Vec<String>, ()> {
  let proc = get_process_info();
  Ok(proc, )
}

#[tauri::command(async)]
async fn print_process() -> Result<Vec<String>, ()> {
  let proc = get_processes();
  Ok(proc, )
}

#[tauri::command(async)]
fn save_entry(entry: &str) {
  add_blockentry(entry)
}

#[tauri::command(async)]
fn remove_entry(entries: Vec<&str>) {
  for entry in entries {
    del_blockentry(entry)
  }
}

#[tauri::command(async)]
async fn start_timer(time: u64) -> bool {
  println!("Starting timer!");

  if !read_forceexit() {
    if !read_hyperfocus() {
      // if normal focus (notify)
      foreground_scanner(time+2).await;
      println!("DEBUG: Normal Focus!");
      return false
    } else {
      // if hyper focus (instant minimize)
      foreground_scanner(time+2).await;
      println!("DEBUG: Hyper Focus!");
      return true
    }
  } else {
    // if force exit (persistent force exit)
    background_scanner(time+2).await;
    println!("DEBUG: Force Exit!");
    return true
  }
}

#[tauri::command(async)]
async fn test_timer(duration: u32) -> u32 {
  return 3
}

#[tauri::command]
fn new_start() {
  check_identifier();
}