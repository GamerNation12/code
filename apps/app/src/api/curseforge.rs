use crate::api::Result;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;

#[tauri::command]
pub async fn search_curseforge(
    query: &str,
    class_id: Option<u32>,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
    page: Option<u32>,
    page_size: Option<u32>,
    sort_field: Option<u32>,
    sort_order: Option<String>,
) -> Result<theseus::curseforge::SearchResponse> {
    Ok(theseus::curseforge::search_curseforge(
        query.to_string(),
        class_id,
        game_version,
        mod_loader_type,
        page,
        page_size,
        sort_field,
        sort_order,
    ).await?)
}

#[tauri::command]
pub async fn get_mod_curseforge(mod_id: u32) -> Result<theseus::curseforge::CurseForgeMod> {
    Ok(theseus::curseforge::get_mod_cf(mod_id).await?)
}

#[tauri::command]
pub async fn get_mod_files_curseforge(
    mod_id: u32,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
) -> Result<Vec<theseus::curseforge::CurseForgeFile>> {
    Ok(theseus::curseforge::get_mod_files_cf(mod_id, game_version, mod_loader_type).await?)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("curseforge")
        .invoke_handler(tauri::generate_handler![
            search_curseforge,
            get_mod_curseforge,
            get_mod_files_curseforge,
        ])
        .build()
}
