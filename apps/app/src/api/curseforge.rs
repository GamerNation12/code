use crate::api::Result;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use theseus::api::curseforge::{search_curseforge, get_mod_cf, get_mod_files_cf, SearchResponse, CurseForgeMod, CurseForgeFile};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("curseforge")
        .invoke_handler(tauri::generate_handler![
            curseforge_search,
            curseforge_get_mod,
            curseforge_get_mod_files,
        ])
        .build()
}

#[tauri::command]
pub async fn curseforge_search(
    query: String,
    class_id: Option<u32>,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
    page: Option<u32>,
) -> Result<SearchResponse> {
    Ok(search_curseforge(query, class_id, game_version, mod_loader_type, page).await?)
}

#[tauri::command]
pub async fn curseforge_get_mod(mod_id: u32) -> Result<CurseForgeMod> {
    Ok(get_mod_cf(mod_id).await?)
}

#[tauri::command]
pub async fn curseforge_get_mod_files(
    mod_id: u32,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
) -> Result<Vec<CurseForgeFile>> {
    Ok(get_mod_files_cf(mod_id, game_version, mod_loader_type).await?)
}
