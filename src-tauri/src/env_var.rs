use serde::{Deserialize, Serialize};
use tauri::command;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVarsResponse {
    pub system_vars: Vec<EnvVar>,
    pub user_vars: Vec<EnvVar>,
}

// 读取系统环境变量
fn read_system_env_vars() -> Result<Vec<EnvVar>, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_key = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
        .map_err(|e| format!("无法打开系统环境变量注册表: {}", e))?;

    let mut vars = Vec::new();

    for (name, value) in env_key.enum_values().map(|x| x.unwrap()) {
        let value_str = match value {
            winreg::RegValue { vtype: REG_SZ, .. }
            | winreg::RegValue {
                vtype: REG_EXPAND_SZ,
                ..
            } => env_key.get_value::<String, _>(&name).unwrap_or_default(),
            _ => continue, // 跳过非字符串类型的值
        };

        vars.push(EnvVar {
            name,
            value: value_str,
        });
    }

    // 按名称排序
    vars.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(vars)
}

// 读取用户环境变量
fn read_user_env_vars() -> Result<Vec<EnvVar>, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu
        .open_subkey("Environment")
        .map_err(|e| format!("无法打开用户环境变量注册表: {}", e))?;

    let mut vars = Vec::new();

    for (name, value) in env_key.enum_values().map(|x| x.unwrap()) {
        let value_str = match value {
            winreg::RegValue { vtype: REG_SZ, .. }
            | winreg::RegValue {
                vtype: REG_EXPAND_SZ,
                ..
            } => env_key.get_value::<String, _>(&name).unwrap_or_default(),
            _ => continue, // 跳过非字符串类型的值
        };

        vars.push(EnvVar {
            name,
            value: value_str,
        });
    }

    // 按名称排序
    vars.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(vars)
}

#[command]
pub fn get_env_vars() -> Result<EnvVarsResponse, String> {
    let system_vars = read_system_env_vars()?;
    let user_vars = read_user_env_vars()?;

    Ok(EnvVarsResponse {
        system_vars,
        user_vars,
    })
}

#[command]
pub fn set_env_var(name: String, value: String, is_system: bool) -> Result<(), String> {
    if is_system {
        // 设置系统环境变量（需要管理员权限）
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let env_key = hklm
            .open_subkey_with_flags(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
                KEY_WRITE,
            )
            .map_err(|e| {
                format!(
                    "无法打开系统环境变量注册表进行写入（可能需要管理员权限）: {}",
                    e
                )
            })?;

        env_key
            .set_value(&name, &value)
            .map_err(|e| format!("无法设置系统环境变量: {}", e))?;
    } else {
        // 设置用户环境变量
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey_with_flags("Environment", KEY_WRITE)
            .map_err(|e| format!("无法打开用户环境变量注册表进行写入: {}", e))?;

        env_key
            .set_value(&name, &value)
            .map_err(|e| format!("无法设置用户环境变量: {}", e))?;
    }

    // 通知系统环境变量已更改
    unsafe {
        use std::ffi::CString;

        extern "system" {
            fn SendMessageTimeoutA(
                hwnd: *mut std::ffi::c_void,
                msg: u32,
                wparam: usize,
                lparam: isize,
                fuflags: u32,
                utimeout: u32,
                lpdwresult: *mut usize,
            ) -> isize;
        }

        const HWND_BROADCAST: *mut std::ffi::c_void = 0xffff as *mut std::ffi::c_void;
        const WM_SETTINGCHANGE: u32 = 0x001A;
        const SMTO_ABORTIFHUNG: u32 = 0x0002;

        let env_cstring = CString::new("Environment").unwrap();
        let mut result: usize = 0;

        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            env_cstring.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            5000,
            &mut result,
        );
    }

    Ok(())
}

#[command]
pub fn delete_env_var(name: String, is_system: bool) -> Result<(), String> {
    if is_system {
        // 删除系统环境变量（需要管理员权限）
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let env_key = hklm
            .open_subkey_with_flags(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
                KEY_WRITE,
            )
            .map_err(|e| {
                format!(
                    "无法打开系统环境变量注册表进行写入（可能需要管理员权限）: {}",
                    e
                )
            })?;

        env_key
            .delete_value(&name)
            .map_err(|e| format!("无法删除系统环境变量: {}", e))?;
    } else {
        // 删除用户环境变量
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey_with_flags("Environment", KEY_WRITE)
            .map_err(|e| format!("无法打开用户环境变量注册表进行写入: {}", e))?;

        env_key
            .delete_value(&name)
            .map_err(|e| format!("无法删除用户环境变量: {}", e))?;
    }

    // 通知系统环境变量已更改
    unsafe {
        use std::ffi::CString;

        extern "system" {
            fn SendMessageTimeoutA(
                hwnd: *mut std::ffi::c_void,
                msg: u32,
                wparam: usize,
                lparam: isize,
                fuflags: u32,
                utimeout: u32,
                lpdwresult: *mut usize,
            ) -> isize;
        }

        const HWND_BROADCAST: *mut std::ffi::c_void = 0xffff as *mut std::ffi::c_void;
        const WM_SETTINGCHANGE: u32 = 0x001A;
        const SMTO_ABORTIFHUNG: u32 = 0x0002;

        let env_cstring = CString::new("Environment").unwrap();
        let mut result: usize = 0;

        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            env_cstring.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            5000,
            &mut result,
        );
    }

    Ok(())
}

#[command]
pub fn check_admin_privileges() -> Result<bool, String> {
    // 尝试以写权限打开系统环境变量注册表键来检测管理员权限
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    match hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
        KEY_WRITE,
    ) {
        Ok(_) => Ok(true),   // 可以打开，说明有管理员权限
        Err(_) => Ok(false), // 无法打开，说明没有管理员权限
    }
}

// 请求管理员权限重启应用 - 使用 Tauri process 插件优雅重启
#[command]
pub async fn request_admin_privileges(app_handle: tauri::AppHandle) -> Result<(), String> {
    use std::env;
    use std::process::Command;

    // 获取当前执行文件路径
    let current_exe = env::current_exe().map_err(|e| format!("无法获取当前执行文件路径: {}", e))?;

    // 先保存窗口状态
    if let Err(e) = tauri_plugin_window_state::AppHandleExt::save_window_state(
        &app_handle,
        tauri_plugin_window_state::StateFlags::all(),
    ) {
        eprintln!("保存窗口状态失败: {}", e);
    }

    // 使用 PowerShell 以管理员身份启动新实例
    let _output = Command::new("powershell")
        .args(&[
            "-WindowStyle",
            "Hidden",
            "-Command",
            &format!(
                "Start-Process -FilePath '{}' -Verb RunAs",
                current_exe.display()
            ),
        ])
        .spawn()
        .map_err(|e| format!("无法启动管理员权限进程: {}", e))?;

    // 使用标准库延迟退出
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        std::process::exit(0);
    });

    Ok(())
}

// 导出配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVarExport {
    pub export_info: ExportInfo,
    pub system_vars: Vec<EnvVar>,
    pub user_vars: Vec<EnvVar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportInfo {
    pub export_time: String,
    pub version: String,
}

// 获取用户文档文件夹路径
#[command]
pub async fn get_documents_dir() -> Result<String, String> {
    let documents_dir = dirs::document_dir().ok_or_else(|| "无法获取文档文件夹路径".to_string())?;
    Ok(documents_dir.to_string_lossy().to_string())
}

// 导出环境变量到 JSON 文件，支持自定义路径
#[command]
pub async fn export_env_vars_to_path(file_path: String) -> Result<String, String> {
    use chrono::Utc;
    use std::fs;

    // 获取当前环境变量
    let system_vars = read_system_env_vars()?;
    let user_vars = read_user_env_vars()?;

    // 如果未提供文件路径，使用默认文档路径
    let final_path = if file_path.is_empty() {
        let documents_dir =
            dirs::document_dir().ok_or_else(|| "无法获取文档文件夹路径".to_string())?;
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("环境变量备份_{}.json", timestamp);
        documents_dir.join(filename)
    } else {
        std::path::PathBuf::from(&file_path)
    };

    // 确保目录存在
    if let Some(parent) = final_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 创建导出数据
    let export_data = EnvVarExport {
        export_info: ExportInfo {
            export_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            version: "1.0".to_string(),
        },
        system_vars,
        user_vars,
    };

    // 序列化为 JSON
    let json_content =
        serde_json::to_string_pretty(&export_data).map_err(|e| format!("序列化数据失败: {}", e))?;

    // 写入文件
    fs::write(&final_path, json_content).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(final_path.to_string_lossy().to_string())
}

// 导出环境变量到 JSON 文件（向后兼容）
#[command]
pub async fn export_env_vars() -> Result<String, String> {
    export_env_vars_to_path(String::new()).await
}

// 在文件管理器中显示文件
#[command]
pub async fn reveal_in_explorer(file_path: String) -> Result<(), String> {
    use std::process::Command;

    // 使用 Windows 的 explorer.exe 并传递 /select 参数来选中文件
    Command::new("explorer")
        .args(&["/select,", &file_path])
        .spawn()
        .map_err(|e| format!("无法打开文件管理器: {}", e))?;

    Ok(())
}

// 打开文件夹
#[command]
pub async fn open_folder(folder_path: String) -> Result<(), String> {
    use std::process::Command;

    // 使用 Windows 的 explorer.exe 直接打开文件夹
    Command::new("explorer")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| format!("无法打开文件夹: {}", e))?;

    Ok(())
}

// 导入环境变量配置
#[command]
pub async fn import_env_vars(file_path: String) -> Result<String, String> {
    use std::fs;

    // 读取文件内容
    let json_content =
        fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;

    // 尝试解析 JSON - 兼容新旧格式
    let import_data: EnvVarExport =
        if let Ok(data) = serde_json::from_str::<EnvVarExport>(&json_content) {
            // 新格式：包含 export_info
            data
        } else {
            // 尝试解析旧格式
            match serde_json::from_str::<serde_json::Value>(&json_content) {
                Ok(json_value) => {
                    // 旧格式1：system/user 字段
                    if let (Some(system), Some(user)) = (
                        json_value
                            .get("system")
                            .and_then(|v| serde_json::from_value::<Vec<EnvVar>>(v.clone()).ok()),
                        json_value
                            .get("user")
                            .and_then(|v| serde_json::from_value::<Vec<EnvVar>>(v.clone()).ok()),
                    ) {
                        EnvVarExport {
                            export_info: ExportInfo {
                                export_time: json_value
                                    .get("exportTime")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("未知")
                                    .to_string(),
                                version: json_value
                                    .get("appVersion")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("1.0")
                                    .to_string(),
                            },
                            system_vars: system,
                            user_vars: user,
                        }
                    } else {
                        return Err("JSON文件格式不正确，缺少必要的环境变量数据".to_string());
                    }
                }
                Err(e) => return Err(format!("解析 JSON 文件失败: {}", e)),
            }
        };

    let mut imported_count = 0;
    let mut failed_count = 0;
    let mut failed_vars = Vec::new();

    // 导入系统变量
    for var in import_data.system_vars {
        match set_env_var_internal(&var.name, &var.value, true) {
            Ok(_) => imported_count += 1,
            Err(e) => {
                failed_count += 1;
                failed_vars.push(format!("系统变量 {}: {}", var.name, e));
            }
        }
    }

    // 导入用户变量
    for var in import_data.user_vars {
        match set_env_var_internal(&var.name, &var.value, false) {
            Ok(_) => imported_count += 1,
            Err(e) => {
                failed_count += 1;
                failed_vars.push(format!("用户变量 {}: {}", var.name, e));
            }
        }
    }

    // 组装结果信息
    let mut result = format!("导入完成！成功导入 {} 个变量", imported_count);
    if failed_count > 0 {
        result.push_str(&format!("，失败 {} 个", failed_count));
        if failed_vars.len() <= 5 {
            result.push_str("：\n");
            result.push_str(&failed_vars.join("\n"));
        } else {
            result.push_str("，请检查文件权限或变量格式");
        }
    }

    Ok(result)
}

// 内部函数：设置环境变量（不发送系统消息）
fn set_env_var_internal(name: &str, value: &str, is_system: bool) -> Result<(), String> {
    if is_system {
        // 设置系统环境变量（需要管理员权限）
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let env_key = hklm
            .open_subkey_with_flags(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
                KEY_WRITE,
            )
            .map_err(|e| {
                format!(
                    "无法打开系统环境变量注册表进行写入（可能需要管理员权限）: {}",
                    e
                )
            })?;

        env_key
            .set_value(name, &value)
            .map_err(|e| format!("无法设置系统环境变量: {}", e))?;
    } else {
        // 设置用户环境变量
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey_with_flags("Environment", KEY_WRITE)
            .map_err(|e| format!("无法打开用户环境变量注册表进行写入: {}", e))?;

        env_key
            .set_value(name, &value)
            .map_err(|e| format!("无法设置用户环境变量: {}", e))?;
    }

    Ok(())
}

// 检查路径是否存在
#[command]
pub async fn check_paths_exist(paths: Vec<String>) -> Result<Vec<bool>, String> {
    use std::path::Path;

    let mut results = Vec::new();

    for path in paths {
        let path_exists = Path::new(&path).exists();
        results.push(path_exists);
    }

    Ok(results)
}

// 配置文件信息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFileInfo {
    pub file_path: String,
    pub file_name: String,
    pub export_time: String,
    pub version: String,
    pub system_vars_count: usize,
    pub user_vars_count: usize,
    pub file_size: u64,
    pub created_time: String,
    pub modified_time: String,
}

// 扫描指定文件夹中的环境变量配置文件
#[command]
pub async fn scan_config_files(folder_path: String) -> Result<Vec<ConfigFileInfo>, String> {
    use std::fs;
    use std::path::Path;

    let folder = Path::new(&folder_path);
    if !folder.exists() || !folder.is_dir() {
        return Ok(Vec::new());
    }

    let mut config_files = Vec::new();

    // 读取文件夹中的所有 .json 文件
    let entries = fs::read_dir(folder).map_err(|e| format!("无法读取文件夹: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取文件项失败: {}", e))?;
        let path = entry.path();

        // 只处理 .json 文件
        if !path.is_file() || path.extension().unwrap_or_default() != "json" {
            continue;
        }

        // 尝试解析文件内容
        match parse_config_file(&path) {
            Ok(config_info) => config_files.push(config_info),
            Err(_) => continue, // 忽略无法解析的文件
        }
    }

    // 按导出时间排序（最新的在前面）
    config_files.sort_by(|a, b| b.export_time.cmp(&a.export_time));

    Ok(config_files)
}

// 解析配置文件信息
fn parse_config_file(file_path: &std::path::Path) -> Result<ConfigFileInfo, String> {
    use chrono::{DateTime, Utc};
    use std::fs;

    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // 读取文件内容
    let json_content = fs::read_to_string(file_path).map_err(|e| format!("读取文件失败: {}", e))?;

    // 首先尝试解析新格式（包含 export_info）
    let import_data: EnvVarExport =
        if let Ok(data) = serde_json::from_str::<EnvVarExport>(&json_content) {
            data
        } else {
            // 如果失败，尝试解析旧格式（直接是 system/user 变量）
            match serde_json::from_str::<serde_json::Value>(&json_content) {
                Ok(json_value) => {
                    // 旧格式：system/user 字段
                    if let (Some(system), Some(user)) = (
                        json_value
                            .get("system")
                            .and_then(|v| serde_json::from_value::<Vec<EnvVar>>(v.clone()).ok()),
                        json_value
                            .get("user")
                            .and_then(|v| serde_json::from_value::<Vec<EnvVar>>(v.clone()).ok()),
                    ) {
                        let export_time = json_value
                            .get("exportTime")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                // 尝试从文件名中提取时间戳
                                extract_timestamp_from_filename(&file_name)
                                    .unwrap_or_else(|| "未知".to_string())
                            });

                        let version = json_value
                            .get("appVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("1.0")
                            .to_string();

                        EnvVarExport {
                            export_info: ExportInfo {
                                export_time,
                                version,
                            },
                            system_vars: system,
                            user_vars: user,
                        }
                    } else {
                        return Err("无法解析环境变量数据".to_string());
                    }
                }
                Err(e) => return Err(format!("无法解析 JSON 文件: {}", e)),
            }
        };

    // 获取文件元数据
    let metadata = fs::metadata(file_path).map_err(|e| format!("获取文件元数据失败: {}", e))?;

    let file_size = metadata.len();

    // 格式化时间
    let created_time = metadata
        .created()
        .ok()
        .and_then(|time| {
            let datetime: Result<DateTime<Utc>, _> = time.try_into();
            datetime.ok()
        })
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "未知".to_string());

    let modified_time = metadata
        .modified()
        .ok()
        .and_then(|time| {
            let datetime: Result<DateTime<Utc>, _> = time.try_into();
            datetime.ok()
        })
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "未知".to_string());

    // 使用导出时间（简化格式）
    let export_time = import_data.export_info.export_time.clone();

    Ok(ConfigFileInfo {
        file_path: file_path.to_string_lossy().to_string(),
        file_name,
        export_time,
        version: import_data.export_info.version,
        system_vars_count: import_data.system_vars.len(),
        user_vars_count: import_data.user_vars.len(),
        file_size,
        created_time,
        modified_time,
    })
}

// 从文件名中提取时间戳
fn extract_timestamp_from_filename(filename: &str) -> Option<String> {
    // 匹配格式：环境变量备份_YYYY-MM-DDTHH-MM-SS.json
    if let Some(start) = filename.find("环境变量备份_") {
        let timestamp_part = &filename[start + "环境变量备份_".len()..];
        if let Some(end) = timestamp_part.find(".json") {
            let timestamp_str = &timestamp_part[..end];
            // 将时间戳格式转换为标准格式: YYYY-MM-DDTHH-MM-SS -> YYYY-MM-DD HH:MM:SS
            if timestamp_str.len() == 19 && timestamp_str.chars().nth(10) == Some('T') {
                let date_part = &timestamp_str[..10]; // YYYY-MM-DD
                let time_part = &timestamp_str[11..].replace('-', ":"); // HH:MM:SS
                return Some(format!("{} {}", date_part, time_part));
            }
        }
    }

    None
}
