<template>
    <div class="settings-page">
        <div class="settings-content">
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
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
    Setting,
    Download,
    Upload,
    Delete
} from '@element-plus/icons-vue'
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
const fileInputRef = ref(null)

// Methods
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
</script>

<style lang="scss" scoped>
@use "../assets/styles/variables.scss" as *;


.settings-page {
    padding: var(--spacing-xl);
    margin-top: 20px;

    .settings-content {
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