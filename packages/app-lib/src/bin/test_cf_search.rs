use tauri::Plugin;

#[tokio::main]
async fn main() {
    let result = theseus::curseforge::search_curseforge(
        "".to_string(), // query
        Some(4471), // class_id
        None, // game_version
        None, // mod_loader_type
        Some(0), // page
        Some(20), // page_size
        Some(1), // sort_field
        Some("desc".to_string()) // sort_order
    ).await;
    
    match result {
        Ok(res) => println!("Success! Found {} mods.", res.data.len()),
        Err(e) => println!("Error: {:?}", e),
    }
}
