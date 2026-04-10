use serde::{Deserialize, Serialize};
use reqwest::Method;
use crate::util::fetch::fetch_json;

const BASE_URL: &str = "https://api.curseforge.com/v1";
const MINECRAFT_GAME_ID: u32 = 432;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeMod {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub logo: Option<CurseForgeLogo>,
    #[serde(default)]
    pub authors: Vec<CurseForgeAuthor>,
    #[serde(default)]
    pub download_count: f64,
    #[serde(default)]
    pub date_modified: Option<String>,
    #[serde(default)]
    pub categories: Vec<CurseForgeCategory>,
    #[serde(default)]
    pub class_id: Option<u32>,
    #[serde(default)]
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeCategory {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeFile {
    pub id: u32,
    pub mod_id: u32,
    pub display_name: String,
    pub file_name: String,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub file_date: String,
    #[serde(default)]
    pub game_versions: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<CurseForgeDependency>,
    #[serde(default)]
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeDependency {
    pub mod_id: u32,
    pub relation_type: u32, // 1 = Embedded, 2 = Optional, 3 = Required, etc
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeLogo {
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeAuthor {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub data: Vec<CurseForgeMod>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default)]
    pub index: u32,
    #[serde(default)]
    pub page_size: u32,
    #[serde(default)]
    pub total_count: u32,
}

pub async fn search_curseforge(
    query: String,
    class_id: Option<u32>,
    game_version: Option<String>,
    mod_loader_type: Option<u32>,
    page: Option<u32>,
    page_size: Option<u32>,
    sort_field: Option<u32>,
    sort_order: Option<String>,
) -> crate::Result<SearchResponse> {
    let state = crate::State::get().await?;
    
    let mut url = format!("{}/mods/search?gameId={}", BASE_URL, MINECRAFT_GAME_ID);
    
    if !query.is_empty() {
        url.push_str(&format!("&searchFilter={}", urlencoding::encode(&query)));
    }
    if let Some(cid) = class_id {
        url.push_str(&format!("&classId={}", cid));
    }
    if let Some(gv) = game_version {
        url.push_str(&format!("&gameVersion={}", urlencoding::encode(&gv)));
    }
    if let Some(mlt) = mod_loader_type {
        url.push_str(&format!("&modLoaderType={}", mlt));
    }
    if let Some(p) = page {
        let ps = page_size.unwrap_or(50);
        url.push_str(&format!("&index={}", p * ps));
    }
    if let Some(ps) = page_size {
        url.push_str(&format!("&pageSize={}", ps));
    }
    if let Some(sf) = sort_field {
        url.push_str(&format!("&sortField={}", sf));
    }
    if let Some(so) = sort_order {
        url.push_str(&format!("&sortOrder={}", urlencoding::encode(&so)));
    }

    tracing::debug!("Searching CurseForge: {}", url);
    
    let res: SearchResponse = fetch_json(
        Method::GET,
        &url,
        None,
        None,
        None,
        &state.api_semaphore,
    ).await.map_err(|e| {
        tracing::error!("CurseForge search failed: {:?}", e);
        e
    })?;

    Ok(res)
}

pub async fn get_mod_cf(mod_id: u32) -> crate::Result<CurseForgeMod> {
    let state = crate::State::get().await?;
    let url = format!("{}/mods/{}", BASE_URL, mod_id);

    #[derive(Deserialize)]
    struct Wrapper { data: CurseForgeMod }

    let res: Wrapper = fetch_json(
        Method::GET,
        &url,
        None,
        None, // No manual header needed anymore
        None,
        &state.api_semaphore,
    ).await?;

    Ok(res.data)
}

pub async fn get_mod_files_cf(mod_id: u32, game_version: Option<String>, mod_loader_type: Option<u32>) -> crate::Result<Vec<CurseForgeFile>> {
    let state = crate::State::get().await?;
    
    let mut url = format!("{}/mods/{}/files?", BASE_URL, mod_id);
    if let Some(gv) = game_version {
        url.push_str(&format!("gameVersion={}&", urlencoding::encode(&gv)));
    }
    if let Some(mlt) = mod_loader_type {
        url.push_str(&format!("modLoaderType={}&", mlt));
    }

    #[derive(Deserialize)]
    struct Wrapper { data: Vec<CurseForgeFile> }

    let res: Wrapper = fetch_json(
        Method::GET,
        &url,
        None,
        None,
        None,
        &state.api_semaphore,
    ).await?;

    Ok(res.data)
}
