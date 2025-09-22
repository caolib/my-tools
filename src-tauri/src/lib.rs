mod env_var;
mod recent_projects;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::path::Path;
use std::process::Command;

use base64::Engine;
use image::ImageEncoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::um::shellapi::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON};
#[cfg(target_os = "windows")]
use winapi::um::wingdi::{
    CreateCompatibleDC, DeleteDC, GetDIBits, GetObjectW, SelectObject, BITMAP, BITMAPINFOHEADER,
    DIB_RGB_COLORS,
};
#[cfg(target_os = "windows")]
use winapi::um::winnt::WCHAR;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetDC, ReleaseDC};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetIconInfo, ICONINFO};

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use std::sync::LazyLock;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{DWORD, UINT};

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

/// 文件图标缓存
#[cfg(target_os = "windows")]
static ICON_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(not(target_os = "windows"))]
static ICON_CACHE: std::sync::LazyLock<Mutex<HashMap<String, String>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(target_os = "windows")]
fn to_wide_chars(s: &str) -> Vec<WCHAR> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

#[cfg(target_os = "windows")]
fn extract_icon_to_base64(file_path: &str) -> Result<String, String> {
    use std::mem;

    let wide_path = to_wide_chars(file_path);

    unsafe {
        let mut shfi: SHFILEINFOW = mem::zeroed();
        let result = SHGetFileInfoW(
            wide_path.as_ptr(),
            0,
            &mut shfi,
            mem::size_of::<SHFILEINFOW>() as UINT,
            SHGFI_ICON | SHGFI_SMALLICON,
        );

        if result == 0 || shfi.hIcon.is_null() {
            return Err("无法获取文件图标".to_string());
        }

        let icon = shfi.hIcon;

        // 获取图标信息
        let mut icon_info: ICONINFO = mem::zeroed();
        if GetIconInfo(icon, &mut icon_info) == 0 {
            return Err("无法获取图标信息".to_string());
        }

        // 创建设备上下文
        let hdc_screen = GetDC(ptr::null_mut());
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        if hdc_mem.is_null() {
            ReleaseDC(ptr::null_mut(), hdc_screen);
            return Err("无法创建内存设备上下文".to_string());
        }

        // 获取位图信息
        let mut bitmap: BITMAP = mem::zeroed();
        if GetObjectW(
            icon_info.hbmColor as *mut _,
            mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as *mut _,
        ) == 0
        {
            DeleteDC(hdc_mem);
            ReleaseDC(ptr::null_mut(), hdc_screen);
            return Err("无法获取位图信息".to_string());
        }

        let width = bitmap.bmWidth;
        let height = bitmap.bmHeight;

        // 选择位图到内存DC
        let old_bitmap = SelectObject(hdc_mem, icon_info.hbmColor as *mut _);

        // 准备位图信息头
        let mut bi: BITMAPINFOHEADER = mem::zeroed();
        bi.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
        bi.biWidth = width;
        bi.biHeight = -height; // 负值表示从上到下
        bi.biPlanes = 1;
        bi.biBitCount = 32;
        bi.biCompression = 0; // BI_RGB

        // 分配像素数据缓冲区
        let pixel_count = (width * height) as usize;
        let mut pixels: Vec<u8> = vec![0; pixel_count * 4];

        // 获取像素数据
        let result = GetDIBits(
            hdc_mem,
            icon_info.hbmColor,
            0,
            height as UINT,
            pixels.as_mut_ptr() as *mut _,
            &mut bi as *mut _ as *mut _,
            DIB_RGB_COLORS,
        );

        // 清理资源
        SelectObject(hdc_mem, old_bitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(ptr::null_mut(), hdc_screen);

        if result == 0 {
            return Err("无法获取像素数据".to_string());
        }

        // 将BGRA格式转换为RGBA格式
        for i in (0..pixels.len()).step_by(4) {
            pixels.swap(i, i + 2); // 交换B和R
        }

        // 使用image库创建RGBA图像
        let img = image::RgbaImage::from_raw(width as u32, height as u32, pixels)
            .ok_or("无法创建图像".to_string())?;

        // 将图像编码为PNG格式
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder
            .write_image(
                img.as_raw(),
                width as u32,
                height as u32,
                image::ColorType::Rgba8,
            )
            .map_err(|e| format!("编码PNG失败: {}", e))?;

        // 转换为Base64
        let base64_string = base64::engine::general_purpose::STANDARD.encode(&png_data);
        Ok(format!("data:image/png;base64,{}", base64_string))
    }
}

/// 获取文件图标（Base64编码）
#[tauri::command]
async fn get_file_icon(file_path: String) -> Result<String, String> {
    // 首先检查文件是否存在
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    // 检查缓存
    {
        let cache = ICON_CACHE.lock().unwrap();
        if let Some(cached_icon) = cache.get(&file_path) {
            return Ok(cached_icon.clone());
        }
    }

    // 提取文件扩展名，用于相同类型文件的缓存
    let extension = std::path::Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let cache_key = if extension.is_empty() {
        // 对于文件夹或无扩展名文件，直接使用路径
        file_path.clone()
    } else {
        // 对于有扩展名的文件，使用扩展名作为缓存键
        format!(".{}", extension)
    };

    // 再次检查缓存（使用扩展名）
    {
        let cache = ICON_CACHE.lock().unwrap();
        if let Some(cached_icon) = cache.get(&cache_key) {
            return Ok(cached_icon.clone());
        }
    }

    // 提取图标
    let icon_result = extract_icon_to_base64(&file_path);

    if let Ok(icon_data) = &icon_result {
        // 缓存图标
        let mut cache = ICON_CACHE.lock().unwrap();
        cache.insert(cache_key, icon_data.clone());

        // 如果缓存过多，清理一些旧的条目
        if cache.len() > 100 {
            // 保留最后50个
            let keys_to_remove: Vec<String> =
                cache.keys().take(cache.len() - 50).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
    }

    icon_result
}

/// 批量获取文件图标
#[tauri::command]
async fn get_file_icons_batch(file_paths: Vec<String>) -> Result<HashMap<String, String>, String> {
    let mut result = HashMap::new();

    for file_path in file_paths {
        match get_file_icon(file_path.clone()).await {
            Ok(icon_data) => {
                result.insert(file_path, icon_data);
            }
            Err(_) => {
                // 失败的文件不加入结果
                continue;
            }
        }
    }

    Ok(result)
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

/// 获取桌面路径
#[tauri::command]
async fn get_desktop_path() -> Result<String, String> {
    use dirs::desktop_dir;

    desktop_dir()
        .ok_or_else(|| "无法获取桌面路径".to_string())
        .map(|path| path.to_string_lossy().to_string())
}

/// 写入文件
#[tauri::command]
async fn write_file(path: String, content: String) -> Result<(), String> {
    use std::fs;

    fs::write(&path, content).map_err(|e| format!("写入文件失败: {}", e))
}

/// 读取文件
#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    use std::fs;

    fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 读取图片文件并转换为 base64
#[tauri::command]
async fn read_image_as_base64(path: String) -> Result<String, String> {
    use base64::Engine;
    use std::fs;

    // 读取图片文件
    let image_bytes = fs::read(&path).map_err(|e| format!("读取图片失败: {}", e))?;

    // 根据文件扩展名确定 MIME 类型
    let mime_type = match Path::new(&path).extension().and_then(|ext| ext.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "image/png", // 默认类型
    };

    // 转换为 base64
    let base64_string = base64::engine::general_purpose::STANDARD.encode(&image_bytes);

    // 返回带有 data URI 前缀的 base64 字符串
    Ok(format!("data:{};base64,{}", mime_type, base64_string))
}

/// 获取文件统计信息
#[derive(Serialize)]
struct FileStats {
    size: u64,
    modified: u64,
    is_file: bool,
    is_dir: bool,
}

#[tauri::command]
async fn get_file_stats(path: String) -> Result<FileStats, String> {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let metadata = fs::metadata(&path).map_err(|e| format!("获取文件信息失败: {}", e))?;

    let modified = metadata
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(FileStats {
        size: metadata.len(),
        modified,
        is_file: metadata.is_file(),
        is_dir: metadata.is_dir(),
    })
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
    host: Option<String>,
    port: Option<u32>,
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

    // Everything服务地址配置
    let host = host.unwrap_or_else(|| "localhost".to_string());
    let port = port.unwrap_or(8080);

    let url = format!(
        "http://{}:{}/?search={}&json=1&offset={}&count={}&sort={}&ascending={}&case={}&wholeword={}&path={}&regex={}&path_column={}&size_column={}&date_modified_column={}&date_created_column=1&attributes_column=1",
        host,
        port,
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
            shell_open,
            get_file_icon,
            get_file_icons_batch,
            get_desktop_path,
            write_file,
            read_file,
            read_image_as_base64,
            get_file_stats,
            recent_projects::get_recent_vscode_projects,
            recent_projects::open_in_vscode,
            recent_projects::open_in_trae,
            recent_projects::open_in_qoder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
