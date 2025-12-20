use tauri::Manager;
use tauri_plugin_http::reqwest;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;
use tar::Archive;
use xz2::read::XzDecoder;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/ 
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_prepare(app: tauri::AppHandle) -> Result<(), String> {
    let url = "https://github.com/lbls741/LinuxLuncherWineRelease/releases/download/Release/wine-osu-winello-fonts-wow64-10.12-1-x86_64.tar.xz".to_string();
    let file_name = "wine-osu-winello-fonts-wow64-10.12-1-x86_64.tar.xz".to_string();

    let downloaded_path = download_file(app.clone(), url, file_name).await?;
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    decompress_file(&downloaded_path, &resource_dir)?;

    // After decompression, create the WINE prefix
    let wine_dist_path = resource_dir.join("wine-osu-winello-fonts-wow64-10.12-1-x86_64");
    let prefix_path = resource_dir.join("wine_prefix");

    // Run blocking operation in a separate thread
    let prefix_path_clone = prefix_path.clone();
    let blocking_task = tauri::async_runtime::spawn_blocking(move || {
        create_wine_prefix(&wine_dist_path, &prefix_path_clone)
    });
    blocking_task.await.map_err(|e| e.to_string())??; // The first ? is for JoinError, the second for the inner Result

    // Update config file with the new prefix path
    update_config_with_prefix_path(&app, &prefix_path)?;

    fs::remove_file(downloaded_path).map_err(|e| e.to_string())?;

    Ok(())
}

fn update_config_with_prefix_path(app: &tauri::AppHandle, prefix_path: &PathBuf) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join("config.yaml");
    let mut config_content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;

    let prefix_path_str = prefix_path.to_str().ok_or("Failed to convert prefix path to string")?;

    config_content = config_content.replace("env_dir: \"\"", &format!("env_dir: \"{}\"", prefix_path_str));

    fs::write(&config_path, config_content).map_err(|e| e.to_string())?;

    Ok(())
}

async fn download_file(app: tauri::AppHandle, url: String, file_path: String) -> Result<PathBuf, String> {
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    if !resource_dir.exists() {
        fs::create_dir_all(&resource_dir).map_err(|e| e.to_string())?;
    }
    let final_path = resource_dir.join(file_path);

    fs::write(&final_path, bytes).map_err(|e| e.to_string())?;
    
    Ok(final_path)
}

fn decompress_file(file_path: &PathBuf, target_dir: &PathBuf) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let decompressor = XzDecoder::new(file);
    let mut archive = Archive::new(decompressor);
    archive.unpack(target_dir).map_err(|e| e.to_string())?;
    Ok(())
}

fn create_wine_prefix(wine_dist_path: &PathBuf, prefix_path: &PathBuf) -> Result<(), String> {
    let wine_executable = wine_dist_path.join("bin").join("wine");
    if !wine_executable.exists() {
        return Err(format!("WINE executable not found at: {:?}", wine_executable));
    }

    let status = Command::new(&wine_executable)
        .env("WINEPREFIX", prefix_path)
        .arg("wineboot")
        .arg("-u")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to create WINE prefix. Exit code: {:?}", status.code()))
    }
}

#[tauri::command]
async fn check_config_and_initialize(app: tauri::AppHandle) -> Result<bool, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    
    let config_path = config_dir.join("config.yaml");

    if config_path.exists() {
        // Config file exists, tell frontend to proceed to main interface.
        Ok(false)
    } else {
        // Config file does not exist, create it and tell frontend to show init interface.
        let initial_config = r#"luncher:
  env_dir: ""
  res_dir: ""
"#;
        fs::write(&config_path, initial_config).map_err(|e| e.to_string())?;
        Ok(true)
    }
}

async fn bootstrap(app: tauri::AppHandle) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let config_path = config_dir.join("config.yaml");

    if config_path.exists() {
        // Config file exists, open and close it (read and discard content)
        let _ = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;

        // Call setup environment and game data functions
        setup_environment(app.clone()).await?;
        get_gamedata(app.clone()).await?;
    } else {
        // Config file does not exist, call initialize
        initialize(app.clone()).await?;
    }

    Ok(())
}

async fn setup_environment(app: tauri::AppHandle) -> Result<(), String> {
    // TODO: Implement environment setup
    Ok(())
}

async fn get_gamedata(app: tauri::AppHandle) -> Result<(), String> {
    // TODO: Implement game data retrieval
    Ok(())
}

async fn initialize(app: tauri::AppHandle) -> Result<(), String> {
    // Get system information using tauri-plugin-os
    let arch = tauri_plugin_os::arch();
    let platform = tauri_plugin_os::platform();

    // Check if system is supported
    // Supported platforms: linux, macos
    // Supported architectures: x86_64 (amd64), aarch64 (arm64)
    let is_supported_platform = platform == "linux" || platform == "macos";
    let is_supported_arch = arch == "x86_64" || arch == "aarch64";

    if !is_supported_platform || !is_supported_arch {
        // System not supported, tell frontend to show "reject run" interface
        // No config file will be created
        return Ok(());
    }

    // Map architecture to arm/x86
    let system_arch = if arch.contains("arm") || arch.contains("aarch") {
        "arm".to_string()
    } else if arch.contains("x86") {
        "x86".to_string()
    } else {
        arch.to_string()
    };

    // Map platform to Linux/MacOS/Other
    let system_os = match platform {
        "linux" => "Linux".to_string(),
        "macos" => "MacOS".to_string(),
        _ => platform.to_string(),
    };

    // Create config directory if it doesn't exist
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    // Create config file with system information
    let config_path = config_dir.join("config.yaml");
    let initial_config = format!(
        r#"luncher:
  env_dir: ""
  res_dir: ""
  system_arch: "{}"
  system_os: "{}"
"#,
        system_arch, system_os
    );

    fs::write(&config_path, initial_config).map_err(|e| e.to_string())?;

    // Tell frontend to show "initialization" interface
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_prepare,
            check_config_and_initialize
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = bootstrap(app_handle).await {
                    eprintln!("Bootstrap error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
