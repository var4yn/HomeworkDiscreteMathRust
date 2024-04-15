// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauriapp::tasks::util;

#[tauri::command]
fn get_boolean_function(n: &str) -> Result<String, &'static str> {
    let Ok(n) = n.parse::<i32>() else {
        return Err("Ошибка парсинга! Ожидалось значение типа i32");
    };
    Ok(tauriapp::tasks::util::BooleanFunction::with_count_args(n).get_func().into())
}

#[tauri::command]
fn get_remind_function(func: &str, n: &str, value: bool) -> Result<String, &'static str> {
    let Ok(n) = n.parse::<i32>() else {
        return Err("Ошибка парсинга! Ожидалось значение типа i32");
    };
    if n == 0 {
        return Err("Аргумент должен быть больше нуля");
    }
    
    let func = util::BooleanFunction::from(func)?;

    if n > func.get_count_args() {
        return Err("Превышено максимальное допустимое значение");
    }

    
    let n = func.get_count_args() - n;

    func.remainde_boolean_function(n, value)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_boolean_function, get_remind_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
