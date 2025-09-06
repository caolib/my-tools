<template>
    <el-dialog v-model="localVisible" title="配置历史" width="900px" :close-on-click-modal="false"
        class="config-history-dialog">
        <div class="dialog-content">
            <!-- 扫描状态和路径信息 -->
            <div class="scan-info" v-if="scanPath">
                <el-alert type="info" :closable="false" show-icon>
                    <template #title>
                        <div class="scan-path-info">
                            <span>扫描路径: </span>
                            <el-button type="primary" text @click="openFolder" class="path-button">
                                {{ scanPath }}
                                <el-icon style="margin-left: 4px;">
                                    <FolderOpened />
                                </el-icon>
                            </el-button>
                        </div>
                    </template>
                </el-alert>
            </div>

            <!-- 加载状态 -->
            <div v-if="loading" class="loading-container">
                <el-skeleton :rows="5" animated />
            </div>

            <!-- 配置文件列表 -->
            <div v-else-if="configFiles.length > 0" class="config-list">
                <el-table :data="configFiles" stripe @row-click="selectConfig" class="config-table"
                    row-class-name="clickable-row">
                    <el-table-column label="文件名" min-width="200">
                        <template #default="{ row }">
                            <div class="file-info">
                                <el-icon class="file-icon">
                                    <Document />
                                </el-icon>
                                <span class="file-name">{{ row.file_name }}</span>
                            </div>
                        </template>
                    </el-table-column>

                    <el-table-column label="导出时间" width="160" sortable prop="export_time">
                        <template #default="{ row }">
                            <div class="time-info">
                                <el-icon>
                                    <Clock />
                                </el-icon>
                                <span>{{ row.export_time }}</span>
                            </div>
                        </template>
                    </el-table-column>

                    <el-table-column label="变量数量" width="120">
                        <template #default="{ row }">
                            <div class="count-info">
                                <el-tag size="small" type="primary">系统: {{ row.system_vars_count }}</el-tag>
                                <el-tag size="small" type="success">用户: {{ row.user_vars_count }}</el-tag>
                            </div>
                        </template>
                    </el-table-column>

                    <el-table-column label="文件大小" width="100">
                        <template #default="{ row }">
                            <span class="file-size">{{ formatFileSize(row.file_size) }}</span>
                        </template>
                    </el-table-column>

                    <el-table-column label="操作" width="80" fixed="right">
                        <template #default="{ row }">
                            <el-button size="small" type="primary" @click.stop="importConfig(row)"
                                :loading="importing === row.file_path">
                                导入
                            </el-button>
                        </template>
                    </el-table-column>
                </el-table>
            </div>

            <!-- 无配置文件提示 -->
            <div v-else class="no-configs">
                <el-empty description="未找到配置文件" :image-size="100">
                    <template #description>
                        <p>在指定路径下未找到环境变量配置文件</p>
                        <p class="hint">请确保路径正确，且文件为 JSON 格式</p>
                    </template>
                </el-empty>
            </div>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button @click="refreshConfigs" :loading="loading" :icon="Refresh">
                    刷新
                </el-button>
                <el-button @click="localVisible = false">
                    关闭
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'
import {
    Document,
    Clock,
    User,
    Monitor,
    Refresh,
    FolderOpened
} from '@element-plus/icons-vue'

const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false
    }
})

const emit = defineEmits(['update:modelValue', 'imported'])

const settingsStore = useSettingsStore()

const localVisible = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
})

const configFiles = ref([])
const loading = ref(false)
const importing = ref('')
const scanPath = computed(() => settingsStore.exportPath)

// 监听对话框显示状态，显示时自动扫描
watch(localVisible, (visible) => {
    if (visible) {
        scanConfigs()
    }
})

// 扫描配置文件
const scanConfigs = async () => {
    if (!scanPath.value) {
        ElMessage.warning('请先在设置中配置导出路径')
        return
    }

    loading.value = true
    try {
        const result = await invoke('scan_config_files', {
            folderPath: scanPath.value
        })
        configFiles.value = result

        if (result.length === 0) {
            ElMessage.info('未找到配置文件')
        } else {
            ElMessage.success(`找到 ${result.length} 个配置文件`)
        }
    } catch (error) {
        ElMessage.error(`扫描配置文件失败: ${error}`)
        console.error('Error scanning config files:', error)
    } finally {
        loading.value = false
    }
}

// 刷新配置列表
const refreshConfigs = () => {
    scanConfigs()
}

// 打开文件夹
const openFolder = async () => {
    if (scanPath.value) {
        try {
            await invoke('open_folder', { folderPath: scanPath.value })
        } catch (error) {
            ElMessage.error(`打开文件夹失败: ${error}`)
            console.error('Error opening folder:', error)
        }
    }
}

// 选择配置（点击行时）
const selectConfig = (row) => {
    // 可以在这里添加预览功能
    console.log('Selected config:', row)
}

// 导入配置
const importConfig = async (config) => {
    try {
        await ElMessageBox.confirm(
            `确定要导入配置文件 "${config.file_name}" 吗？这将覆盖同名的环境变量。`,
            '确认导入',
            {
                type: 'warning',
                confirmButtonText: '确定导入',
                cancelButtonText: '取消'
            }
        )

        importing.value = config.file_path
        const result = await invoke('import_env_vars', {
            filePath: config.file_path
        })

        ElMessage.success(result)
        emit('imported') // 通知父组件刷新环境变量
        localVisible.value = false
    } catch (error) {
        if (error !== 'cancel') {
            ElMessage.error(`导入失败: ${error}`)
            console.error('Import error:', error)
        }
    } finally {
        importing.value = ''
    }
}

// 格式化文件大小
const formatFileSize = (bytes) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}
</script>

<style lang="scss" scoped>
.config-history-dialog {
    .dialog-content {
        min-height: 400px;
    }

    .scan-info {
        margin-bottom: var(--spacing-md);

        .scan-path-info {
            display: flex;
            align-items: center;

            .path-button {
                padding: 2px 8px;
                font-family: monospace;
                border-radius: 4px;
                transition: all 0.2s;

                &:hover {
                    background-color: var(--el-color-primary-light-8);
                }
            }
        }
    }

    .loading-container {
        padding: var(--spacing-lg);
    }

    .config-list {
        .config-table {
            :deep(.clickable-row) {
                cursor: pointer;

                &:hover {
                    background-color: var(--el-table-row-hover-bg-color);
                }
            }

            .file-info {
                display: flex;
                align-items: center;
                gap: var(--spacing-xs);

                .file-icon {
                    color: var(--el-color-primary);
                }

                .file-name {
                    font-weight: 500;
                }
            }

            .time-info,
            .user-info,
            .computer-info {
                display: flex;
                align-items: center;
                gap: var(--spacing-xs);
                font-size: var(--font-size-sm);

                .el-icon {
                    color: var(--el-color-info);
                }
            }

            .count-info {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }

            .file-size {
                font-size: var(--font-size-sm);
                color: var(--el-color-info);
                font-family: monospace;
            }
        }
    }

    .no-configs {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 300px;

        .hint {
            color: var(--el-color-info);
            font-size: var(--font-size-sm);
            margin-top: var(--spacing-xs);
        }
    }

    .dialog-footer {
        display: flex;
        justify-content: flex-end;
        gap: var(--spacing-sm);
    }
}
</style>
