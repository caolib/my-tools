mod env_var;
mod recent_projects;

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// 提交类型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitType {
    value: String,
    label: String,
    icon: String,
}

// 更新托盘菜单的快捷键显示
#[tauri::command]
fn update_tray_shortcuts(
    app: tauri::AppHandle,
    state: tauri::State<TrayState>,
    env_var_manager: String,
    file_search: String,
    projects: String,
    commit_generator: String,
) -> Result<(), String> {
    update_tray_menu(
        &app,
        &state,
        env_var_manager,
        file_search,
        projects,
        commit_generator,
        vec![],
    )
}

// 更新托盘菜单（包含提交类型）
#[tauri::command]
fn update_tray_menu_with_commit_types(
    app: tauri::AppHandle,
    state: tauri::State<TrayState>,
    env_var_manager: String,
    file_search: String,
    projects: String,
    commit_generator: String,
    commit_types: Vec<CommitType>,
) -> Result<(), String> {
    update_tray_menu(
        &app,
        &state,
        env_var_manager,
        file_search,
        projects,
        commit_generator,
        commit_types,
    )
}

// 内部函数：更新托盘菜单
fn update_tray_menu(
    app: &tauri::AppHandle,
    state: &tauri::State<TrayState>,
    env_var_manager: String,
    file_search: String,
    projects: String,
    commit_generator: String,
    commit_types: Vec<CommitType>,
) -> Result<(), String> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};

    // 更新状态
    if let Ok(mut shortcuts) = state.shortcuts.lock() {
        shortcuts.env_var_manager = env_var_manager.clone();
        shortcuts.file_search = file_search.clone();
        shortcuts.projects = projects.clone();
        shortcuts.commit_generator = commit_generator.clone();
    }

    // 构建菜单项文本（包含快捷键提示）
    let env_text = if env_var_manager.is_empty() {
        "环境变量管理".to_string()
    } else {
        format!("环境变量管理\t{}", env_var_manager)
    };

    let file_search_text = if file_search.is_empty() {
        "文件搜索".to_string()
    } else {
        format!("文件搜索\t{}", file_search)
    };

    let projects_text = if projects.is_empty() {
        "项目管理".to_string()
    } else {
        format!("项目管理\t{}", projects)
    };

    let commit_generator_text = if commit_generator.is_empty() {
        "提交生成器".to_string()
    } else {
        format!("提交生成器\t{}", commit_generator)
    };

    // 重建托盘菜单
    let env_item = MenuItemBuilder::with_id("env", env_text)
        .build(app)
        .map_err(|e| e.to_string())?;
    let file_search_item = MenuItemBuilder::with_id("file-search", file_search_text)
        .build(app)
        .map_err(|e| e.to_string())?;
    let projects_item = MenuItemBuilder::with_id("projects", projects_text)
        .build(app)
        .map_err(|e| e.to_string())?;

    // 创建提交生成器子菜单
    let commit_menu = if !commit_types.is_empty() {
        // 添加默认的"提交生成器"选项
        let default_commit_item = MenuItemBuilder::with_id("commit-generator", "提交生成器")
            .build(app)
            .map_err(|e| e.to_string())?;

        // 添加分隔符
        let sep = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;

        // 创建子菜单构建器，先添加默认项和分隔符
        let mut submenu_builder = SubmenuBuilder::new(app, commit_generator_text)
            .item(&default_commit_item)
            .item(&sep);

        // 为每个提交类型创建并添加菜单项
        for commit_type in commit_types {
            let item_text = format!("{} {}", commit_type.icon, commit_type.label);
            let item_id = format!("commit-type-{}", commit_type.value);
            let item = MenuItemBuilder::with_id(item_id, item_text)
                .build(app)
                .map_err(|e| e.to_string())?;
            submenu_builder = submenu_builder.item(&item);
        }

        let submenu = submenu_builder.build().map_err(|e| e.to_string())?;
        Some(submenu)
    } else {
        None
    };

    let settings_item = MenuItemBuilder::with_id("settings", "设置")
        .build(app)
        .map_err(|e| e.to_string())?;

    let separator = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出应用")
        .build(app)
        .map_err(|e| e.to_string())?;

    // 构建菜单
    let menu = if let Some(commit_submenu) = commit_menu {
        MenuBuilder::new(app)
            .items(&[
                &env_item,
                &file_search_item,
                &projects_item,
                &commit_submenu,
                &settings_item,
                &separator,
                &quit_item,
            ])
            .build()
            .map_err(|e| e.to_string())?
    } else {
        MenuBuilder::new(app)
            .items(&[
                &env_item,
                &file_search_item,
                &projects_item,
                &settings_item,
                &separator,
                &quit_item,
            ])
            .build()
            .map_err(|e| e.to_string())?
    };

    // 更新托盘菜单
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }

    Ok(())
}

use std::path::Path;
use std::process::Command;

use base64::Engine;
use image::ImageEncoder;
use std::collections::HashMap;
use std::sync::Mutex;

// 全局快捷键信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShortcutInfo {
    env_var_manager: String,
    file_search: String,
    projects: String,
    commit_generator: String,
}

// 托盘图标状态
struct TrayState {
    shortcuts: Mutex<ShortcutInfo>,
}

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

/// 获取缓存信息
#[derive(Debug, Serialize)]
struct CacheInfo {
    cache_path: String,
    cache_size: u64,
    cache_size_formatted: String,
}

#[tauri::command]
async fn get_cache_info(app: tauri::AppHandle) -> Result<CacheInfo, String> {
    use std::fs;
    use tauri::Manager;

    // 获取应用缓存目录
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("获取缓存目录失败: {}", e))?;

    let cache_path = cache_dir.to_string_lossy().to_string();

    // 计算缓存大小
    fn dir_size(path: &std::path::Path) -> std::io::Result<u64> {
        let mut size = 0;
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += dir_size(&entry.path())?;
                }
            }
        }
        Ok(size)
    }

    let cache_size = if cache_dir.exists() {
        dir_size(&cache_dir).unwrap_or(0)
    } else {
        0
    };

    // 格式化缓存大小
    let cache_size_formatted = if cache_size == 0 {
        "0 B".to_string()
    } else if cache_size < 1024 {
        format!("{} B", cache_size)
    } else if cache_size < 1024 * 1024 {
        format!("{:.2} KB", cache_size as f64 / 1024.0)
    } else if cache_size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", cache_size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", cache_size as f64 / (1024.0 * 1024.0 * 1024.0))
    };

    Ok(CacheInfo {
        cache_path,
        cache_size,
        cache_size_formatted,
    })
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
    use tauri::tray::TrayIconBuilder;
    use tauri::{
        menu::{MenuBuilder, MenuItemBuilder},
        Manager,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .manage(TrayState {
            shortcuts: Mutex::new(ShortcutInfo {
                env_var_manager: String::new(),
                file_search: String::new(),
                projects: String::new(),
                commit_generator: String::new(),
            }),
        })
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::menu::PredefinedMenuItem;

                // 注册全局快捷键插件
                app.handle()
                    .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;

                // 注册更新插件（仅桌面平台）
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;

                // 创建托盘菜单项
                let env_item = MenuItemBuilder::with_id("env", "环境变量管理").build(app)?;
                let file_search_item =
                    MenuItemBuilder::with_id("file-search", "文件搜索").build(app)?;
                let projects_item = MenuItemBuilder::with_id("projects", "项目管理").build(app)?;
                let commit_generator_item =
                    MenuItemBuilder::with_id("commit-generator", "提交生成器").build(app)?;
                let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;

                // 创建分隔符
                let separator = PredefinedMenuItem::separator(app)?;

                let quit_item = MenuItemBuilder::with_id("quit", "退出应用").build(app)?;

                // 构建菜单
                let menu = MenuBuilder::new(app)
                    .items(&[
                        &env_item,
                        &file_search_item,
                        &projects_item,
                        &commit_generator_item,
                        &settings_item,
                        &separator,
                        &quit_item,
                    ])
                    .build()?;

                // 创建系统托盘
                let _tray = TrayIconBuilder::with_id("main")
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .show_menu_on_left_click(false) // 禁用左键显示菜单，只在右键显示
                    .on_menu_event(move |app, event| {
                        use tauri::Emitter;

                        let window = app.get_webview_window("main");
                        let event_id = event.id().as_ref();

                        if let Some(window) = &window {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }

                        match event_id {
                            "env" => {
                                if let Some(window) = window {
                                    let _ = window.emit("navigate", "/env");
                                }
                            }
                            "file-search" => {
                                if let Some(window) = window {
                                    let _ = window.emit("navigate", "/file-search");
                                }
                            }
                            "projects" => {
                                if let Some(window) = window {
                                    let _ = window.emit("navigate", "/projects");
                                }
                            }
                            "commit-generator" => {
                                if let Some(window) = window {
                                    let _ = window.emit("navigate", "/commit-generator");
                                }
                            }
                            "settings" => {
                                if let Some(window) = window {
                                    let _ = window.emit("navigate", "/settings");
                                }
                            }
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {
                                // 处理提交类型的菜单项
                                if event_id.starts_with("commit-type-") {
                                    if let Some(window) = window {
                                        let commit_type =
                                            event_id.strip_prefix("commit-type-").unwrap_or("");
                                        let route =
                                            format!("/commit-generator?type={}", commit_type);
                                        let _ = window.emit("navigate", route);
                                    }
                                }
                            }
                        }
                    })
                    .on_tray_icon_event(|tray, event| {
                        // 处理托盘图标点击事件 - 只处理左键单击
                        use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                match window.is_visible() {
                                    Ok(true) => {
                                        // 窗口可见，隐藏它
                                        let _ = window.hide();
                                    }
                                    _ => {
                                        // 窗口隐藏或状态未知，显示它
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_version,
            get_cache_info,
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
            recent_projects::get_recent_projects,
            recent_projects::open_in_vscode,
            recent_projects::open_in_trae,
            recent_projects::open_in_qoder,
            recent_projects::open_in_idea,
            recent_projects::open_in_webstorm,
            recent_projects::open_in_pycharm,
            update_tray_shortcuts,
            update_tray_menu_with_commit_types
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
