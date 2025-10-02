// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod card_generator;
mod commands;
mod exporters;
mod fake_data;
mod networks;
mod validator;

use card_generator::CreditCardGenerator;
use commands::AppState;
use std::sync::Mutex;

fn main() {
    let generator = CreditCardGenerator::new();
    
    tauri::Builder::default()
        .manage(AppState {
            generator: Mutex::new(generator),
        })
        .invoke_handler(tauri::generate_handler![
            commands::generate_cards,
            commands::validate_card,
            commands::export_cards,
            commands::generate_users,
            commands::generate_lorem,
            commands::get_currencies,
            commands::get_networks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


