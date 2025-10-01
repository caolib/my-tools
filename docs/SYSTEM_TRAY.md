# 系统托盘功能

## 功能概述

应用支持系统托盘功能，提供快速访问和窗口管理能力。

## 核心功能

### 1. 托盘图标交互

#### 🖱️ 左键单击
- **功能**：快速切换窗口显示/隐藏
- **行为**：
  - 窗口显示时 → 隐藏到托盘
  - 窗口隐藏时 → 显示并聚焦

#### 🖱️ 右键单击
- **功能**：显示功能菜单
- **菜单项**：
  - 🗂️ **环境变量管理**：打开并跳转到环境变量管理界面
  - 🔍 **文件搜索**：打开并跳转到文件搜索界面
  - 📁 **项目管理**：打开并跳转到项目管理界面
  - ⚙️ **设置**：打开设置界面
  - ━━━━━━━━━━━━━━
  - ❌ **退出应用**：完全关闭应用程序

### 2. 窗口关闭行为设置

- **设置选项**：`点击关闭按钮时的行为`
  - 位置：设置 → 全局设置
  - 三个选项：
    1. **最小化到托盘**（默认）：关闭后隐藏到系统托盘，可通过托盘图标恢复
    2. **退出应用**：直接退出应用，完全关闭
    3. **无动作**：不响应关闭操作，窗口保持打开

- **行为逻辑**：
  - `closeButtonBehavior = 'minimize'`（默认）：点击关闭按钮 → 隐藏到托盘
  - `closeButtonBehavior = 'quit'`：点击关闭按钮 → 直接退出应用
  - `closeButtonBehavior = 'none'`：点击关闭按钮 → 无动作，窗口保持打开
  - 托盘菜单"退出应用"：无论设置如何，都完全退出应用

## 快捷键显示

托盘菜单会自动显示已设置的全局快捷键。例如：
- 环境变量管理 `Ctrl+Shift+E`
- 文件搜索 `Ctrl+Shift+F`
- 项目管理 `Ctrl+Shift+P`

设置或清除快捷键后，托盘菜单会自动更新。

## 使用场景

### 场景 1：快速隐藏窗口
- 左键单击托盘图标，窗口立即隐藏
- 或点击窗口关闭按钮（默认设置下）

### 场景 2：从托盘恢复窗口
- 左键单击托盘图标，窗口立即显示并聚焦

### 场景 3：快速切换功能
1. 右键单击托盘图标
2. 选择对应功能（如"环境变量管理"）
3. 窗口自动显示并跳转到该界面
4. 菜单中会显示该功能的全局快捷键（如果已设置）

### 场景 4：完全退出应用
- 右键托盘 → 点击"退出应用"
- 或在设置中将关闭按钮行为改为"退出应用"后点击关闭按钮

### 场景 5：防止误关闭
- 在设置中将关闭按钮行为改为"无动作"
- 点击关闭按钮不会有任何响应
- 必须通过托盘菜单的"退出应用"来关闭

## 技术实现

### 前端实现

#### 1. 状态管理 (`src/stores/settings.js`)

```javascript
// 窗口关闭行为设置：'minimize'（最小化到托盘，默认）、'quit'（退出应用）、'none'（无动作）
closeButtonBehavior: 'minimize',

// 重置默认值
resetToDefaults() {
    this.closeButtonBehavior = 'minimize'
    // ...
}
```

#### 2. 窗口关闭事件处理 (`src/App.vue`)

```javascript
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useSettingsStore } from './stores/settings'

const currentWindow = getCurrentWindow()
const settingsStore = useSettingsStore()

// 监听窗口关闭请求事件
const unlistenCloseRequested = await currentWindow.onCloseRequested(async (event) => {
  const { closeToTray } = settingsStore
  
  if (closeToTray) {
    // 最小化到托盘
    event.preventDefault()
    await currentWindow.hide()
  }
  // 如果 closeToTray 为 false，则让窗口正常关闭
})

// 组件卸载时清理监听器
onUnmounted(() => {
    unlistenCloseRequested?.()
})
```

#### 3. 设置界面 (`src/views/Settings.vue`)

**关闭行为设置**：
```vue
<el-form-item>
    <template #label>
        <span>点击关闭按钮时的行为</span>
        <el-tooltip placement="top">
            <template #content>
                <div>最小化到托盘：关闭后隐藏到系统托盘</div>
                <div>退出应用：直接关闭应用</div>
                <div>无动作：不响应关闭操作</div>
            </template>
            <el-icon style="margin-left: 4px; cursor: help;">
                <QuestionFilled />
            </el-icon>
        </el-tooltip>
    </template>
    <el-radio-group v-model="settingsStore.closeButtonBehavior">
        <el-radio value="minimize">最小化到托盘</el-radio>
        <el-radio value="quit">退出应用</el-radio>
        <el-radio value="none">无动作</el-radio>
    </el-radio-group>
</el-form-item>
```

**更新托盘菜单显示快捷键**：
```javascript
// 更新托盘菜单显示快捷键
const updateTrayMenu = async () => {
    try {
        await invoke('update_tray_shortcuts', {
            envVarManager: shortcuts.value.envVarManager || '',
            fileSearch: shortcuts.value.fileSearch || '',
            projects: shortcuts.value.projects || ''
        })
    } catch (error) {
        console.warn('更新托盘菜单失败:', error)
    }
}

// 在设置快捷键后调用
const saveShortcut = async (key) => {
    // ... 设置快捷键逻辑
    settingsStore.setGlobalShortcut(key, shortcut)
    
    // 更新托盘菜单
    await updateTrayMenu()
    
    ElMessage.success(`快捷键 ${shortcut} 设置成功`)
}

// 在清除快捷键后调用
const clearShortcut = async (key) => {
    // ... 清除快捷键逻辑
    settingsStore.clearGlobalShortcut(key)
    
    // 更新托盘菜单
    await updateTrayMenu()
    
    ElMessage.success('快捷键已清除')
}

// 页面加载时更新托盘菜单
onMounted(async () => {
    // ...
    await updateTrayMenu()
})
```

### 后端实现

#### 1. 依赖配置 (`src-tauri/Cargo.toml`)

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
```

#### 2. 状态管理 (`src-tauri/src/lib.rs`)

```rust
// 全局快捷键信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShortcutInfo {
    env_var_manager: String,
    file_search: String,
    projects: String,
}

// 托盘图标状态
struct TrayState {
    shortcuts: Mutex<ShortcutInfo>,
}
```

#### 3. 动态更新托盘菜单 (`src-tauri/src/lib.rs`)

```rust
#[tauri::command]
fn update_tray_shortcuts(
    app: tauri::AppHandle,
    state: tauri::State<TrayState>,
    env_var_manager: String,
    file_search: String,
    projects: String,
) -> Result<(), String> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};

    // 更新状态
    if let Ok(mut shortcuts) = state.shortcuts.lock() {
        shortcuts.env_var_manager = env_var_manager.clone();
        shortcuts.file_search = file_search.clone();
        shortcuts.projects = projects.clone();
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

    // 重建托盘菜单
    let env_item = MenuItemBuilder::with_id("env", env_text)
        .build(&app)
        .map_err(|e| e.to_string())?;
    let file_search_item = MenuItemBuilder::with_id("file-search", file_search_text)
        .build(&app)
        .map_err(|e| e.to_string())?;
    let projects_item = MenuItemBuilder::with_id("projects", projects_text)
        .build(&app)
        .map_err(|e| e.to_string())?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置")
        .build(&app)
        .map_err(|e| e.to_string())?;

    let separator = PredefinedMenuItem::separator(&app).map_err(|e| e.to_string())?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出应用")
        .build(&app)
        .map_err(|e| e.to_string())?;

    let menu = MenuBuilder::new(&app)
        .items(&[
            &env_item,
            &file_search_item,
            &projects_item,
            &settings_item,
            &separator,
            &quit_item,
        ])
        .build()
        .map_err(|e| e.to_string())?;

    // 更新托盘菜单
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }

    Ok(())
}
```

#### 4. 托盘功能实现 (`src-tauri/src/lib.rs`)

```rust
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建托盘菜单项
            let env_item = MenuItemBuilder::with_id("env", "环境变量管理").build(app)?;
            let file_search_item = MenuItemBuilder::with_id("file-search", "文件搜索").build(app)?;
            let projects_item = MenuItemBuilder::with_id("projects", "项目管理").build(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;

            // 分隔符
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出应用").build(app)?;

            // 构建完整菜单
            let menu = MenuBuilder::new(app)
                .items(&[
                    &env_item,
                    &file_search_item,
                    &projects_item,
                    &settings_item,
                    &separator,
                    &quit_item
                ])
                .build()?;

            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)  // 关键：禁用左键显示菜单
                // 处理菜单点击事件
                .on_menu_event(move |app, event| {
                    let window = app.get_webview_window("main");
                    
                    match event.id().as_ref() {
                        "env" => {
                            if let Some(window) = window {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.eval("window.location.hash = '#/env'");
                            }
                        }
                        "file-search" => {
                            if let Some(window) = window {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.eval("window.location.hash = '#/file-search'");
                            }
                        }
                        "projects" => {
                            if let Some(window) = window {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.eval("window.location.hash = '#/projects'");
                            }
                        }
                        "settings" => {
                            if let Some(window) = window {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.eval("window.location.hash = '#/settings'");
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                // 处理托盘图标点击事件（左键切换显示/隐藏）
                .on_tray_icon_event(|tray, event| {
                    // 只处理左键单击的按键抬起事件
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![...])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 3. 权限配置 (`src-tauri/capabilities/default.json`)

```json
{
  "permissions": [
    "core:window:allow-hide",
    "core:tray:default",
    "core:menu:default"
  ]
}
```

## 技术细节与问题修复

### 点击事件处理的关键点

#### 问题 1：右键菜单闪烁消失

**原因**：使用 `matches!(event, TrayIconEvent::Click { .. })` 会匹配所有点击事件（左键、右键、中键），导致右键点击时同时触发窗口切换逻辑，菜单显示后立即消失。

**解决方案**：使用精确的模式匹配，只响应左键的按键抬起事件。

```rust
// ❌ 错误写法 - 匹配所有点击
if matches!(event, TrayIconEvent::Click { .. }) {
    // 处理点击
}

// ✅ 正确写法 - 只匹配左键抬起
if let TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Up,
    ..
} = event
{
    // 处理左键点击
}
```

#### 问题 2：左键点击显示菜单

**原因**：未禁用左键显示菜单的默认行为。

**解决方案**：在 `TrayIconBuilder` 中添加 `.show_menu_on_left_click(false)`。

```rust
let _tray = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .show_menu_on_left_click(false)  // 关键：禁用左键显示菜单
    .on_menu_event(/* ... */)
    .on_tray_icon_event(/* ... */)
    .build(app)?;
```

### 事件类型说明

#### TrayIconEvent 枚举

```rust
pub enum TrayIconEvent {
    Click {
        button: MouseButton,        // 鼠标按钮类型
        button_state: MouseButtonState, // 按钮状态
        // ... 其他字段
    },
    DoubleClick { /* ... */ },
    Enter { /* ... */ },
    Leave { /* ... */ },
    Move { /* ... */ },
}
```

#### MouseButton 枚举

```rust
pub enum MouseButton {
    Left,    // 左键
    Right,   // 右键
    Middle,  // 中键
}
```

#### MouseButtonState 枚举

```rust
pub enum MouseButtonState {
    Up,   // 按键抬起
    Down, // 按键按下
}
```

### 完整行为矩阵

| 操作 | show_menu_on_left_click | 事件匹配 | 结果 |
|------|------------------------|---------|------|
| 左键单击 | false | Left + Up | ✅ 切换窗口显示/隐藏 |
| 左键单击 | true | Left + Up | ❌ 显示菜单（不符合预期） |
| 右键单击 | false | Right + Up | ✅ 显示菜单（系统行为） |
| 右键单击 | false | 不匹配 | ✅ 不执行窗口切换 |

## 常见问题

### Q: 为什么点击关闭按钮后窗口还在？
**A**：这是默认行为。点击关闭按钮会将窗口最小化到托盘，避免误关闭。如需完全退出：
- 右键托盘图标 → 退出应用
- 或在设置中将"点击关闭按钮时的行为"改为"退出应用"

### Q: 如何快速显示/隐藏窗口？
**A**：左键单击托盘图标即可。

### Q: 托盘图标在哪里？
**A**：在系统任务栏的通知区域（Windows 右下角）。可能需要点击向上箭头展开隐藏的图标。

### Q: 托盘菜单和全局快捷键有什么区别？
**A**：
- **托盘菜单**：需要用鼠标操作，可以看到所有功能选项
- **全局快捷键**：可以在任何应用中使用键盘快速打开，更高效

两者配合使用，提供灵活的访问方式。

## 测试检查清单

### 基础功能
- [ ] 托盘图标正确显示
- [ ] 左键单击：显示→隐藏→显示（循环测试）
- [ ] 右键单击：菜单完整显示，不闪烁
- [ ] 菜单分隔符正确显示

### 菜单导航
- [ ] 环境变量管理 - 正确跳转
- [ ] 文件搜索 - 正确跳转
- [ ] 项目管理 - 正确跳转
- [ ] 设置 - 正确跳转
- [ ] 退出应用 - 应用关闭

### 窗口状态
- [ ] 窗口隐藏时，菜单项可以显示窗口
- [ ] 窗口显示时，菜单项可以切换界面
- [ ] 窗口聚焦正常

### 关闭行为
- [ ] 默认（最小化到托盘）：关闭按钮 → 隐藏到托盘
- [ ] 退出应用模式：关闭按钮 → 退出应用
- [ ] 无动作模式：关闭按钮 → 无响应
- [ ] 托盘菜单"退出"始终退出应用（不受设置影响）
- [ ] 设置持久化（重启后保持）

### 边界情况
- [ ] 快速连续点击托盘图标
- [ ] 窗口最小化时的行为
- [ ] 窗口最大化时的行为

## 平台兼容性

- **Windows**：✅ 完全支持，行为符合预期
- **macOS**：⚠️ 托盘在顶部菜单栏，左键行为可能不同
- **Linux**：⚠️ 取决于桌面环境，部分功能可能受限
