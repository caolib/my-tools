mod env_var;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::process::Command;
use std::path::Path;
use reqwest;
use serde::{Deserialize, Serialize};

/// Everything搜索结果项
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: Option<String>,
    pub date_modified: Option<String>,
    pub date_created: Option<String>,
    pub attributes: Option<String>,
}

/// Everything搜索响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    #[serde(rename = "results")]
    pub results: Vec<SearchResult>,
}

#[tauri::command]
async fn open_file_location(path: String) -> Result<(), String> {
    let path = path.trim_matches('"');
    let path = Path::new(path);
    if !path.exists() {
        return Err("文件路径不存在".to_string());
    }

    // 获取文件所在的目录
    let _parent_dir = path
        .parent()
        .ok_or_else(|| "无法获取文件所在目录".to_string())?;

    // 在文件管理器中打开目录并选中文件
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开文件位置失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path.parent().unwrap())
            .spawn()
            .map_err(|e| format!("打开文件位置失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开文件位置失败: {}", e))?;
    }

    Ok(())
}

/// 使用系统默认应用程序打开文件
#[tauri::command]
async fn shell_open(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("无法打开文件: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件: {}", e))?;
    }
    
    Ok(())
}

/// 代理Everything搜索请求，解决CORS问题
#[tauri::command]
async fn search_everything(
    search: String,
    offset: Option<u32>,
    count: Option<u32>,
    sort: Option<String>,
    ascending: Option<bool>,
    case: Option<bool>,
    wholeword: Option<bool>,
    path: Option<bool>,
    regex: Option<bool>,
    path_column: Option<u32>,
    size_column: Option<u32>,
    date_modified_column: Option<u32>,
) -> Result<SearchResponse, String> {
    let offset = offset.unwrap_or(0);
    let count = count.unwrap_or(20);
    let sort = sort.unwrap_or_else(|| "name".to_string());
    let ascending = if ascending.unwrap_or(true) { 1 } else { 0 };
    let case = if case.unwrap_or(false) { 1 } else { 0 };
    let wholeword = if wholeword.unwrap_or(false) { 1 } else { 0 };
    let path = if path.unwrap_or(false) { 1 } else { 0 };
    let regex = if regex.unwrap_or(false) { 1 } else { 0 };
    
    // 列显示控制参数，默认全部显示
    let path_column = path_column.unwrap_or(1);
    let size_column = size_column.unwrap_or(1);
    let date_modified_column = date_modified_column.unwrap_or(1);

    let url = format!(
        "http://localhost:8080/?search={}&json=1&offset={}&count={}&sort={}&ascending={}&case={}&wholeword={}&path={}&regex={}&path_column={}&size_column={}&date_modified_column={}&date_created_column=1&attributes_column=1",
        urlencoding::encode(&search),
        offset,
        count,
        sort,
        ascending,
        case,
        wholeword,
        path,
        regex,
        path_column,
        size_column,
        date_modified_column
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("搜索请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("搜索服务返回错误: {}", response.status()));
    }

    let search_response: SearchResponse = response
        .json()
        .await
        .map_err(|e| format!("解析搜索结果失败: {}", e))?;

    Ok(search_response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            env_var::get_env_vars,
            env_var::set_env_var,
            env_var::delete_env_var,
            env_var::check_admin_privileges,
            env_var::request_admin_privileges,
            env_var::export_env_vars,
            env_var::export_env_vars_to_path,
            env_var::get_documents_dir,
            env_var::import_env_vars,
            env_var::reveal_in_explorer,
            env_var::open_folder,
            env_var::check_paths_exist,
            env_var::scan_config_files,
            open_file_location,
            search_everything,
            shell_open
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
