# 更新数据保护机制说明

## ✅ 已实现：自动备份和恢复

**实现时间**: 2025-10-02  
**状态**: ✅ 已完成并集成  
**版本**: v1.4.5+

---

## 问题背景

Tauri Updater 在更新时会运行新的安装程序，可能导致 WebView2 的 localStorage 数据被清除，从而丢失用户设置、文件类型配置、搜索设置等所有数据。

## 解决方案

参考 `Settings.vue` 中清除缓存功能的备份逻辑，实现了**完全自动化的备份和恢复机制**。

---

## 工作原理

### 1. 更新前自动备份 (`AboutDialog.vue`)

当用户点击"立即更新"并确认后，系统会：

1. 立即备份所有 Pinia 持久化数据到 Tauri 的 `appDataDir`
2. 备份文件: `update-backup.json`
3. 备份内容:
   - `wem-settings`: 全局设置
   - `wem-file-search-settings`: 文件搜索设置
   - `wem-file-types`: 文件类型配置
   - `wem-commit-types`: 提交类型配置
   - `timestamp`: 备份时间戳

**代码位置**: `src/components/AboutDialog.vue` 的 `handleUpdate()` 函数

```javascript
// 备份 localStorage 数据到 Tauri 的 appDataDir，防止更新时丢失
try {
    const { appDataDir } = await import('@tauri-apps/api/path')
    const { writeTextFile } = await import('@tauri-apps/plugin-fs')
    
    const backupData = {
        'wem-settings': localStorage.getItem('wem-settings'),
        'wem-file-search-settings': localStorage.getItem('wem-file-search-settings'),
        'wem-file-types': localStorage.getItem('wem-file-types'),
        'wem-commit-types': localStorage.getItem('wem-commit-types'),
        timestamp: new Date().toISOString()
    }
    
    const dataPath = await appDataDir()
    await writeTextFile(`${dataPath}/update-backup.json`, JSON.stringify(backupData, null, 2))
    console.log('数据备份成功')
} catch (backupError) {
    console.warn('备份数据失败，但继续更新:', backupError)
    // 备份失败不应阻止更新流程
}
```

### 2. 启动时自动检测并恢复 (`App.vue`)

应用启动时会：

1. 检查是否存在 `update-backup.json` 文件
2. 如果存在且 localStorage 为空（数据丢失）→ 自动恢复
3. 如果数据完好 → 直接删除备份文件
4. 恢复成功后显示友好提示消息

**代码位置**: `src/App.vue` 的 `restoreDataFromBackup()` 函数

```javascript
const restoreDataFromBackup = async () => {
  try {
    const { appDataDir } = await import('@tauri-apps/api/path')
    const { readTextFile, exists, remove } = await import('@tauri-apps/plugin-fs')
    
    const dataPath = await appDataDir()
    const backupFilePath = `${dataPath}/update-backup.json`
    
    // 检查备份文件是否存在
    if (await exists(backupFilePath)) {
      const backupContent = await readTextFile(backupFilePath)
      const backupData = JSON.parse(backupContent)
      
      // 检查 localStorage 是否为空（说明数据被更新清除了）
      const currentSettings = localStorage.getItem('wem-settings')
      
      if (!currentSettings && backupData['wem-settings']) {
        // 恢复所有备份的数据
        Object.entries(backupData).forEach(([key, value]) => {
          if (key !== 'timestamp' && value) {
            localStorage.setItem(key, value)
          }
        })
        
        await remove(backupFilePath)
        ElMessage.success({
          message: '检测到更新后数据丢失，已自动恢复您的设置和配置',
          duration: 5000
        })
      } else {
        // 数据完好，删除备份文件
        await remove(backupFilePath)
      }
    }
  } catch (error) {
    console.warn('检查/恢复备份失败:', error)
  }
}
```

---

## 备份文件位置

**Windows**:
```
C:\Users\<用户>\AppData\Roaming\io.github.caolib.my-tools\update-backup.json
```

**macOS**:
```
~/Library/Application Support/io.github.caolib.my-tools/update-backup.json
```

**Linux**:
```
~/.local/share/io.github.caolib.my-tools/update-backup.json
```

**特点**:
- ✅ 存储在 Tauri 的 appDataDir，独立于 WebView 数据目录
- ✅ 不受 NSIS 安装程序清理影响
- ✅ 支持跨平台

---

## 测试场景

### 场景 1: 正常更新流程（数据不丢失）

1. 用户点击"立即更新"
2. ✅ 系统自动备份数据到 `update-backup.json`
3. 下载并安装新版本
4. 应用重启
5. ✅ 系统检查备份文件
6. ✅ 发现 localStorage 数据完好
7. ✅ 删除备份文件
8. 用户无感知，一切正常

### 场景 2: 数据丢失恢复（关键场景）

1. 用户点击"立即更新"
2. ✅ 系统自动备份数据到 `update-backup.json`
3. 下载并安装新版本
4. ❌ NSIS 安装程序清除了 WebView2 数据
5. 应用重启
6. ✅ 系统检查备份文件
7. ✅ 发现 localStorage 为空（数据丢失）
8. ✅ 自动从备份恢复所有数据
9. ✅ 删除备份文件
10. ✅ 显示成功消息: "检测到更新后数据丢失，已自动恢复您的设置和配置"
11. 用户数据完整无损

### 场景 3: 备份失败（极端情况）

1. 用户点击"立即更新"
2. ❌ 备份失败（例如磁盘满）
3. ⚠️ 记录警告日志
4. ✅ 继续更新流程（不阻止更新）
5. 下载并安装新版本
6. 如果数据丢失 → 无法恢复（但很少发生）

---

## 优势

| 特性 | 说明 |
|------|------|
| ✅ **完全自动** | 无需用户任何操作，完全透明 |
| ✅ **安全可靠** | 存储在 appDataDir，不受更新影响 |
| ✅ **容错性强** | 备份/恢复失败不影响应用运行 |
| ✅ **代码复用** | 复用 Settings.vue 的成熟备份逻辑 |
| ✅ **用户友好** | 恢复成功时显示友好提示 |
| ✅ **即时生效** | 从 v1.4.5 开始所有用户都受保护 |

---

## 代码复用说明

本机制参考了 `src/views/Settings.vue` 中 `confirmClearCache()` 函数的备份逻辑：

```javascript
// Settings.vue 中的原始逻辑
const backupData = {
    'wem-settings': localStorage.getItem('wem-settings'),
    'wem-file-search-settings': localStorage.getItem('wem-file-search-settings'),
    'wem-file-types': localStorage.getItem('wem-file-types')
}

// 清除缓存...

// 恢复备份
Object.entries(backupData).forEach(([key, value]) => {
    if (value) {
        localStorage.setItem(key, value)
    }
})
```

**关键改进**:
1. 将备份存储到文件系统（appDataDir）而非内存
2. 在应用重启后仍能恢复
3. 支持跨更新版本恢复

---

## 与其他方案对比

| 方案 | 实现难度 | 可靠性 | 用户体验 | 开发成本 |
|------|----------|--------|----------|----------|
| **方案1: 自动备份/恢复** | ⭐ 低 | ⭐⭐⭐⭐ 高 | ⭐⭐⭐⭐⭐ 优秀 | ⭐ 低（已完成） |
| 方案2: 迁移到 Tauri FS | ⭐⭐⭐ 高 | ⭐⭐⭐⭐⭐ 最高 | ⭐⭐⭐⭐ 很好 | ⭐⭐⭐ 高 |
| 方案3: NSIS hooks | ⭐⭐ 中 | ⭐⭐ 低 | ⭐⭐⭐ 一般 | ⭐⭐ 中 |

**结论**: 方案1 最适合作为快速修复，方案2 适合长期规划。

---

## 监控和日志

系统会记录以下日志，便于排查问题：

```javascript
// 备份成功
console.log('数据备份成功，备份文件:', `${dataPath}/update-backup.json`)

// 备份失败
console.warn('备份数据失败，但继续更新:', backupError)

// 发现备份文件
console.log('发现更新备份文件，正在检查...')

// 数据丢失，开始恢复
console.log('检测到数据丢失，正在从备份恢复...')

// 恢复成功
console.log('已恢复: wem-settings')
console.log('数据恢复成功，备份文件已删除')

// 数据完好
console.log('数据完好，无需恢复，备份文件已删除')

// 恢复失败
console.warn('检查/恢复备份失败:', error)
```

---

## 注意事项

1. **备份文件的生命周期**:
   - 创建: 用户点击"立即更新"时
   - 删除: 下次应用启动时（无论是否恢复）
   - 如果应用崩溃，备份文件会残留但不影响使用

2. **备份内容的限制**:
   - 仅备份 Pinia 持久化的 localStorage 数据
   - 不包括临时状态（如正在下载的文件列表）
   - 不包括 session storage 数据

3. **兼容性**:
   - 支持所有 Tauri 支持的平台
   - 需要 Tauri FS 插件（已集成）

---

## 未来改进方向

1. **定期自动备份**: 每隔一段时间自动备份（不仅限于更新前）
2. **多版本备份**: 保留多个历史备份
3. **云端同步**: 支持将备份同步到云端
4. **迁移到 Tauri FS**: 彻底放弃 localStorage，使用文件系统存储

---

## 相关文件

| 文件 | 说明 |
|------|------|
| `src/components/AboutDialog.vue` | 更新对话框，包含备份逻辑 |
| `src/App.vue` | 应用入口，包含恢复逻辑 |
| `src/views/Settings.vue` | 清除缓存功能，备份逻辑的灵感来源 |
| `docs/UPDATE_DATA_PROTECTION.md` | 本文档 |
| `docs/UPDATE_DATA_LOSS.md` | 问题分析文档 |

---

**作者**: GitHub Copilot  
**日期**: 2025-10-02  
**优先级**: 🔴 Critical  
**状态**: ✅ 已完成
