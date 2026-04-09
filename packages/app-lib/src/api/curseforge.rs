use serde::{Deserialize, Serialize};
use reqwest::Method;
use crate::util::fetch::{fetch_json, FetchSemaphore};

const BASE_URL: &str = "https://api.curseforge.com/v1";
const MINECRAFT_GAME_ID: u32 = 432;
const HARDCODED_API_KEY: &str = "$2a$10$vS0j.1Y8NfA/tByE0iBe5K7j7S3E0JvD5y8P25ZAgX6u4m/5/K97Zgxte";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeMod {
    pub id: u32,
    pub name: String,
    pub summary: Option<String>,
    pub website_url: Option<String>,
    pub logo: Option<CurseForgeLogo>,
    pub authors: Vec<CurseForgeAuthor>,
    pub download_count: f64,
    pub date_modified: String,
    pub categories: Vec<CurseForgeCategory>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeCategory {
    pub id: u32,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeFile {
    pub id: u32,
    pub mod_id: u32,
    pub display_name: String,
    pub file_name: String,
    pub download_url: Option<String>,
    pub file_date: String,
    pub game_versions: Vec<String>,
    pub dependencies: Vec<CurseForgeDependency>,
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
    pub thumbnail_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeAuthor {
    pub name: String,
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
) -> crate::Result<SearchResponse> {
    let state = crate::State::get().await?;
    let api_key = HARDCODED_API_KEY;
    
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
        url.push_str(&format!("&index={}", p * 50)); // Assuming 50 per page
    }

    let header = ("x-api-key", api_key);
    
    tracing::debug!("Searching CurseForge: {}", url);
    
    let res: SearchResponse = fetch_json(
        Method::GET,
        &url,
        None,
        None,
        Some(header),
        &state.api_semaphore,
    ).await.map_err(|e| {
        tracing::error!("CurseForge search failed: {:?}", e);
        e
    })?;

    Ok(res)
}

pub async fn get_mod_cf(mod_id: u32) -> crate::Result<CurseForgeMod> {
    let state = crate::State::get().await?;
    let api_key = HARDCODED_API_KEY;
    let url = format!("{}/mods/{}", BASE_URL, mod_id);

    #[derive(Deserialize)]
    struct Wrapper { data: CurseForgeMod }

    let header = ("x-api-key", api_key);

    let res: Wrapper = fetch_json(
        Method::GET,
        &url,
        None,
        None,
        Some(header),
        &state.api_semaphore,
    ).await?;

    Ok(res.data)
}

pub async fn get_mod_files_cf(mod_id: u32, game_version: Option<String>, mod_loader_type: Option<u32>) -> crate::Result<Vec<CurseForgeFile>> {
    let state = crate::State::get().await?;
    let api_key = HARDCODED_API_KEY;
    
    let mut url = format!("{}/mods/{}/files?", BASE_URL, mod_id);
    if let Some(gv) = game_version {
        url.push_str(&format!("gameVersion={}&", urlencoding::encode(&gv)));
    }
    if let Some(mlt) = mod_loader_type {
        url.push_str(&format!("modLoaderType={}&", mlt));
    }

    #[derive(Deserialize)]
    struct Wrapper { data: Vec<CurseForgeFile> }

    let header = ("x-api-key", api_key);

    let res: Wrapper = fetch_json(
        Method::GET,
        &url,
        None,
        None,
        Some(header),
        &state.api_semaphore,
    ).await?;

    Ok(res.data)
}
