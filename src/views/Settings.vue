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
                        <el-form-item label="使用编辑器打开项目时将自动关闭本应用">
                            <el-switch v-model="settingsStore.closeAfterOpenProject" />
                        </el-form-item>
                    </el-form>
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
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
    Setting,
    Download,
    Upload,
    Delete,
    FolderOpened
} from '@element-plus/icons-vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'
import { useFileSearchSettingsStore } from '@/stores/fileSearchSettings'
import { useFileTypesStore } from '@/stores/fileTypes'

// Stores
const settingsStore = useSettingsStore()
const fileSearchStore = useFileSearchSettingsStore()
const fileTypesStore = useFileTypesStore()

// Reactive data
const exporting = ref(false)
const resetting = ref(false)
const clearingCache = ref(false)
const fileInputRef = ref(null)
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

        const allSettings = {
            settings: settingsStore.$state,
            fileSearchSettings: fileSearchStore.$state,
            fileTypes: fileTypesStore.$state
        }

        const settings = {
            exportDate: new Date().toISOString(),
            version: '1.0.0',
            data: allSettings
        }

        const blob = new Blob([JSON.stringify(settings, null, 2)], {
            type: 'application/json'
        })

        const url = URL.createObjectURL(blob)
        const a = document.createElement('a')
        a.href = url
        a.download = `mytools-settings-backup-${new Date().toISOString().split('T')[0]}.json`
        document.body.appendChild(a)
        a.click()
        document.body.removeChild(a)
        URL.revokeObjectURL(url)

        ElMessage.success('设置导出成功！')
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
            'fileSearchSettings': localStorage.getItem('fileSearchSettings'),
            'fileTypes': localStorage.getItem('fileTypes')
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

// Lifecycle
onMounted(() => {
    loadCacheInfo()
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
        gap: var(--spacing-lg);

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