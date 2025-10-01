# 全局快捷键功能

## 功能概述

应用支持设置全局快捷键，可以在任何应用中快速打开本应用的特定界面。

## 支持的快捷键

- **环境变量管理**：快速打开环境变量管理界面
- **文件搜索**：快速打开文件搜索界面
- **项目管理**：快速打开项目管理界面

## 使用说明

### 设置快捷键

1. 打开应用设置界面
2. 找到"全局快捷键"设置卡片
3. 在对应功能的输入框中，直接按下你想设置的快捷键组合
   - 例如：按下 `Ctrl+Shift+E` 会自动填入该快捷键
4. 点击"设置"按钮保存

### 清除快捷键

- 点击"清除"按钮即可移除已设置的快捷键

### 快捷键冲突

- 如果快捷键已被其他功能使用，系统会提示"该快捷键已被其他功能使用"
- 如果快捷键被系统占用，系统会提示"该快捷键已被系统占用"
- 建议使用 `Ctrl+Shift+[字母]` 的组合，避免冲突

## 技术实现

### 前端实现

#### 1. 状态管理 (`src/stores/settings.js`)

```javascript
// 全局快捷键设置
globalShortcuts: {
  envVarManager: '', // 打开环境变量管理界面的快捷键
  fileSearch: '', // 打开文件搜索界面的快捷键
  projects: '', // 打开项目管理界面的快捷键
}
```

#### 2. 设置界面 (`src/views/Settings.vue`)

**快捷键输入处理**：
```javascript
const handleShortcutInput = (event, key) => {
    event.preventDefault()

    // 构建快捷键字符串
    const parts = []
    if (event.ctrlKey) parts.push('Ctrl')
    if (event.shiftKey) parts.push('Shift')
    if (event.altKey) parts.push('Alt')
    if (event.metaKey) parts.push('Super')

    // 获取按键
    const keyCode = event.key
    if (!['Control', 'Shift', 'Alt', 'Meta'].includes(keyCode)) {
        parts.push(keyCode.toUpperCase())
    }

    // 至少需要一个修饰键
    if (parts.length >= 2) {
        settingsStore.globalShortcuts[key] = parts.join('+')
    }
}
```

**快捷键注册**：
```javascript
const handleSetShortcut = async (key) => {
    const shortcut = settingsStore.globalShortcuts[key]

    if (!shortcut) {
        ElMessage.warning('请先输入快捷键')
        return
    }

    try {
        // 检查是否与其他快捷键冲突
        for (const [k, v] of Object.entries(settingsStore.globalShortcuts)) {
            if (k !== key && v === shortcut) {
                ElMessage.warning('该快捷键已被其他功能使用')
                return
            }
        }

        // 取消旧快捷键
        const oldShortcut = registeredShortcuts[key]
        if (oldShortcut) {
            try {
                await unregister(oldShortcut)
            } catch (error) {
                console.warn('取消旧快捷键失败:', error)
            }
        }

        // 注册新快捷键
        await register(shortcut, async () => {
            const window = getCurrentWindow()
            await window.show()
            await window.setFocus()

            // 导航到对应界面
            router.push(getRouteForKey(key))
        })

        registeredShortcuts[key] = shortcut
        ElMessage.success(`快捷键 ${shortcut} 设置成功`)
    } catch (error) {
        console.error('设置快捷键失败:', error)
        ElMessage.error('该快捷键已被系统占用')
    }
}
```

**应用启动时自动注册**：
```javascript
onMounted(async () => {
    // 从 store 加载快捷键
    const shortcuts = settingsStore.globalShortcuts

    // 注册所有已保存的快捷键
    for (const [key, shortcut] of Object.entries(shortcuts)) {
        if (shortcut) {
            try {
                await register(shortcut, async () => {
                    const window = getCurrentWindow()
                    await window.show()
                    await window.setFocus()
                    router.push(getRouteForKey(key))
                })

                registeredShortcuts[key] = shortcut
                console.log(`✅ 已注册全局快捷键: ${shortcut} -> ${getKeyLabel(key)}`)
            } catch (error) {
                console.warn(`❌ 注册全局快捷键失败: ${shortcut}`, error)
            }
        }
    }
})
```

### 后端实现

全局快捷键功能主要在前端实现，使用 Tauri 的 `@tauri-apps/plugin-global-shortcut` 插件。

### 权限配置 (`src-tauri/capabilities/default.json`)

```json
{
  "permissions": [
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-is-registered"
  ]
}
```

## 使用示例

### 场景 1：设置环境变量管理快捷键

1. 打开设置 → 全局快捷键
2. 在"环境变量管理"输入框中按下 `Ctrl+Shift+E`
3. 点击"设置"按钮
4. 现在可以在任何应用中按 `Ctrl+Shift+E` 打开环境变量管理界面

### 场景 2：修改已设置的快捷键

1. 在输入框中按下新的快捷键组合
2. 点击"设置"按钮
3. 旧快捷键自动失效，新快捷键生效

### 场景 3：清除快捷键

1. 点击对应功能的"清除"按钮
2. 快捷键被移除，按键不再有效

## 推荐快捷键组合

- **环境变量管理**：`Ctrl+Shift+E`
- **文件搜索**：`Ctrl+Shift+F`
- **项目管理**：`Ctrl+Shift+P`

## 注意事项

1. **修饰键要求**：快捷键必须包含至少一个修饰键（Ctrl、Shift、Alt、Super）
2. **避免冲突**：不要使用系统已占用的快捷键（如 `Ctrl+C`、`Ctrl+V` 等）
3. **持久化**：快捷键设置会自动保存，应用重启后自动加载
4. **跨应用**：全局快捷键在任何应用中都有效，即使本应用在后台

## 故障排除

### 快捷键不生效

1. 检查快捷键是否已成功设置（输入框中显示快捷键）
2. 确认快捷键未被系统或其他应用占用
3. 尝试使用不同的快捷键组合

### 提示"快捷键已被系统占用"

- 更换其他快捷键组合
- 优先使用 `Ctrl+Shift+[字母]` 的组合

### 快捷键冲突

- 系统会自动检测与其他功能的冲突
- 每个功能只能设置一个快捷键
- 一个快捷键不能同时分配给多个功能
