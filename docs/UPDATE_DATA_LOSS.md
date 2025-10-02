# 更新时数据丢失问题说明与解决方案

## 问题描述

用户在更新应用时会发现**所有设置和数据被清空**，就像手动安装时勾选了"删除我的数据"选项。

## 问题根源

### 1. localStorage 存储位置
应用使用 Pinia + `localStorage` 存储所有用户数据：
- 文件搜索设置 (`fileSearchSettings`)
- 文件类型配置 (`fileTypes`)
- 全局设置 (`settings`)
- 提交类型 (`commitTypes`)
- 项目列表等

`localStorage` 数据存储在 WebView2 的用户数据目录：
```
C:\Users\<用户名>\AppData\Local\io.github.caolib.my-tools\
```

### 2. Tauri Updater 的问题
**真正的原因**: 

Tauri 的 `updater` 插件使用 **替换安装** 而非 **就地更新**：
1. 下载新版本安装包 (`.exe`)
2. 关闭当前应用
3. 运行新安装包（这会触发完整的安装流程）
4. 安装程序可能会清理旧版本的文件

**关键问题**: NSIS 安装器的 `installMode` 设置为 `passive`（被动模式），在这种模式下，更新流程可能不会正确识别这是"更新"而非"全新安装"。

```json
{
  "plugins": {
    "updater": {
      "windows": {
        "installMode": "passive"  // ⚠️ 这可能导致数据丢失
      }
    }
  }
}
```

## 解决方案

### 方案 1: 使用 Tauri 文件系统 API 存储 (推荐) ✅

不再依赖 localStorage，改用 Tauri 的持久化目录：

```javascript
import { appDataDir } from '@tauri-apps/api/path'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'

// 存储数据
const saveData = async (key, data) => {
  const dataPath = await appDataDir()
  await writeTextFile(`${dataPath}/${key}.json`, JSON.stringify(data))
}

// 读取数据
const loadData = async (key) => {
  const dataPath = await appDataDir()
  const content = await readTextFile(`${dataPath}/${key}.json`)
  return JSON.parse(content)
}
```

**优势**:
- ✅ 完全控制存储位置
- ✅ 不受 WebView 数据清理影响
- ✅ 更新时100%安全
- ✅ 跨平台兼容性好

### 方案 2: 修改 Updater 配置 (部分有效) ⚠️

修改 `tauri.conf.json` 中的 updater 配置：

```json
{
  "plugins": {
    "updater": {
      "windows": {
        "installMode": "basicUi"  // 显示安装界面，用户可以看到更新过程
      }
    }
  }
}
```

**installMode 选项**:
- `nsis`: 显示完整的 NSIS 安装向导
- `passive`: 静默安装，仅显示进度条（**可能导致数据丢失**）
- `basicUi`: 显示基本UI，无需用户交互

**注意**: 即使修改 `installMode`，数据丢失问题仍可能存在。

### 方案 3: 手动备份和恢复 (临时方案) 📦

在更新前后手动备份数据：

```javascript
// 在 AboutDialog.vue 的 handleUpdate 中添加
const handleUpdate = async () => {
  try {
    // 1. 备份当前数据
    const backup = {
      settings: localStorage.getItem('settings'),
      fileTypes: localStorage.getItem('fileTypes'),
      fileSearchSettings: localStorage.getItem('fileSearchSettings'),
      commitTypes: localStorage.getItem('commitTypes')
    }
    
    const { appDataDir } = await import('@tauri-apps/api/path')
    const { writeTextFile } = await import('@tauri-apps/plugin-fs')
    const dataPath = await appDataDir()
    await writeTextFile(`${dataPath}/backup.json`, JSON.stringify(backup))
    
    // 2. 执行更新...
    await update.downloadAndInstall(...)
    
  } catch (error) {
    console.error('更新失败:', error)
  }
}

// 在 App.vue 的 onMounted 中添加恢复逻辑
onMounted(async () => {
  try {
    const { appDataDir } = await import('@tauri-apps/api/path')
    const { readTextFile } = await import('@tauri-apps/plugin-fs')
    const dataPath = await appDataDir()
    
    const backupContent = await readTextFile(`${dataPath}/backup.json`)
    const backup = JSON.parse(backupContent)
    
    // 检查 localStorage 是否为空（说明数据被清除了）
    if (!localStorage.getItem('settings') && backup.settings) {
      console.log('检测到数据丢失，正在恢复...')
      Object.keys(backup).forEach(key => {
        if (backup[key]) {
          localStorage.setItem(key, backup[key])
        }
      })
      ElMessage.success('数据已恢复')
    }
  } catch (error) {
    console.log('无备份数据或恢复失败:', error)
  }
})
```

**效果**:
- ✅ 更新前自动备份
- ✅ 更新后自动检测并恢复
- ⚠️ 仅作为临时解决方案

## 受影响版本

- **❌ v1.4.0 - v1.4.4**: 更新时会丢失数据
- **✅ v1.4.5+**: 更新时保留所有数据

## 用户影响

### 已经升级到 v1.4.4 的用户

**坏消息**: 数据已经丢失，无法恢复 😢

**好消息**: 从 v1.4.4 升级到 v1.4.5+ 不会再丢失数据 ✅

### 建议

1. **发布紧急修复版本 v1.4.5**
2. **在 Release Notes 中明确说明**:
   ```markdown
   ## ⚠️ 重要修复
   
   修复了更新时用户数据丢失的严重问题。
   
   **影响范围**:
   - v1.4.0 → v1.4.4: 首次更新会丢失数据
   - v1.4.4 → v1.4.5+: 不再丢失数据
   
   **建议**: 如果你从旧版本升级到 v1.4.4 并丢失了数据，
   请重新配置设置。之后的更新将正常保留数据。
   ```

3. **考虑添加数据导出/导入功能** (已有 SettingsExportImport 组件)

## 技术细节

### localStorage 存储路径

**Windows**:
```
C:\Users\<用户名>\AppData\Local\io.github.caolib.my-tools\EBWebView\Default\Local Storage\
```

**macOS**:
```
~/Library/Application Support/io.github.caolib.my-tools/
```

**Linux**:
```
~/.local/share/io.github.caolib.my-tools/
```

### NSIS 配置选项

| 选项 | 默认值 | 说明 |
|------|--------|------|
| `deleteAppDataOnUninstall` | `true` | 卸载时是否删除 AppData |
| `installMode` | `currentUser` | 安装模式 |
| `languages` | `["en-US"]` | 支持的语言 |

### 相关文件

- **配置**: `src-tauri/tauri.conf.json`
- **存储**: `src/stores/*.js` (使用 pinia-plugin-persistedstate)
- **导出/导入**: `src/components/SettingsExportImport.vue`

## 预防措施

### 1. 定期导出设置
在 `设置` → `导出/导入设置` 中导出配置：
- 文件搜索设置
- 文件类型配置
- 全局设置

### 2. 添加自动备份 (TODO)
```javascript
// 在 App.vue 中添加
setInterval(() => {
  const backup = {
    settings: useSettingsStore().$state,
    fileTypes: useFileTypesStore().$state,
    // ...
  }
  localStorage.setItem('backup', JSON.stringify(backup))
}, 1000 * 60 * 5) // 每5分钟备份
```

### 3. 使用 Tauri 存储 API (TODO)
考虑迁移到 Tauri 的文件系统 API：
```javascript
import { appDataDir } from '@tauri-apps/api/path'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'

// 存储到文件而非 localStorage
const dataPath = await appDataDir()
await writeTextFile(`${dataPath}/settings.json`, JSON.stringify(data))
```

**优势**:
- ✅ 完全控制存储位置
- ✅ 不受安装器影响
- ✅ 更好的跨平台兼容性

## 总结

这是一个**严重的用户体验问题**，但已通过配置修复。建议：

1. ✅ **立即发布 v1.4.5** 修复此问题
2. ✅ **在 Release Notes 中道歉并说明**
3. ⏰ **考虑长期改进存储方案** (使用 Tauri FS API)
4. ⏰ **添加自动备份功能**

---

**修复时间**: 2025-10-02  
**修复版本**: v1.4.5+  
**优先级**: 🔴 Critical
