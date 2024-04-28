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
    pub mod page3;
    pub mod page4;
    pub mod client_utils;
}

mod parser;

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            client_functions::page1::get_random_bool_func,
            client_functions::page2::get_remind_function,
            client_functions::page3::get_func_from_remainde,
            client_functions::page4::get_name_funcs,
            client_functions::page4::get_random_vector_func,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}