use crate::api::Result;
use theseus::api::curseforge::{search_curseforge, SearchResponse};

#[tauri::command]
pub async fn search_cf(
    query: String,
    class_id: Option<u32>,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
    page: Option<u32>,
) -> Result<SearchResponse> {
    Ok(search_curseforge(query, class_id, game_version, mod_loader_type, page).await?)
}

#[tauri::command]
pub async fn get_mod_cf(mod_id: u32) -> Result<theseus::api::curseforge::CurseForgeMod> {
    Ok(theseus::api::curseforge::get_mod_cf(mod_id).await?)
}

#[tauri::command]
pub async fn get_mod_files_cf(
    mod_id: u32,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
) -> Result<Vec<theseus::api::curseforge::CurseForgeFile>> {
    Ok(theseus::api::curseforge::get_mod_files_cf(mod_id, game_version, mod_loader_type).await?)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("curseforge")
        .invoke_handler(tauri::generate_handler![
            search_cf,
            get_mod_cf,
            get_mod_files_cf
        ])
        .build()
}

