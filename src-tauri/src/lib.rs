mod tasks {
    pub mod util;
    pub mod task3;
    pub mod task4;
    pub mod task5;
    pub mod task6and7;
    pub mod task8and9;
    pub mod task10and11;
    pub mod task12;
}

mod client_functions {
    pub mod page1;
    pub mod page2;
}

mod parser;

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            client_functions::page1::get_random_bool_func,
            client_functions::page2::get_remind_function,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}