<template>
    <div class="settings-page">
        <div class="settings-content">
            <!-- 全局设置卡片 -->
            <el-card class="settings-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Setting />
                        </el-icon>
                        <span>全局设置</span>
                    </div>
                </template>
                <div class="card-content">
                    <el-form label-position="left">
                        <el-form-item>
                            <template #label>
                                <span>打开项目后的行为</span>
                                <el-tooltip placement="top">
                                    <template #content>
                                        <div>最小化到托盘：打开项目后隐藏到系统托盘</div>
                                        <div>退出应用：打开项目后直接关闭应用</div>
                                        <div>无动作：打开项目后保持应用打开</div>
                                    </template>
                                    <el-icon style="margin-left: 4px; cursor: help;">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </template>
                            <el-radio-group v-model="settingsStore.afterOpenProjectBehavior">
                                <el-radio value="none">无动作</el-radio>
                                <el-radio value="minimize">最小化到托盘</el-radio>
                                <el-radio value="quit">退出应用</el-radio>
                            </el-radio-group>
                        </el-form-item>
                        <el-form-item>
                            <template #label>
                                <span>复制信息后的行为</span>
                                <el-tooltip placement="top">
                                    <template #content>
                                        <div>最小化到托盘：复制后隐藏到系统托盘</div>
                                        <div>退出应用：复制后直接关闭应用</div>
                                        <div>无动作：复制后保持应用打开</div>
                                    </template>
                                    <el-icon style="margin-left: 4px; cursor: help;">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </template>
                            <el-radio-group v-model="settingsStore.afterCopyCommitBehavior">
                                <el-radio value="none">无动作</el-radio>
                                <el-radio value="minimize">最小化到托盘</el-radio>
                                <el-radio value="quit">退出应用</el-radio>
                            </el-radio-group>
                        </el-form-item>
                        <el-form-item>
                            <template #label>
                                <span>点击关闭按钮时最小化到托盘</span>
                                <el-tooltip content="关闭后将最小化到系统托盘，而不是退出应用" placement="top">
                                    <el-icon style="margin-left: 4px; cursor: help;">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </template>
                            <el-switch v-model="settingsStore.closeToTray" />
                        </el-form-item>
                    </el-form>
                </div>
            </el-card>

            <!-- 全局快捷键设置卡片 -->
            <el-card class="settings-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Calendar />
                        </el-icon>
                        <span>全局快捷键</span>
                    </div>
                </template>
                <div class="card-content">
                    <div class="shortcuts-settings">
                        <div class="shortcut-item">
                            <div class="shortcut-label">
                                <el-icon>
                                    <Document />
                                </el-icon>
                                <span>环境变量管理</span>
                            </div>
                            <div class="shortcut-input">
                                <el-input v-model="shortcuts.envVarManager" placeholder="未设置"
                                    @keydown="handleShortcutCapture($event, 'envVarManager')" clearable
                                    @clear="clearShortcut('envVarManager')">
                                    <template #append>
                                        <el-button :icon="shortcuts.envVarManager ? Check : Plus"
                                            @click="saveShortcut('envVarManager')"
                                            :type="shortcuts.envVarManager ? 'success' : 'primary'" />
                                    </template>
                                </el-input>
                            </div>
                        </div>

                        <div class="shortcut-item">
                            <div class="shortcut-label">
                                <el-icon>
                                    <Search />
                                </el-icon>
                                <span>文件搜索</span>
                            </div>
                            <div class="shortcut-input">
                                <el-input v-model="shortcuts.fileSearch" placeholder="未设置"
                                    @keydown="handleShortcutCapture($event, 'fileSearch')" clearable
                                    @clear="clearShortcut('fileSearch')">
                                    <template #append>
                                        <el-button :icon="shortcuts.fileSearch ? Check : Plus"
                                            @click="saveShortcut('fileSearch')"
                                            :type="shortcuts.fileSearch ? 'success' : 'primary'" />
                                    </template>
                                </el-input>
                            </div>
                        </div>

                        <div class="shortcut-item">
                            <div class="shortcut-label">
                                <el-icon>
                                    <Folder />
                                </el-icon>
                                <span>项目管理</span>
                            </div>
                            <div class="shortcut-input">
                                <el-input v-model="shortcuts.projects" placeholder="未设置"
                                    @keydown="handleShortcutCapture($event, 'projects')" clearable
                                    @clear="clearShortcut('projects')">
                                    <template #append>
                                        <el-button :icon="shortcuts.projects ? Check : Plus"
                                            @click="saveShortcut('projects')"
                                            :type="shortcuts.projects ? 'success' : 'primary'" />
                                    </template>
                                </el-input>
                            </div>
                        </div>

                        <div class="shortcut-item">
                            <div class="shortcut-label">
                                <el-icon>
                                    <Edit />
                                </el-icon>
                                <span>提交生成器</span>
                            </div>
                            <div class="shortcut-input">
                                <el-input v-model="shortcuts.commitGenerator" placeholder="未设置"
                                    @keydown="handleShortcutCapture($event, 'commitGenerator')" clearable
                                    @clear="clearShortcut('commitGenerator')">
                                    <template #append>
                                        <el-button :icon="shortcuts.commitGenerator ? Check : Plus"
                                            @click="saveShortcut('commitGenerator')"
                                            :type="shortcuts.commitGenerator ? 'success' : 'primary'" />
                                    </template>
                                </el-input>
                            </div>
                        </div>
                    </div>

                    <el-alert type="info" :closable="false" class="shortcut-tips">
                        <template #title>
                            <div>💡 提示：在输入框中直接按键会自动识别快捷键组合</div>
                        </template>
                    </el-alert>
                </div>
            </el-card>

            <!-- 设置管理卡片 -->
            <el-card class="settings-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Setting />
                        </el-icon>
                        <span>设置管理</span>
                    </div>
                </template>
                <div class="card-content">
                    <div class="action-buttons">
                        <el-button type="primary" @click="exportSettings" :loading="exporting">
                            <el-icon>
                                <Download />
                            </el-icon>
                            导出设置
                        </el-button>
                        <el-button @click="triggerFileInput">
                            <el-icon>
                                <Upload />
                            </el-icon>
                            导入设置
                        </el-button>
                        <el-button @click="showCommitTypesManager">
                            <el-icon>
                                <Edit />
                            </el-icon>
                            管理提交类型
                        </el-button>
                        <el-button type="danger" @click="confirmResetSettings" :loading="resetting">
                            <el-icon>
                                <Delete />
                            </el-icon>
                            重置设置
                        </el-button>
                    </div>
                    <input ref="fileInputRef" type="file" accept=".json" @change="handleFileSelect"
                        class="file-input" />
                </div>
            </el-card>

            <!-- 缓存管理卡片 -->
            <el-card class="settings-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Delete />
                        </el-icon>
                        <span>缓存管理</span>
                    </div>
                </template>
                <div class="card-content">
                    <p class="card-description">
                        清除应用缓存可以释放磁盘空间，您的设置和数据不会受到影响。
                    </p>

                    <!-- 缓存信息显示 -->
                    <div class="cache-info">
                        <div class="info-item">
                            <span class="info-label">缓存大小：</span>
                            <span class="info-value">{{ cacheInfo.cacheSizeFormatted }}</span>
                        </div>
                        <div class="info-item">
                            <span class="info-label">上次清理还是在：</span>
                            <span class="info-value">{{ lastClearTimeFormatted }}</span>
                        </div>
                        <div class="info-item">
                            <span class="info-label">缓存路径：</span>
                            <span class="info-value path-value">{{ cacheInfo.cachePath }}</span>
                            <el-button link type="primary" :icon="FolderOpened" @click="openCacheFolder" size="small"
                                v-if="cacheInfo.cachePath">
                                打开文件夹
                            </el-button>
                        </div>
                    </div>

                    <div class="action-buttons">
                        <el-button type="warning" @click="confirmClearCache" :loading="clearingCache">
                            <el-icon>
                                <Delete />
                            </el-icon>
                            清除缓存
                        </el-button>
                    </div>
                </div>
            </el-card>
        </div>
    </div>

    <!-- 提交类型管理器对话框 -->
    <CommitTypesManager ref="commitTypesManagerRef" />
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
    Setting,
    Download,
    Upload,
    Delete,
    FolderOpened,
    Calendar,
    Document,
    Search,
    Folder,
    Edit,
    Check,
    Plus,
    QuestionFilled
} from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useSettingsStore } from '@/stores/settings'
import { useFileSearchSettingsStore } from '@/stores/fileSearchSettings'
import { useFileTypesStore } from '@/stores/fileTypes'
import { useCommitTypesStore } from '@/stores/commitTypes'
import { registerShortcut, unregisterShortcut, checkShortcutAvailable } from '@/utils/shortcutManager'
import { useRouter } from 'vue-router'
import CommitTypesManager from '@/components/CommitTypesManager.vue'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { join } from '@tauri-apps/api/path'

// Stores
const settingsStore = useSettingsStore()
const fileSearchStore = useFileSearchSettingsStore()
const fileTypesStore = useFileTypesStore()
const commitTypesStore = useCommitTypesStore()

// Router
const router = useRouter()

// Reactive data
const exporting = ref(false)
const resetting = ref(false)
const clearingCache = ref(false)
const fileInputRef = ref(null)
const commitTypesManagerRef = ref(null)
const shortcuts = ref({
    envVarManager: '',
    fileSearch: '',
    projects: '',
    commitGenerator: ''
})
const cacheInfo = ref({
    cachePath: '',
    cacheSize: 0,
    cacheSizeFormatted: '计算中...'
})

// Computed
const lastClearTimeFormatted = computed(() => {
    if (!settingsStore.lastCacheClearTime) {
        return '上次'
    }
    const date = new Date(settingsStore.lastCacheClearTime)
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
    })
})

// Methods
const loadCacheInfo = async () => {
    try {
        const info = await invoke('get_cache_info')
        cacheInfo.value = {
            cachePath: info.cache_path,
            cacheSize: info.cache_size,
            cacheSizeFormatted: info.cache_size_formatted
        }
    } catch (error) {
        console.error('获取缓存信息失败:', error)
        cacheInfo.value.cacheSizeFormatted = '获取失败'
    }
}

const openCacheFolder = async () => {
    try {
        await invoke('shell_open', { path: cacheInfo.value.cachePath })
    } catch (error) {
        ElMessage.error('打开缓存文件夹失败：' + error)
    }
}

const exportSettings = async () => {
    try {
        exporting.value = true

        // 直接使用settingsStore中的设置
        const exportPath = settingsStore.exportPath
        const autoOpenFolder = settingsStore.autoOpenFolder

        // 使用默认文件名格式：全局设置备份_日期.json
        const timestamp = new Date().toISOString().split('T')[0]
        const defaultFileName = `my-tools_${timestamp}.json`

        let finalExportPath = ''

        // 如果有设置默认路径，直接使用
        if (exportPath) {
            finalExportPath = await join(exportPath, defaultFileName)
        } else {
            // 否则让用户选择路径
            const documentsDir = await invoke('get_documents_dir')
            const selected = await save({
                title: '导出全局设置',
                defaultPath: await join(documentsDir, defaultFileName),
                filters: [
                    {
                        name: 'JSON文件',
                        extensions: ['json']
                    }
                ]
            })

            if (!selected) {
                exporting.value = false
                return
            }
            finalExportPath = selected
        }

        // 准备导出数据
        const allSettings = {
            settings: settingsStore.$state,
            fileSearchSettings: fileSearchStore.$state,
            fileTypes: fileTypesStore.$state,
            commitTypes: commitTypesStore.$state
        }

        const settings = {
            exportDate: new Date().toISOString(),
            version: '1.0.0',
            data: allSettings
        }

        // 写入文件
        await writeTextFile(finalExportPath, JSON.stringify(settings, null, 2))

        ElMessage.success('全局设置已导出')

        // 如果设置了自动打开文件夹
        if (autoOpenFolder) {
            try {
                // 直接传入文件路径，让explorer选中这个文件
                await invoke('reveal_in_explorer', { filePath: finalExportPath })
            } catch (error) {
                console.error('打开文件夹失败:', error)
            }
        }
    } catch (error) {
        ElMessage.error('导出设置失败：' + error.message)
    } finally {
        exporting.value = false
    }
}

const triggerFileInput = () => {
    fileInputRef.value?.click()
}

const handleFileSelect = (e) => {
    const files = e.target.files
    if (files.length > 0) {
        handleFile(files[0])
    }
}

const handleFile = async (file) => {
    if (!file.name.endsWith('.json')) {
        ElMessage.error('请选择 JSON 格式的文件')
        return
    }

    try {
        const text = await file.text()
        const data = JSON.parse(text)

        if (!data.data) {
            ElMessage.error('文件格式不正确，缺少设置数据')
            return
        }

        await ElMessageBox.confirm(
            '导入设置将覆盖当前所有配置，是否继续？',
            '确认导入',
            {
                type: 'warning'
            }
        )

        // 导入设置到各个 store
        if (data.data.settings) {
            settingsStore.$patch(data.data.settings)
        }
        if (data.data.fileSearchSettings) {
            fileSearchStore.$patch(data.data.fileSearchSettings)
        }
        if (data.data.fileTypes) {
            fileTypesStore.$patch(data.data.fileTypes)
        }
        if (data.data.commitTypes) {
            commitTypesStore.$patch(data.data.commitTypes)
        }

        ElMessage.success('设置导入成功！')
    } catch (error) {
        ElMessage.error('导入失败：' + error.message)
    }
}

const confirmResetSettings = async () => {
    try {
        await ElMessageBox.confirm(
            '此操作将重置所有设置为默认值，包括主题、文件类型、搜索配置等。此操作不可撤销，确定要继续吗？',
            '重置所有设置',
            {
                type: 'error',
                confirmButtonText: '确定重置',
                cancelButtonText: '取消'
            }
        )

        resetting.value = true

        // 重置不同语法的 stores
        // Options API stores (有 $reset 方法)
        if (typeof settingsStore.$reset === 'function') {
            settingsStore.$reset()
        }
        if (typeof fileSearchStore.$reset === 'function') {
            fileSearchStore.$reset()
        }

        // Setup API stores (需要调用自定义重置方法)
        if (typeof fileTypesStore.resetToDefault === 'function') {
            fileTypesStore.resetToDefault()
        }

        ElMessage.success('所有设置已重置为默认值')
    } catch (error) {
        if (error !== 'cancel') {
            ElMessage.error('重置设置失败：' + error.message)
        }
    } finally {
        resetting.value = false
    }
}

const confirmClearCache = async () => {
    try {
        await ElMessageBox.confirm(
            '此操作将清除应用的所有缓存数据，包括浏览器缓存、图片缓存等，\n您的设置和配置数据不会受到影响，\n确定要继续吗？',
            '清除缓存',
            {
                type: 'warning',
                confirmButtonText: '确定清除',
                cancelButtonText: '取消'
            }
        )

        clearingCache.value = true

        // 备份 pinia 持久化存储
        const backupData = {
            'wem-settings': localStorage.getItem('wem-settings'),
            'wem-file-search-settings': localStorage.getItem('wem-file-search-settings'),
            'wem-file-types': localStorage.getItem('wem-file-types')
        }

        try {
            // 清除 webview 缓存
            const webview = getCurrentWebviewWindow()
            await webview.clearAllBrowsingData()

            // 恢复 pinia 持久化存储
            Object.entries(backupData).forEach(([key, value]) => {
                if (value) {
                    localStorage.setItem(key, value)
                }
            })

            // 记录清理时间
            settingsStore.setLastCacheClearTime(new Date().toISOString())

            // 重新加载缓存信息
            await loadCacheInfo()

            ElMessage.success('缓存已清除！')
        } catch (error) {
            // 如果清除失败，确保恢复备份
            Object.entries(backupData).forEach(([key, value]) => {
                if (value) {
                    localStorage.setItem(key, value)
                }
            })
            throw error
        }
    } catch (error) {
        if (error !== 'cancel') {
            console.error('清除缓存失败:', error)
            ElMessage.error('清除缓存失败：' + error.message)
        }
    } finally {
        clearingCache.value = false
    }
}

// 快捷键处理方法
const handleShortcutCapture = (event, key) => {
    event.preventDefault()

    const modifiers = []
    if (event.ctrlKey || event.metaKey) modifiers.push('Ctrl')
    if (event.shiftKey) modifiers.push('Shift')
    if (event.altKey) modifiers.push('Alt')

    let keyStr = event.key

    // 只按修饰键不处理
    if (keyStr === 'Control' || keyStr === 'Shift' || keyStr === 'Alt' || keyStr === 'Meta') {
        return
    }

    // 转换特殊键名
    if (keyStr === ' ') keyStr = 'Space'
    if (keyStr.length === 1) keyStr = keyStr.toUpperCase()

    // 处理功能键
    if (keyStr.startsWith('F') && keyStr.length <= 3) {
        shortcuts.value[key] = modifiers.length > 0
            ? modifiers.join('+') + '+' + keyStr
            : keyStr
        return
    }

    if (modifiers.length > 0) {
        shortcuts.value[key] = modifiers.join('+') + '+' + keyStr
    } else {
        shortcuts.value[key] = keyStr
    }
}

const saveShortcut = async (key) => {
    const shortcut = shortcuts.value[key]

    if (!shortcut) {
        ElMessage.warning('请先输入快捷键')
        return
    }

    try {
        // 获取路由映射
        const routeMap = {
            envVarManager: '/env-var',
            fileSearch: '/',
            projects: '/projects'
        }

        // 检查是否已被其他功能使用
        const otherKeys = Object.keys(shortcuts.value).filter(k => k !== key)
        for (const otherKey of otherKeys) {
            if (shortcuts.value[otherKey] === shortcut) {
                ElMessage.warning('该快捷键已被其他功能使用')
                return
            }
        }

        // 先取消旧的注册
        const oldShortcut = settingsStore.getGlobalShortcut(key)
        if (oldShortcut) {
            await unregisterShortcut(oldShortcut)
        }

        // 注册新快捷键
        await registerShortcut(key, shortcut)

        // 保存到设置
        settingsStore.setGlobalShortcut(key, shortcut)

        // 更新托盘菜单
        await updateTrayMenu()

        ElMessage.success(`快捷键 ${shortcut} 设置成功`)
    } catch (error) {
        console.error('设置快捷键失败:', error)
        ElMessage.error('设置失败：' + (error.message || error))
    }
}

const clearShortcut = async (key) => {
    const shortcut = settingsStore.getGlobalShortcut(key)

    if (shortcut) {
        await unregisterShortcut(shortcut)
    }

    shortcuts.value[key] = ''
    settingsStore.clearGlobalShortcut(key)

    // 更新托盘菜单
    await updateTrayMenu()

    ElMessage.success('快捷键已清除')
}

const getKeyLabel = (key) => {
    const labels = {
        envVarManager: '环境变量管理',
        fileSearch: '文件搜索',
        projects: '项目管理',
        commitGenerator: '提交生成器'
    }
    return labels[key] || key
}

// 更新托盘菜单显示快捷键
const updateTrayMenu = async () => {
    try {
        const commitTypes = commitTypesStore.allCommitTypes.map(ct => ({
            value: ct.value,
            label: ct.label,
            icon: ct.icon
        }))

        await invoke('update_tray_menu_with_commit_types', {
            envVarManager: shortcuts.value.envVarManager || '',
            fileSearch: shortcuts.value.fileSearch || '',
            projects: shortcuts.value.projects || '',
            commitGenerator: shortcuts.value.commitGenerator || '',
            commitTypes
        })
    } catch (error) {
        console.warn('更新托盘菜单失败:', error)
    }
}

const initShortcuts = () => {
    // 从 store 加载快捷键
    shortcuts.value.envVarManager = settingsStore.getGlobalShortcut('envVarManager')
    shortcuts.value.fileSearch = settingsStore.getGlobalShortcut('fileSearch')
    shortcuts.value.projects = settingsStore.getGlobalShortcut('projects')
    shortcuts.value.commitGenerator = settingsStore.getGlobalShortcut('commitGenerator')
}

// 注意：registerAllShortcuts 现在在 App.vue 中全局调用
// 这里保留此函数用于测试或手动重新注册
const registerAllShortcuts = async () => {
    // 从 utils/shortcutManager 导入并调用
    const { registerAllShortcuts: registerAll } = await import('@/utils/shortcutManager')
    await registerAll()
}

// 显示提交类型管理器
const showCommitTypesManager = () => {
    commitTypesManagerRef.value?.showDialog()
}

// Lifecycle
onMounted(async () => {
    loadCacheInfo()
    initShortcuts()
    // 注意：全局快捷键已在 App.vue 中注册，这里只是更新托盘菜单显示
    await updateTrayMenu()
})
</script>

<style lang="scss" scoped>
@use "../assets/styles/variables.scss" as *;


.settings-page {
    padding: var(--spacing-xl);
    margin-top: 20px;

    .settings-content {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);

        .settings-card {
            .card-header {
                display: flex;
                align-items: center;
                gap: var(--spacing-sm);
                font-weight: 600;

                .header-icon {
                    color: var(--el-color-primary);
                    font-size: 1.2rem;
                }
            }

            .card-content {
                .card-description {
                    color: var(--el-text-color-regular);
                    margin-bottom: var(--spacing-lg);
                    line-height: 1.6;
                }

                .setting-tip {
                    margin-top: 4px;
                    font-size: 12px;
                    color: var(--el-text-color-secondary);
                    line-height: 1.4;
                }

                .cache-info {
                    background: var(--el-fill-color-light);
                    border-radius: var(--el-border-radius-base);
                    padding: var(--spacing-md);
                    margin-bottom: var(--spacing-lg);

                    .info-item {
                        display: flex;
                        align-items: center;
                        gap: var(--spacing-sm);
                        padding: var(--spacing-xs) 0;

                        &:not(:last-child) {
                            border-bottom: 1px solid var(--el-border-color-lighter);
                        }

                        .info-label {
                            font-weight: 500;
                            color: var(--el-text-color-regular);
                            min-width: 80px;
                        }

                        .info-value {
                            flex: 1;
                            color: var(--el-text-color-primary);

                            &.path-value {
                                font-family: 'Consolas', 'Monaco', monospace;
                                font-size: 12px;
                                word-break: break-all;
                                color: var(--el-text-color-secondary);
                            }
                        }
                    }
                }

                .action-buttons {
                    display: flex;
                    gap: var(--spacing-md);
                    align-items: center;
                }

                .file-input {
                    display: none;
                }

                // 快捷键设置样式
                .shortcuts-settings {
                    display: flex;
                    flex-direction: column;
                    gap: var(--spacing-lg);
                    margin-bottom: var(--spacing-lg);

                    .shortcut-item {
                        display: flex;
                        align-items: center;
                        gap: var(--spacing-md);

                        .shortcut-label {
                            display: flex;
                            align-items: center;
                            gap: var(--spacing-xs);
                            min-width: 150px;
                            font-weight: 500;
                            color: var(--el-text-color-primary);

                            .el-icon {
                                color: var(--el-color-primary);
                                font-size: 1.1rem;
                            }
                        }

                        .shortcut-input {
                            flex: 1;
                            max-width: 400px;

                            :deep(.el-input-group__append) {
                                padding: 0;

                                .el-button {
                                    margin: 0;
                                }
                            }

                            .el-input {
                                font-family: 'Consolas', 'Monaco', monospace;
                            }
                        }
                    }
                }

                .shortcut-tips {
                    margin-top: var(--spacing-md);

                    :deep(.el-alert__title) {
                        div {
                            line-height: 1.8;
                            font-size: 0.875rem;
                        }
                    }
                }
            }
        }
    }
}

// 响应式设计
@media (max-width: 768px) {
    .settings-page {
        padding: var(--spacing-md);

        .settings-content .action-buttons {
            flex-direction: column;
            width: 100%;

            .el-button {
                width: 100%;
            }
        }
    }
}
</style>