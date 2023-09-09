// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod to_kana;
use crate::to_kana::{romanji2syll, syll2kana, extract_ambigous_kana};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn to_kana_fn(name: &str, convert_type: &str) -> Vec<String> {
    if let Ok(syllables) = romanji2syll(name) {
        if let Ok(syllables_vec) = extract_ambigous_kana(syllables) {
            let mut kana_res: Vec<String> = vec![];
            for syllables in syllables_vec {
                if let Ok(kana) = syll2kana(syllables, convert_type) {
                    kana_res.push(kana.concat());
                }
            } 
            kana_res
        } else {
            vec![String::from("Not found")]
        }
    } else {
        vec![String::from("Not found")]
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![to_kana_fn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
