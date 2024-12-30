// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use password_manager_lib::models::PasswordEntry;
use password_manager_lib::db;

#[tauri::command]
async fn save_password(entry: PasswordEntry) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO passwords (name, login_id, password, url) VALUES (?1, ?2, ?3, ?4)",
        (
            &entry.name,
            &entry.login_id,
            &entry.password,
            &entry.url,
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_passwords() -> Result<Vec<PasswordEntry>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, login_id, password, url FROM passwords")
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([], |row| {
            Ok(PasswordEntry {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                login_id: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
async fn get_password(id: i64) -> Result<PasswordEntry, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, login_id, password, url FROM passwords WHERE id = ?")
        .map_err(|e| e.to_string())?;
    
    let entry = stmt
        .query_row([id], |row| {
            Ok(PasswordEntry {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                login_id: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    Ok(entry)
}

#[tauri::command]
async fn update_password(entry: PasswordEntry) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE passwords SET name = ?1, login_id = ?2, password = ?3, url = ?4 WHERE id = ?5",
        (
            &entry.name,
            &entry.login_id,
            &entry.password,
            &entry.url,
            entry.id,
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_password,
            get_passwords,
            update_password,
            get_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
