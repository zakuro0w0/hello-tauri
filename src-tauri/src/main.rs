#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// tauri::commandを付けることでUI側に公開するコマンドとなる
#[tauri::command]
fn counter(count: i32) -> String {
    let x = unsafe { hello_world_cpp() };
    format!("current count = {}", count + x)
}

// C/C++で実装した関数を宣言する(名前が一致していること)
extern "C" {
    // C++で実装した関数
    // C++のintはi32に相当する
    fn hello_world_cpp() -> i32;
}

fn main() {
    tauri::Builder::default()
        // UIに公開したい関数はここで登録しておく必要がある
        .invoke_handler(tauri::generate_handler![greet, counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
