use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct RecentProjectItem {
    pub label: String,
    pub path: String,
    pub kind: String, // folder / workspace
    pub mtime: Option<u64>,
}

#[tauri::command]
pub fn get_recent_vscode_projects(
    storage_path: Option<String>,
) -> Result<Vec<RecentProjectItem>, String> {
    // 若用户传入自定义路径则直接使用
    let storage_file = if let Some(custom) = storage_path {
        let p = PathBuf::from(&custom);
        if !p.exists() {
            return Err(format!("指定的 storage.json 不存在: {}", custom));
        }
        p
    } else {
        // VSCode recent 工作区信息：较新版在 globalStorage/storage.json, 旧说明在 User/storage.json
        let mut roaming = dirs::data_dir()
            .or_else(|| dirs::config_dir())
            .ok_or_else(|| "无法定位用户数据目录".to_string())?;
        roaming.push("Code");
        roaming.push("User");
        let candidate = {
            let mut p = roaming.clone();
            p.push("globalStorage");
            p.push("storage.json");
            if p.exists() {
                p
            } else {
                // 回退旧路径
                let mut legacy = roaming.clone();
                legacy.push("storage.json");
                legacy
            }
        };
        if !candidate.exists() {
            return Ok(vec![]);
        }
        candidate
    };

    let content = fs::read_to_string(&storage_file)
        .map_err(|e| format!("读取 VSCode storage.json 失败: {}", e))?;

    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 VSCode storage.json JSON 失败: {}", e))?;

    let mut items: Vec<RecentProjectItem> = Vec::new();

    // 1. workspaces3（如果有）
    if let Some(workspaces3) = json.get("workspaces3") {
        if let Some(recent) = workspaces3
            .get("recentWorkspaces")
            .or_else(|| workspaces3.get("recent"))
        {
            if let Some(arr) = recent.as_array() {
                for entry in arr {
                    if let Some(folder_uri) = entry.get("folderUri").and_then(|v| v.as_str()) {
                        if let Some(parsed) = decode_file_uri(folder_uri) {
                            push_item(&mut items, entry.get("label"), parsed, "folder");
                        }
                    } else if let Some(workspace) = entry.get("workspace") {
                        if let Some(config_path) =
                            workspace.get("configPath").and_then(|v| v.as_str())
                        {
                            if let Some(parsed) = decode_file_uri(config_path) {
                                push_item(&mut items, entry.get("label"), parsed, "workspace");
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. profileAssociations.workspaces (只包含 URI，没有 label)
    if let Some(profile_associations) = json.get("profileAssociations") {
        if let Some(workspaces) = profile_associations.get("workspaces") {
            if let Some(obj) = workspaces.as_object() {
                for (uri, _profile) in obj.iter() {
                    if uri.starts_with("file://") {
                        // 只处理本地
                        if let Some(parsed) = decode_file_uri(uri) {
                            push_item(&mut items, None, parsed, "folder");
                        }
                    }
                }
            }
        }
    }

    // 3. windowsState.lastActiveWindow.folder
    if let Some(windows_state) = json.get("windowsState") {
        if let Some(last_active) = windows_state.get("lastActiveWindow") {
            if let Some(folder_uri) = last_active.get("folder").and_then(|v| v.as_str()) {
                if folder_uri.starts_with("file://") {
                    if let Some(parsed) = decode_file_uri(folder_uri) {
                        push_item(&mut items, None, parsed, "folder");
                    }
                }
            }
        }
    }

    // 4. backupWorkspaces.folders 列表
    if let Some(backup) = json.get("backupWorkspaces") {
        if let Some(folders) = backup.get("folders") {
            if let Some(arr) = folders.as_array() {
                for entry in arr {
                    if let Some(folder_uri) = entry.get("folderUri").and_then(|v| v.as_str()) {
                        if let Some(parsed) = decode_file_uri(folder_uri) {
                            push_item(&mut items, None, parsed, "folder");
                        }
                    }
                }
            }
        }
    }

    // 排序：优先按 mtime 降序，其次按 label
    items.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.label.cmp(&b.label)));

    Ok(items)
}

#[tauri::command]
pub fn open_in_vscode(path: String) -> Result<(), String> {
    // 允许打开文件夹或 workspace 文件
    // 1. 先尝试 PATH 中的 code
    let candidates = collect_code_candidates();
    let mut last_err: Option<String> = None;
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);
        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 VSCode 失败: 未找到 code 可执行文件。尝试过: {}{}",
        candidates.join(", "),
        last_err
            .map(|e| format!("; 最后错误: {}", e))
            .unwrap_or_default()
    ))
}

fn collect_code_candidates() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    // 直接使用 PATH 中的名称
    list.push("code".to_string());
    list.push("code.cmd".to_string());
    list.push("code.exe".to_string());

    // 常见安装目录 (User 安装)
    if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
        let mut p = PathBuf::from(&local_app);
        p.push("Programs");
        p.push("Microsoft VS Code");
        p.push("Code.exe");
        list.push(p.to_string_lossy().to_string());
    }
    // System-wide Program Files
    if let Some(pf) = std::env::var_os("ProgramFiles") {
        let mut p = PathBuf::from(&pf);
        p.push("Microsoft VS Code");
        p.push("Code.exe");
        list.push(p.to_string_lossy().to_string());
    }
    if let Some(pf86) = std::env::var_os("ProgramFiles(x86)") {
        let mut p = PathBuf::from(&pf86);
        p.push("Microsoft VS Code");
        p.push("Code.exe");
        list.push(p.to_string_lossy().to_string());
    }

    // 去重，保持顺序
    let mut seen = std::collections::HashSet::new();
    list.retain(|p| seen.insert(p.clone()));
    list
}

fn decode_file_uri(uri: &str) -> Option<PathBuf> {
    // 只处理 file:/// 开头
    let lower = uri.to_lowercase();
    if !lower.starts_with("file://") {
        return None;
    }
    // 去除 file:// 前缀（通常有三个斜杠）
    let without_scheme = uri.trim_start_matches("file://");
    // URL 解码
    let decoded = urlencoding::decode(without_scheme).ok()?;
    // Windows 路径可能以 /c:/ 开头
    let mut path_str = decoded.to_string();
    if path_str.starts_with('/') && path_str.chars().nth(2) == Some(':') {
        // /c:/...
        path_str.remove(0); // 移除最前面 '/'
    }
    Some(PathBuf::from(path_str.replace(
        '/',
        std::path::MAIN_SEPARATOR.to_string().as_str(),
    )))
}

fn get_mtime(path: &PathBuf) -> Option<u64> {
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(time) = meta.modified() {
            if let Ok(duration) = time.duration_since(std::time::UNIX_EPOCH) {
                return Some(duration.as_secs());
            }
        }
    }
    None
}

fn push_item(
    items: &mut Vec<RecentProjectItem>,
    label_value: Option<&serde_json::Value>,
    path: PathBuf,
    kind: &str,
) {
    let label = label_value
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| infer_label(&path));
    // 去重：如果已有同路径则忽略
    if items.iter().any(|it| it.path == path.to_string_lossy()) {
        return;
    }
    items.push(RecentProjectItem {
        label,
        path: path.to_string_lossy().to_string(),
        kind: kind.to_string(),
        mtime: get_mtime(&path),
    });
}

fn infer_label(path: &PathBuf) -> String {
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        name.to_string()
    } else {
        path.to_string_lossy().to_string()
    }
}
