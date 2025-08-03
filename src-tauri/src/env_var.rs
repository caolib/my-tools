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
            winreg::RegValue { vtype: REG_SZ, .. } |
            winreg::RegValue { vtype: REG_EXPAND_SZ, .. } => {
                env_key.get_value::<String, _>(&name).unwrap_or_default()
            },
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
            winreg::RegValue { vtype: REG_SZ, .. } |
            winreg::RegValue { vtype: REG_EXPAND_SZ, .. } => {
                env_key.get_value::<String, _>(&name).unwrap_or_default()
            },
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
            .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_WRITE)
            .map_err(|e| format!("无法打开系统环境变量注册表进行写入（可能需要管理员权限）: {}", e))?;
        
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
            .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_WRITE)
            .map_err(|e| format!("无法打开系统环境变量注册表进行写入（可能需要管理员权限）: {}", e))?;
        
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
    
    match hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_WRITE) {
        Ok(_) => Ok(true),  // 可以打开，说明有管理员权限
        Err(_) => Ok(false), // 无法打开，说明没有管理员权限
    }
}

// 请求管理员权限重启应用
#[command]
pub fn request_admin_privileges() -> Result<(), String> {
    use std::process::Command;
    use std::env;
    
    // 获取当前执行文件路径
    let current_exe = env::current_exe()
        .map_err(|e| format!("无法获取当前执行文件路径: {}", e))?;
    
    // 使用 PowerShell 以管理员身份重启应用
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Start-Process -FilePath '{}' -Verb RunAs", current_exe.display())
        ])
        .output()
        .map_err(|e| format!("无法启动管理员权限进程: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("启动管理员权限进程失败: {}", stderr));
    }
    
    // 关闭当前进程
    std::process::exit(0);
}
