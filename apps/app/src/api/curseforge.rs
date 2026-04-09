use crate::api::Result;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use theseus::curseforge::{search_curseforge, get_mod_cf, get_mod_files_cf};

#[tauri::command]
pub async fn search_curseforge_handler(
    query: &str,
    index: u32,
    filter: u32,
) -> Result<theseus::curseforge::SearchResponse> {
    Ok(search_curseforge(query, index, filter).await?)
}

#[tauri::command]
pub async fn get_mod_cf_handler(mod_id: u32) -> Result<theseus::curseforge::CurseForgeMod> {
    Ok(get_mod_cf(mod_id).await?)
}

#[tauri::command]
pub async fn get_mod_files_cf_handler(
    mod_id: u32,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
) -> Result<Vec<theseus::curseforge::CurseForgeFile>> {
    Ok(get_mod_files_cf(mod_id, game_version, mod_loader_type).await?)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("curseforge")
        .invoke_handler(tauri::generate_handler![
            search_curseforge_handler,
            get_mod_cf_handler,
            get_mod_files_cf_handler,
        ])
        .build()
}
