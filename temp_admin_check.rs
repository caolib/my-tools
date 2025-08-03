#[command]
pub fn check_admin_privileges() -> Result<bool, String> {
    // 尝试以写权限打开系统环境变量注册表键来检测管理员权限
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    match hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_WRITE) {
        Ok(_) => Ok(true),  // 可以打开，说明有管理员权限
        Err(_) => Ok(false), // 无法打开，说明没有管理员权限
    }
}
