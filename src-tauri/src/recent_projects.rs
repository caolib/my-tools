use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Serialize)]
pub struct RecentProjectItem {
    pub label: String,
    pub path: String,
    pub kind: String, // folder / workspace
    pub mtime: Option<u64>,
    pub source: String, // vscode | trae | qoder | idea
}

#[tauri::command]
pub fn get_recent_projects(
    vscode_storage_path: Option<String>,
    trae_storage_path: Option<String>,
    qoder_storage_path: Option<String>,
    idea_storage_path: Option<String>,
    webstorm_storage_path: Option<String>,
    pycharm_storage_path: Option<String>,
) -> Result<Vec<RecentProjectItem>, String> {
    println!(
        "[get_recent_vscode_projects] Starting with idea_storage_path: {:?}, webstorm_storage_path: {:?}, pycharm_storage_path: {:?}",
        idea_storage_path, webstorm_storage_path, pycharm_storage_path
    );

    // 收集 VSCode 主 storage.json (可能来自用户自定义 or 自动推断)
    let vscode_storage = if let Some(custom) = vscode_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!("指定的 storage.json 不存在: {}", custom));
        }
    } else {
        let mut roaming = dirs::data_dir()
            .or_else(|| dirs::config_dir())
            .ok_or_else(|| "无法定位用户数据目录".to_string())?;
        roaming.push("Code");
        roaming.push("User");
        let mut p = roaming.clone();
        p.push("globalStorage");
        p.push("storage.json");
        if !p.exists() {
            // 回退旧路径
            p = roaming.clone();
            p.push("storage.json");
        }
        if p.exists() {
            Some(p)
        } else {
            None
        }
    };

    let mut items: Vec<RecentProjectItem> = Vec::new();

    if let Some(vs_path) = vscode_storage {
        println!("[recent_projects] VSCode storage: {}", vs_path.display());
        if let Ok(content) = fs::read_to_string(&vs_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let before = items.len();
                parse_editor_json(&json, "vscode", &mut items);
                println!(
                    "[recent_projects] VSCode parsed added {} items (total {}).",
                    items.len() - before,
                    items.len()
                );
            }
        }
    }

    // Trae 编辑器路径 (与 VSCode 目录结构类似: Roaming/Trae/User/...)
    // Trae: 优先使用传入的 trae_storage_path，否则自动推断
    let trae_path_opt: Option<PathBuf> = if let Some(custom) = trae_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!("指定的 Trae storage.json 不存在: {}", custom));
        }
    } else {
        if let Some(mut roaming) = dirs::data_dir().or_else(|| dirs::config_dir()) {
            roaming.push("Trae");
            roaming.push("User");
            let mut p = roaming.clone();
            p.push("globalStorage");
            p.push("storage.json");
            if !p.exists() {
                p = roaming.clone();
                p.push("storage.json");
            }
            if p.exists() {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(trae_path) = trae_path_opt {
        println!("[recent_projects] Trae storage: {}", trae_path.display());
        if let Ok(content) = fs::read_to_string(&trae_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let before = items.len();
                parse_editor_json(&json, "trae", &mut items);
                println!(
                    "[recent_projects] Trae parsed added {} items (total {}).",
                    items.len() - before,
                    items.len()
                );
            }
        }
    }

    // Qoder 编辑器路径 (与 VSCode 目录结构类似: Roaming/Qoder/User/...)
    let qoder_path_opt: Option<PathBuf> = if let Some(custom) = qoder_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!("指定的 Qoder storage.json 不存在: {}", custom));
        }
    } else {
        if let Some(mut roaming) = dirs::data_dir().or_else(|| dirs::config_dir()) {
            roaming.push("Qoder");
            roaming.push("User");
            let mut p = roaming.clone();
            p.push("globalStorage");
            p.push("storage.json");
            if !p.exists() {
                p = roaming.clone();
                p.push("storage.json");
            }
            if p.exists() {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(qoder_path) = qoder_path_opt {
        println!("[recent_projects] Qoder storage: {}", qoder_path.display());
        if let Ok(content) = fs::read_to_string(&qoder_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let before = items.len();
                parse_editor_json(&json, "qoder", &mut items);
                println!(
                    "[recent_projects] Qoder parsed added {} items (total {}).",
                    items.len() - before,
                    items.len()
                );
            }
        }
    }

    // IntelliJ IDEA 项目扫描 (recentProjects.xml)
    let idea_path_opt: Option<PathBuf> = if let Some(custom) = idea_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!("指定的 IDEA recentProjects.xml 不存在: {}", custom));
        }
    } else {
        // 自动搜索 IDEA 配置目录
        find_idea_recent_projects_xml()
    };

    if let Some(idea_path) = idea_path_opt {
        println!(
            "[recent_projects] IDEA recentProjects.xml: {}",
            idea_path.display()
        );
        if let Ok(content) = fs::read_to_string(&idea_path) {
            let before = items.len();
            parse_jetbrains_xml(&content, "idea", &mut items);
            println!(
                "[recent_projects] IDEA parsed added {} items (total {}).",
                items.len() - before,
                items.len()
            );
        }
    }

    // WebStorm 项目扫描 (recentProjects.xml)
    let webstorm_path_opt: Option<PathBuf> = if let Some(custom) = webstorm_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!(
                "指定的 WebStorm recentProjects.xml 不存在: {}",
                custom
            ));
        }
    } else {
        // 自动搜索 WebStorm 配置目录
        find_webstorm_recent_projects_xml()
    };

    if let Some(webstorm_path) = webstorm_path_opt {
        println!(
            "[recent_projects] WebStorm recentProjects.xml: {}",
            webstorm_path.display()
        );
        if let Ok(content) = fs::read_to_string(&webstorm_path) {
            let before = items.len();
            parse_jetbrains_xml(&content, "webstorm", &mut items);
            println!(
                "[recent_projects] WebStorm parsed added {} items (total {}).",
                items.len() - before,
                items.len()
            );
        }
    }

    // PyCharm 项目扫描 (recentProjects.xml)
    let pycharm_path_opt: Option<PathBuf> = if let Some(custom) = pycharm_storage_path.clone() {
        let p = PathBuf::from(&custom);
        if p.exists() {
            Some(p)
        } else {
            return Err(format!(
                "指定的 PyCharm recentProjects.xml 不存在: {}",
                custom
            ));
        }
    } else {
        // 自动搜索 PyCharm 配置目录
        find_pycharm_recent_projects_xml()
    };

    if let Some(pycharm_path) = pycharm_path_opt {
        println!(
            "[recent_projects] PyCharm recentProjects.xml: {}",
            pycharm_path.display()
        );
        if let Ok(content) = fs::read_to_string(&pycharm_path) {
            let before = items.len();
            parse_jetbrains_xml(&content, "pycharm", &mut items);
            println!(
                "[recent_projects] PyCharm parsed added {} items (total {}).",
                items.len() - before,
                items.len()
            );
        }
    }

    // 排序：mtime DESC -> source -> label
    items.sort_by(|a, b| {
        b.mtime
            .cmp(&a.mtime)
            .then_with(|| a.source.cmp(&b.source))
            .then_with(|| a.label.cmp(&b.label))
    });
    Ok(items)
}

#[tauri::command]
pub fn open_in_vscode(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        // 如果传入的是目录，尝试补全 Code.exe / code.exe
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("Code.exe").to_string_lossy().to_string(),
                pb.join("code.exe").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_code_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_vscode] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

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

#[tauri::command]
pub fn open_in_trae(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("Trae.exe").to_string_lossy().to_string(),
                pb.join("trae.exe").to_string_lossy().to_string(),
                pb.join("trae.cmd").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_trae_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_trae] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 Trae 失败: 未找到 trae 可执行文件。尝试过: {}{}",
        candidates.join(", "),
        last_err
            .map(|e| format!("; 最后错误: {}", e))
            .unwrap_or_default()
    ))
}

fn collect_trae_candidates() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    list.push("trae".to_string());
    list.push("trae.exe".to_string());
    list.push("trae.cmd".to_string());
    if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
        let mut p = PathBuf::from(&local_app);
        p.push("Programs");
        p.push("Trae");
        p.push("Trae.exe");
        list.push(p.to_string_lossy().to_string());
    }
    if let Some(pf) = std::env::var_os("ProgramFiles") {
        let mut p = PathBuf::from(&pf);
        p.push("Trae");
        p.push("Trae.exe");
        list.push(p.to_string_lossy().to_string());
    }
    if let Some(pf86) = std::env::var_os("ProgramFiles(x86)") {
        let mut p = PathBuf::from(&pf86);
        p.push("Trae");
        p.push("Trae.exe");
        list.push(p.to_string_lossy().to_string());
    }
    let mut seen = std::collections::HashSet::new();
    list.retain(|p| seen.insert(p.clone()));
    list
}

fn collect_qoder_candidates() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    list.push("qoder".to_string());
    list.push("qoder.exe".to_string());
    list.push("qoder.cmd".to_string());
    if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
        let mut p = PathBuf::from(&local_app);
        p.push("Programs");
        p.push("Qoder");
        p.push("Qoder.exe");
        list.push(p.to_string_lossy().to_string());
    }
    if let Some(pf) = std::env::var_os("ProgramFiles") {
        let mut p = PathBuf::from(&pf);
        p.push("Qoder");
        p.push("Qoder.exe");
        list.push(p.to_string_lossy().to_string());
    }
    if let Some(pf86) = std::env::var_os("ProgramFiles(x86)") {
        let mut p = PathBuf::from(&pf86);
        p.push("Qoder");
        p.push("Qoder.exe");
        list.push(p.to_string_lossy().to_string());
    }
    let mut seen = std::collections::HashSet::new();
    list.retain(|p| seen.insert(p.clone()));
    list
}

#[tauri::command]
pub fn open_in_qoder(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("Qoder.exe").to_string_lossy().to_string(),
                pb.join("qoder.exe").to_string_lossy().to_string(),
                pb.join("qoder.cmd").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_qoder_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_qoder] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 Qoder 失败: 未找到 qoder 可执行文件。尝试过: {}{}",
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
    let lower = uri.to_lowercase();
    if !lower.starts_with("file://") {
        return None;
    }
    let without_scheme = uri.trim_start_matches("file://");
    let decoded = urlencoding::decode(without_scheme).ok()?;
    let mut path_str = decoded.to_string();
    if path_str.starts_with('/') && path_str.chars().nth(2) == Some(':') {
        path_str.remove(0);
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
    source: &str,
) {
    let label = label_value
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| infer_label(&path));
    // 仅在同一来源 & 同一路径已存在时跳过，允许不同来源并存
    if items
        .iter()
        .any(|it| it.path == path.to_string_lossy() && it.source == source)
    {
        return;
    }
    items.push(RecentProjectItem {
        label,
        path: path.to_string_lossy().to_string(),
        kind: kind.to_string(),
        mtime: get_mtime(&path),
        source: source.to_string(),
    });
}

fn parse_editor_json(json: &serde_json::Value, source: &str, items: &mut Vec<RecentProjectItem>) {
    // workspaces3
    if let Some(workspaces3) = json.get("workspaces3") {
        if let Some(recent) = workspaces3
            .get("recentWorkspaces")
            .or_else(|| workspaces3.get("recent"))
        {
            if let Some(arr) = recent.as_array() {
                for entry in arr {
                    if let Some(folder_uri) = entry.get("folderUri").and_then(|v| v.as_str()) {
                        if let Some(parsed) = decode_file_uri(folder_uri) {
                            push_item(items, entry.get("label"), parsed, "folder", source);
                        }
                    } else if let Some(workspace) = entry.get("workspace") {
                        if let Some(config_path) =
                            workspace.get("configPath").and_then(|v| v.as_str())
                        {
                            if let Some(parsed) = decode_file_uri(config_path) {
                                push_item(items, entry.get("label"), parsed, "workspace", source);
                            }
                        }
                    }
                }
            }
        }
    }
    // profileAssociations
    if let Some(profile_associations) = json.get("profileAssociations") {
        if let Some(workspaces) = profile_associations.get("workspaces") {
            if let Some(obj) = workspaces.as_object() {
                for (uri, _) in obj.iter() {
                    if uri.starts_with("file://") {
                        if let Some(parsed) = decode_file_uri(uri) {
                            push_item(items, None, parsed, "folder", source);
                        }
                    }
                }
            }
        }
    }
    // windowsState
    if let Some(windows_state) = json.get("windowsState") {
        if let Some(last_active) = windows_state.get("lastActiveWindow") {
            if let Some(folder_uri) = last_active.get("folder").and_then(|v| v.as_str()) {
                if folder_uri.starts_with("file://") {
                    if let Some(parsed) = decode_file_uri(folder_uri) {
                        push_item(items, None, parsed, "folder", source);
                    }
                }
            }
        }
        // Trae: maybe there is an openedWindows array with objects containing folder/workspace
        if let Some(opened) = windows_state
            .get("openedWindows")
            .and_then(|v| v.as_array())
        {
            for w in opened {
                if let Some(folder_uri) = w.get("folder").and_then(|v| v.as_str()) {
                    if folder_uri.starts_with("file://") {
                        if let Some(parsed) = decode_file_uri(folder_uri) {
                            push_item(items, None, parsed, "folder", source);
                        }
                    }
                }
                if let Some(workspace) = w.get("workspace") {
                    if let Some(config_path) = workspace.get("configPath").and_then(|v| v.as_str())
                    {
                        if let Some(parsed) = decode_file_uri(config_path) {
                            push_item(items, None, parsed, "workspace", source);
                        }
                    }
                }
            }
        }
    }
    // backupWorkspaces
    if let Some(backup) = json.get("backupWorkspaces") {
        if let Some(folders) = backup.get("folders") {
            if let Some(arr) = folders.as_array() {
                for entry in arr {
                    if let Some(folder_uri) = entry.get("folderUri").and_then(|v| v.as_str()) {
                        if let Some(parsed) = decode_file_uri(folder_uri) {
                            push_item(items, None, parsed, "folder", source);
                        }
                    }
                }
            }
        }
    }

    // Additional structures possibly used by Trae / forks
    // openedPathsList: array of objects { folderUri?, workspace? } similar to workspaces3.recent
    if let Some(opened_paths) = json.get("openedPathsList") {
        if let Some(arr) = opened_paths.as_array() {
            for entry in arr {
                if let Some(folder_uri) = entry.get("folderUri").and_then(|v| v.as_str()) {
                    if let Some(parsed) = decode_file_uri(folder_uri) {
                        push_item(items, entry.get("label"), parsed, "folder", source);
                    }
                } else if let Some(workspace) = entry.get("workspace") {
                    if let Some(config_path) = workspace.get("configPath").and_then(|v| v.as_str())
                    {
                        if let Some(parsed) = decode_file_uri(config_path) {
                            push_item(items, entry.get("label"), parsed, "workspace", source);
                        }
                    }
                }
            }
        }
    }
    // recentRoots: array of file:// URIs
    if let Some(recent_roots) = json.get("recentRoots") {
        if let Some(arr) = recent_roots.as_array() {
            for v in arr {
                if let Some(uri) = v.as_str() {
                    if let Some(parsed) = decode_file_uri(uri) {
                        push_item(items, None, parsed, "folder", source);
                    }
                }
            }
        }
    }
}

fn infer_label(path: &PathBuf) -> String {
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        name.to_string()
    } else {
        path.to_string_lossy().to_string()
    }
}

/// 解析 JetBrains 系列编辑器的 recentProjects.xml 文件
fn parse_jetbrains_xml(content: &str, source: &str, items: &mut Vec<RecentProjectItem>) {
    // JetBrains 系列编辑器的XML格式：
    // <component name="RecentProjectsManager">
    //   <option name="additionalInfo">
    //     <map>
    //       <entry key="C:/code/Java/project-name">
    //         <value>...</value>
    //       </entry>
    //     </map>
    //   </option>
    // </component>

    // 查找 additionalInfo 部分
    if let Some(additional_info_start) = content.find(r#"<option name="additionalInfo">"#) {
        if let Some(map_start) = content[additional_info_start..].find("<map>") {
            let map_start_pos = additional_info_start + map_start + 5; // 5 = "<map>".len()
            if let Some(map_end) = content[map_start_pos..].find("</map>") {
                let map_end_pos = map_start_pos + map_end;
                let map_content = &content[map_start_pos..map_end_pos];

                // 查找所有 <entry key="..." > 标签
                let mut search_pos = 0;
                while let Some(entry_start) = map_content[search_pos..].find(r#"<entry key=""#) {
                    let absolute_pos = search_pos + entry_start + 12; // 12 = r#"<entry key=""#.len()
                    if let Some(quote_end) = map_content[absolute_pos..].find('"') {
                        let path_str = &map_content[absolute_pos..absolute_pos + quote_end];

                        // 标准化路径，与其他编辑器保持一致
                        let mut path_buf = PathBuf::from(path_str);

                        // 将路径标准化为Windows格式（如果需要）
                        let normalized_path = path_buf
                            .to_string_lossy()
                            .replace('/', &std::path::MAIN_SEPARATOR.to_string());
                        path_buf = PathBuf::from(normalized_path);

                        if path_buf.exists() {
                            let label = infer_label(&path_buf);
                            let mtime = path_buf
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs());

                            items.push(RecentProjectItem {
                                label,
                                path: path_buf.to_string_lossy().to_string(),
                                kind: "folder".to_string(),
                                mtime,
                                source: source.to_string(),
                            });
                        }

                        search_pos = absolute_pos + quote_end + 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
#[tauri::command]
pub fn open_in_idea(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("idea64.exe").to_string_lossy().to_string(),
                pb.join("idea.exe").to_string_lossy().to_string(),
                pb.join("idea.cmd").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_idea_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_idea] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 IntelliJ IDEA 失败: 未找到 idea 可执行文件。尝试过: {}{}",
        candidates.join(", "),
        last_err
            .map(|e| format!("; 最后错误: {}", e))
            .unwrap_or_default()
    ))
}

#[tauri::command]
pub fn open_in_pycharm(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("pycharm64.exe").to_string_lossy().to_string(),
                pb.join("pycharm.exe").to_string_lossy().to_string(),
                pb.join("pycharm.cmd").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_pycharm_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_pycharm] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 PyCharm 失败: 未找到 pycharm 可执行文件。尝试过: {}{}",
        candidates.join(", "),
        last_err
            .map(|e| format!("; 最后错误: {}", e))
            .unwrap_or_default()
    ))
}

#[tauri::command]
pub fn open_in_webstorm(path: String, exe_path: Option<String>) -> Result<(), String> {
    let candidates = if let Some(custom) = exe_path {
        let pb = std::path::Path::new(&custom);
        if pb.is_dir() {
            vec![
                pb.join("webstorm64.exe").to_string_lossy().to_string(),
                pb.join("webstorm.exe").to_string_lossy().to_string(),
                pb.join("webstorm.cmd").to_string_lossy().to_string(),
            ]
        } else {
            vec![custom]
        }
    } else {
        collect_webstorm_candidates()
    };
    let mut last_err: Option<String> = None;
    println!("[open_in_webstorm] try candidates: {:?}", candidates);
    for cand in &candidates {
        let mut cmd = std::process::Command::new(cand);
        cmd.arg(&path);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(format!("{} -> {}", cand, e));
                continue;
            }
        }
    }
    Err(format!(
        "启动 WebStorm 失败: 未找到 webstorm 可执行文件。尝试过: {}{}",
        candidates.join(", "),
        last_err
            .map(|e| format!("; 最后错误: {}", e))
            .unwrap_or_default()
    ))
}

fn collect_jetbrains_candidates(product_name: &str, exe_name: &str) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    // 常见的 JetBrains 产品安装路径
    let common_paths = [
        format!(
            "C:/Program Files/JetBrains/{} Community Edition",
            product_name
        ),
        format!("C:/Program Files/JetBrains/{} Ultimate", product_name),
        format!("C:/Program Files/JetBrains/{}", product_name),
        format!(
            "C:/Program Files (x86)/JetBrains/{} Community Edition",
            product_name
        ),
        format!("C:/Program Files (x86)/JetBrains/{} Ultimate", product_name),
        format!("C:/Program Files (x86)/JetBrains/{}", product_name),
    ];

    for base in &common_paths {
        // 查找各种版本目录
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    let bin_path = entry.path().join("bin");
                    let exe64 = bin_path.join(format!("{}64.exe", exe_name));
                    let exe = bin_path.join(format!("{}.exe", exe_name));

                    if exe64.exists() {
                        list.push(exe64.to_string_lossy().to_string());
                    }
                    if exe.exists() {
                        list.push(exe.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    // 也尝试 PATH 中的名称
    list.push(format!("{}64", exe_name));
    list.push(exe_name.to_string());
    list.push(format!("{}64.exe", exe_name));
    list.push(format!("{}.exe", exe_name));
    list.push(format!("{}.cmd", exe_name));

    list
}

fn collect_idea_candidates() -> Vec<String> {
    collect_jetbrains_candidates("IntelliJ IDEA", "idea")
}

fn collect_webstorm_candidates() -> Vec<String> {
    collect_jetbrains_candidates("WebStorm", "webstorm")
}

fn collect_pycharm_candidates() -> Vec<String> {
    collect_jetbrains_candidates("PyCharm", "pycharm")
}

/// 查找 JetBrains 系列编辑器的 recentProjects.xml 文件
fn find_jetbrains_recent_projects_xml(product_prefix: &str) -> Option<PathBuf> {
    // 在用户数据目录中搜索 JetBrains/{ProductPrefix}* 目录
    if let Some(mut data_dir) = dirs::data_dir().or_else(|| dirs::config_dir()) {
        data_dir.push("JetBrains");
        if data_dir.exists() {
            if let Ok(entries) = fs::read_dir(&data_dir) {
                for entry in entries.flatten() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    // 匹配 IntelliJIdea2023.1, WebStorm2024.2, etc.
                    if dir_name.starts_with(product_prefix) {
                        let mut recent_projects_path = entry.path();
                        recent_projects_path.push("options");
                        recent_projects_path.push("recentProjects.xml");
                        if recent_projects_path.exists() {
                            println!(
                                "[find_jetbrains_recent_projects_xml] Found {}: {}",
                                product_prefix,
                                recent_projects_path.display()
                            );
                            return Some(recent_projects_path);
                        }
                    }
                }
            }
        }
    }
    println!(
        "[find_jetbrains_recent_projects_xml] No {} config found",
        product_prefix
    );
    None
}

/// 查找 IDEA 的 recentProjects.xml 文件
fn find_idea_recent_projects_xml() -> Option<PathBuf> {
    find_jetbrains_recent_projects_xml("IntelliJIdea")
}

/// 查找 WebStorm 的 recentProjects.xml 文件
fn find_webstorm_recent_projects_xml() -> Option<PathBuf> {
    find_jetbrains_recent_projects_xml("WebStorm")
}

/// 查找 PyCharm 的 recentProjects.xml 文件
fn find_pycharm_recent_projects_xml() -> Option<PathBuf> {
    find_jetbrains_recent_projects_xml("PyCharm")
}
