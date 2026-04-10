use std::io::Cursor;
use serde::{Deserialize, Serialize};
use crate::{State, profile};
use crate::api::curseforge::get_mod_files_cf;
use crate::event::emit::{emit_loading, init_or_edit_loading, loading_try_for_each_concurrent};
use crate::event::{LoadingBarId, LoadingBarType};
use crate::state::ProfileInstallStage;
use crate::util::fetch::{fetch_advanced, write};
use async_zip::base::read::seek::ZipFileReader;
use reqwest::Method;
use urlencoding::encode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeManifest {
    pub minecraft: MinecraftInfo,
    pub manifest_type: String,
    pub manifest_version: u32,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<CurseForgeFileReference>,
    pub overrides: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftInfo {
    pub version: String,
    pub mod_loaders: Vec<ModLoaderInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModLoaderInfo {
    pub id: String,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeFileReference {
    pub project_id: u32,
    pub file_id: u32,
    pub required: bool,
}

pub async fn install_zipped_cfpack(
    zip_bytes: bytes::Bytes,
    profile_path: String,
    existing_loading_bar: Option<LoadingBarId>,
) -> crate::Result<String> {
    let state = State::get().await?;
    let reader = Cursor::new(&zip_bytes);

    let mut zip_reader = ZipFileReader::with_tokio(reader).await.map_err(|_| {
        crate::Error::from(crate::ErrorKind::InputError(
            "Failed to read CurseForge modpack zip".to_string(),
        ))
    })?;

    // Find and parse manifest.json
    let manifest_idx = zip_reader.file().entries().iter().position(|f| {
        f.filename().as_str().is_ok_and(|name| name == "manifest.json")
    }).ok_or_else(|| {
        crate::ErrorKind::InputError("No manifest.json found in CurseForge pack".to_string())
    })?;

    let mut manifest_str = String::new();
    let mut reader = zip_reader.reader_with_entry(manifest_idx).await?;
    reader.read_to_string_checked(&mut manifest_str).await?;

    let manifest: CurseForgeManifest = serde_json::from_str(&manifest_str)?;

    // Map ModLoader
    let primary_loader = manifest.minecraft.mod_loaders.iter().find(|l| l.primary)
        .or_else(|| manifest.minecraft.mod_loaders.first());
    
    let (mod_loader, loader_version) = if let Some(loader) = primary_loader {
        let parts: Vec<&str> = loader.id.split('-').collect();
        match parts.first().map(|&s| s.to_lowercase()).as_deref() {
            Some("forge") => (crate::data::ModLoader::Forge, parts.get(1).map(|&s| s.to_string())),
            Some("fabric") => (crate::data::ModLoader::Fabric, parts.get(1).map(|&s| s.to_string())),
            Some("quilt") => (crate::data::ModLoader::Quilt, parts.get(1).map(|&s| s.to_string())),
            Some("neoforge") => (crate::data::ModLoader::NeoForge, parts.get(1).map(|&s| s.to_string())),
            _ => (crate::data::ModLoader::Vanilla, None),
        }
    } else {
        (crate::data::ModLoader::Vanilla, None)
    };

    // Update Profile Info
    profile::edit(&profile_path, |prof| {
        prof.name = manifest.name.clone();
        prof.game_version = manifest.minecraft.version.clone();
        prof.loader = mod_loader;
        prof.loader_version = loader_version.clone();
        prof.install_stage = ProfileInstallStage::PackInstalling;
        async { Ok(()) }
    }).await?;

    let loading_bar = init_or_edit_loading(
        existing_loading_bar,
        LoadingBarType::PackDownload {
            profile_path: profile_path.clone(),
            pack_name: manifest.name.clone(),
            icon: None,
            pack_id: None,
            pack_version: Some(manifest.version.clone()),
        },
        100.0,
        "Installing CurseForge modpack",
    ).await?;

    // Download Mods
    let num_files = manifest.files.len();
    loading_try_for_each_concurrent(
        futures::stream::iter(manifest.files.into_iter().map(Ok::<_, crate::Error>)),
        None,
        Some(&loading_bar),
        70.0,
        num_files,
        None,
        |file_ref| {
            let profile_path = profile_path.clone();
            async move {
                if !file_ref.required { return Ok(()); }

                // Fetch file details to get download URL
                let mod_files = get_mod_files_cf(file_ref.project_id, None, None).await?;
                let file_info = mod_files.into_iter().find(|f| f.id == file_ref.file_id)
                    .ok_or_else(|| crate::ErrorKind::InputError(format!("File {} not found for mod {}", file_ref.file_id, file_ref.project_id)))?;

                let download_url = file_info.download_url.clone().unwrap_or_else(|| {
                    // Prism-style Edge Fallback
                    let file_id = file_info.id;
                    let part_a = file_id / 1000;
                    let part_b = file_id % 1000;
                    format!(
                        "https://edge.forgecdn.net/files/{}/{}/{}",
                        part_a,
                        format!("{:03}", part_b),
                        urlencoding::encode(&file_info.file_name)
                    )
                });

                let state = crate::State::get().await?;
                let data = fetch_advanced(
                    Method::GET,
                    &download_url,
                    None,
                    None,
                    None,
                    None,
                    &state.fetch_semaphore
                ).await?;

                let target_path = profile::get_full_path(&profile_path).await?
                    .join("mods")
                    .join(&file_info.file_name);
                
                write(&target_path, &data, &state.io_semaphore).await?;

                Ok(())
            }
        }
    ).await?;

    // Extract Overrides
    emit_loading(&loading_bar, 0.0, Some("Extracting overrides"))?;
    let overrides_prefix = format!("{}/", manifest.overrides);
    let entries = zip_reader.file().entries().to_vec();
    for (idx, entry) in entries.iter().enumerate() {
        let filename = entry.filename().as_str().unwrap_or_default();
        if filename.starts_with(&overrides_prefix) && !filename.ends_with('/') {
            let relative_path = &filename[overrides_prefix.len()..];
            let mut file_bytes = Vec::new();
            
            // Re-read carefully
            let mut reader = zip_reader.reader_with_entry(idx).await?;
            reader.read_to_end_checked(&mut file_bytes).await?;

            let target_path = profile::get_full_path(&profile_path).await?.join(relative_path);
            write(&target_path, &bytes::Bytes::from(file_bytes), &state.io_semaphore).await?;
        }
    }

    if let Some(profile_val) = profile::get(&profile_path).await? {
        crate::launcher::install_minecraft(&profile_val, Some(loading_bar), false).await?;
    }

    Ok(profile_path)
}

pub async fn install_zipped_cfpack_wrapper(
    location: crate::api::pack::install_from::CreatePackLocation,
    profile_path: String,
) -> crate::Result<String> {
    let result = match location {
        crate::api::pack::install_from::CreatePackLocation::FromCurseForgeVersionId {
            project_id,
            file_id,
            title,
            icon_url,
        } => {
            let create_pack = crate::api::pack::install_from::generate_pack_from_cf_version_id(
                project_id,
                file_id,
                title,
                icon_url,
                profile_path.clone(),
                None,
            ).await?;
            install_zipped_cfpack(create_pack.file, profile_path.clone(), create_pack.description.existing_loading_bar).await
        }
        _ => Err(crate::ErrorKind::InputError("Invalid location for CurseForge pack install".to_string()).into()),
    };

    match result {
        Ok(profile) => Ok(profile),
        Err(err) => {
            let _ = crate::api::profile::remove(&profile_path).await;
            Err(err)
        }
    }
}
