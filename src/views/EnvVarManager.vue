<template>
    <div class="env-manager">
        <!-- 自定义标题栏 -->
        <Titlebar />

        <!-- 主内容区域 -->
        <div class="main-content">
            <!-- 搜索和操作栏 -->
            <div class="toolbar">
                <div class="search-section">
                    <el-input v-model="searchText" placeholder="搜索变量名/值/全部" clearable size="default"
                        style="width: 500px" @input="onSearchInput">
                        <template #append>
                            <el-select v-model="searchType" size="default" style="width: 100px" @change="onSearchInput">
                                <el-option label="全部" value="all" />
                                <el-option label="变量名" value="name" />
                                <el-option label="变量值" value="value" />
                            </el-select>
                        </template>
                    </el-input>
                </div>

                <div class="action-section">
                    <!-- 导入导出按钮 -->
                    <el-dropdown @command="handleImportExport" trigger="click">
                        <el-button size="default" :icon="Setting">
                            配置
                            <el-icon>
                                <ArrowDown />
                            </el-icon>
                        </el-button>
                        <template #dropdown>
                            <el-dropdown-menu>
                                <el-dropdown-item command="settings">
                                    <el-icon>
                                        <Setting />
                                    </el-icon>
                                    设置
                                </el-dropdown-item>
                                <el-dropdown-item command="history">
                                    <el-icon>
                                        <Clock />
                                    </el-icon>
                                    配置历史
                                </el-dropdown-item>
                                <el-dropdown-item command="export">
                                    <el-icon>
                                        <Download />
                                    </el-icon>
                                    导出配置
                                </el-dropdown-item>
                                <el-dropdown-item command="import">
                                    <el-icon>
                                        <Upload />
                                    </el-icon>
                                    导入配置
                                </el-dropdown-item>
                            </el-dropdown-menu>
                        </template>
                    </el-dropdown>

                    <!-- 刷新按钮 -->
                    <el-button @click="loadEnvVars" size="default" :icon="Refresh" :loading="loading">
                        刷新
                    </el-button>

                    <!-- 管理员权限状态 -->
                    <el-tag v-if="!isAdmin" type="warning" style="cursor: pointer" round
                        @click="requestAdminPrivileges">
                        没有以管理员身份运行
                    </el-tag>
                    <el-tag v-else type="success">
                        <el-icon>
                            <Check />
                        </el-icon>
                        管理员
                    </el-tag>
                </div>
            </div>
            <el-collapse v-model="activeCollapse" class="collapse-container">
                <!-- 系统变量 -->
                <el-collapse-item name="system" class="collapse-item">
                    <template #title>
                        <div class="collapse-header">
                            <div class="header-info">
                                <el-icon>
                                    <Monitor />
                                </el-icon>
                                <div class="header-text">
                                    <h2 class="section-title">系统环境变量</h2>
                                    <p class="section-desc">{{ systemVars.length }} 个变量</p>
                                </div>
                            </div>
                            <div class="header-actions">
                                <el-button type="primary" text @click.stop="showAddSystemDialog" class="add-btn"
                                    :disabled="!isAdmin">
                                    <el-icon>
                                        <Plus />
                                    </el-icon>
                                    添加
                                </el-button>
                            </div>
                        </div>
                    </template>
                    <div class="vars-container">
                        <EnvVarCard v-for="row in filteredSystemVars" :key="row.name" :env-var="row" :is-admin="isAdmin"
                            :disable-edit="!isAdmin || row.name === 'Path' && !isAdmin" :highlight="searchText"
                            :highlight-type="searchType" @edit="(row) => editVar(row, 'system')"
                            @delete="(row) => deleteVar(row, 'system')" />
                    </div>
                </el-collapse-item>

                <!-- 用户变量 -->
                <el-collapse-item name="user" class="collapse-item">
                    <template #title>
                        <div class="collapse-header">
                            <div class="header-info">
                                <el-icon class="user-icon">
                                    <User />
                                </el-icon>
                                <div class="header-text">
                                    <h2 class="section-title">用户环境变量</h2>
                                    <p class="section-desc">{{ userVars.length }} 个变量</p>
                                </div>
                            </div>
                            <div class="header-actions">
                                <el-button type="primary" @click.stop="showAddUserDialog" class="add-btn" text>
                                    <el-icon>
                                        <Plus />
                                    </el-icon>
                                    添加
                                </el-button>
                            </div>
                        </div>
                    </template>
                    <div class="vars-container">
                        <EnvVarCard v-for="row in filteredUserVars" :key="row.name" :env-var="row" :is-admin="true"
                            :disable-edit="false" :highlight="searchText" :highlight-type="searchType"
                            @edit="(row) => editVar(row, 'user')" @delete="(row) => deleteVar(row, 'user')" />
                    </div>
                </el-collapse-item>
            </el-collapse>

            <!-- 添加/编辑对话框 -->
            <el-dialog v-model="showAddDialog" title="编辑环境变量" width="500px" :close-on-click-modal="false"
                class="custom-dialog">
                <el-form :model="newVarForm" label-width="0" class="form-container">
                    <el-form-item required>
                        <el-input v-model="newVarForm.name" placeholder="变量名" size="large" />
                    </el-form-item>
                    <el-form-item required>
                        <el-input v-model="newVarForm.value" placeholder="变量值" type="textarea" :rows="10"
                            size="large" />
                    </el-form-item>
                    <el-alert v-if="newVarForm.scope === 'system'" title="注意：修改系统环境变量需要管理员权限" type="warning"
                        :closable="false" show-icon />
                </el-form>
                <template #footer>
                    <div class="dialog-footer">
                        <el-button @click="cancelEdit" size="large">取消</el-button>
                        <el-button @click="addVar" :loading="submitting" size="large">{{ isEditing ? '更新' : '添加'
                        }}</el-button>
                    </div>
                </template>
            </el-dialog>
        </div>
        <!-- 设置对话框 -->
        <SettingsDialog v-model="showSettingsDialog" />

        <!-- 配置历史对话框 -->
        <ConfigHistoryDialog v-model="showHistoryDialog" @imported="loadEnvVars" />
    </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { save, open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { join } from '@tauri-apps/api/path'
import { getVersion } from '@tauri-apps/api/app'
import {
    Plus,
    Refresh,
    Monitor,
    User,
    Edit,
    Delete,
    Moon,
    Sunny,
    Setting,
    ArrowDown,
    Download,
    Upload,
    Check,
    Clock
} from '@element-plus/icons-vue'
import EnvVarCard from '../components/EnvVarCard.vue'
import Titlebar from '../components/Titlebar.vue'
import SettingsDialog from '../components/SettingsDialog.vue'
import ConfigHistoryDialog from '../components/ConfigHistoryDialog.vue'

const systemVars = ref([])
const userVars = ref([])
const searchText = ref('')
const searchType = ref('all')

// 使用Pinia store管理折叠状态
const settingsStore = useSettingsStore()
const activeCollapse = computed({
    get: () => settingsStore.collapsedKeys,
    set: (keys) => settingsStore.setCollapsedKeys(keys)
})
// 过滤后的变量，并将 Path 变量置顶
function sortPathFirst(vars) {
    const idx = vars.findIndex(v => v.name === 'Path')
    if (idx === -1) return vars
    return [vars[idx], ...vars.slice(0, idx), ...vars.slice(idx + 1)]
}

const filteredSystemVars = computed(() => {
    let arr = []
    if (!searchText.value) {
        arr = systemVars.value
    } else if (searchType.value === 'all') {
        arr = systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
    } else if (searchType.value === 'name') {
        arr = systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
    } else if (searchType.value === 'value') {
        arr = systemVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
    } else {
        arr = systemVars.value
    }
    return sortPathFirst(arr)
})

const filteredUserVars = computed(() => {
    let arr = []
    if (!searchText.value) {
        arr = userVars.value
    } else if (searchType.value === 'all') {
        arr = userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
    } else if (searchType.value === 'name') {
        arr = userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
    } else if (searchType.value === 'value') {
        arr = userVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
    } else {
        arr = userVars.value
    }
    return sortPathFirst(arr)
})

// 处理搜索事件
const onSearchInput = () => {
    // 搜索时自动展开所有面板
    if (searchText.value) {
        activeCollapse.value = ['system', 'user']
    }
}

// 处理配置操作
const handleImportExport = (command) => {
    if (command === "settings") {
        showSettingsDialog.value = true;
    } else if (command === "history") {
        showHistoryDialog.value = true;
    } else if (command === "export") {
        exportEnvVars();
    } else if (command === "import") {
        importEnvVars();
    }
};
const showAddDialog = ref(false)
const showSettingsDialog = ref(false)
const showHistoryDialog = ref(false)
const loading = ref(false)
const submitting = ref(false)
const isAdmin = ref(false) // 管理员权限状态

const newVarForm = ref({
    name: '',
    value: '',
    scope: 'user'
})

const editMode = ref(false)
const editingVar = ref(null)

// 显示添加系统变量对话框
const showAddSystemDialog = () => {
    newVarForm.value = { name: '', value: '', scope: 'system' }
    editMode.value = false
    showAddDialog.value = true
}

// 显示添加用户变量对话框
const showAddUserDialog = () => {
    newVarForm.value = { name: '', value: '', scope: 'user' }
    editMode.value = false
    showAddDialog.value = true
}




// 检测管理员权限
const checkAdminPrivileges = async () => {
    try {
        // 调用后端检测管理员权限
        const result = await invoke('check_admin_privileges')
        isAdmin.value = result
    } catch (error) {
        isAdmin.value = false
        console.log('检测管理员权限失败:', error)
    }
}

// 请求管理员权限
const requestAdminPrivileges = async () => {
    try {
        await ElMessageBox.confirm(
            '应用将以管理员身份重新启动，是否继续？',
            '管理员权限确认',
            {
                type: 'warning',
                confirmButtonText: '确定',
                cancelButtonText: '取消'
            }
        )

        // 调用后端以管理员身份重启（已集成窗口状态保存）
        await invoke('request_admin_privileges')
    } catch (error) {
        if (error !== 'cancel') {
            ElMessage.error(`请求管理员权限失败: ${error}`)
            console.error('Error requesting admin privileges:', error)
        }
    }
}

// 仅在编辑时为 true
const isEditing = computed(() => editMode.value)

// 加载真实的环境变量
const loadEnvVars = async () => {
    loading.value = true
    try {
        const result = await invoke('get_env_vars')
        systemVars.value = result.system_vars || []
        userVars.value = result.user_vars || []
        ElMessage({
            message: `成功加载 ${systemVars.value.length + userVars.value.length} 个环境变量`,
            grouping: true,
            offset: 40,
            duration: 1000,
            type: 'success',
        })
    } catch (error) {
        ElMessage.error(`获取环境变量失败: ${error}`)
        console.error('Error loading env vars:', error)
    } finally {
        loading.value = false
    }
}

const editVar = (row, scope) => {
    newVarForm.value = {
        ...row,
        scope: scope
    }
    editMode.value = true
    showAddDialog.value = true
}

const cancelEdit = () => {
    showAddDialog.value = false
    newVarForm.value = { name: '', value: '', scope: 'user' }
    editMode.value = false
}

const deleteVar = async (row, scope) => {
    try {
        const isSystem = scope === 'system'
        await invoke('delete_env_var', {
            name: row.name,
            isSystem
        })

        ElMessage.success(`变量 "${row.name}" 删除成功`)
        await loadEnvVars() // 重新加载环境变量
    } catch (error) {
        ElMessage.error(`删除失败: ${error}`)
        console.error('Error deleting env var:', error)
    }
}

const addVar = async () => {
    if (!newVarForm.value.name || !newVarForm.value.value) {
        ElMessage.warning('请填写完整的变量信息')
        return
    }

    // 检查变量是否已存在
    const targetVars = newVarForm.value.scope === 'system' ? systemVars.value : userVars.value
    const existingVar = targetVars.find(v => v.name === newVarForm.value.name)

    if (existingVar && !isEditing.value) {
        try {
            await ElMessageBox.confirm(
                `变量 "${newVarForm.value.name}" 已存在，是否覆盖？`,
                '变量已存在',
                {
                    type: 'warning',
                    confirmButtonText: '覆盖',
                    cancelButtonText: '取消'
                }
            )
        } catch (error) {
            if (error === 'cancel') return
        }
    }

    submitting.value = true
    try {
        // 调用 Tauri 后端 API 添加变量
        await invoke('set_env_var', {
            name: newVarForm.value.name,
            value: newVarForm.value.value,
            isSystem: newVarForm.value.scope === 'system'
        })

        const action = isEditing.value ? '更新' : '添加'
        ElMessage.success(`变量 "${newVarForm.value.name}" ${action}成功`)
        showAddDialog.value = false
        newVarForm.value = { name: '', value: '', scope: 'user' }
        await loadEnvVars() // 重新加载环境变量
    } catch (error) {
        const action = isEditing.value ? '更新' : '添加'
        ElMessage.error(`${action}失败: ${error}`)
        console.error('Error adding env var:', error)
    } finally {
        submitting.value = false
    }
}


// 初始化
onMounted(() => {
    checkAdminPrivileges()
    loadEnvVars()
    // 折叠状态已从Pinia store自动加载
})

// 导出环境变量
const exportEnvVars = async () => {
    try {
        loading.value = true

        // 直接使用settingsStore中的设置
        const exportPath = settingsStore.exportPath
        const autoOpenFolder = settingsStore.autoOpenFolder

        // 使用默认文件名格式：环境变量备份_时间戳.json
        const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)
        const defaultFileName = `环境变量备份_${timestamp}.json`

        let finalExportPath = ''

        // 如果有设置默认路径，直接使用
        if (exportPath) {
            finalExportPath = await join(exportPath, defaultFileName)
        } else {
            // 否则让用户选择路径
            const documentsDir = await invoke('get_documents_dir')
            const selected = await save({
                title: '导出环境变量',
                defaultPath: join(documentsDir, defaultFileName),
                filters: [
                    {
                        name: 'JSON文件',
                        extensions: ['json']
                    }
                ]
            })

            if (!selected) {
                loading.value = false
                return
            }
            finalExportPath = selected
        }

        // 获取所有环境变量数据 - 使用简化格式
        const now = new Date()
        const localTime = new Date(now.getTime() - now.getTimezoneOffset() * 60000).toISOString().replace('T', ' ').slice(0, 19)

        const data = {
            export_info: {
                export_time: localTime,
                version: await getVersion()
            },
            system_vars: systemVars.value,
            user_vars: userVars.value
        }

        // 写入文件
        await writeTextFile(finalExportPath, JSON.stringify(data, null, 2))

        ElMessage.success('环境变量已导出')

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
        console.error('导出失败:', error)
        ElMessage.error({
            message: '导出失败，可能是未设置导出路径',
            duration: 3000
        })
    } finally {
        loading.value = false
    }
}

// 导入环境变量配置
const importEnvVars = async () => {
    try {
        // 打开文件选择对话框
        const selected = await open({
            title: '选择环境变量配置文件',
            filters: [{
                name: 'JSON 配置文件',
                extensions: ['json']
            }],
            multiple: false
        })

        if (!selected) {
            return // 用户取消选择
        }

        // 确认导入
        await ElMessageBox.confirm(
            '导入配置会覆盖已存在的同名环境变量，是否继续？',
            '确认导入',
            {
                type: 'warning',
                confirmButtonText: '确定导入',
                cancelButtonText: '取消'
            }
        )

        // 调用后端导入
        const result = await invoke('import_env_vars', { filePath: selected })
        ElMessage.success(result)

        // 重新加载环境变量
        await loadEnvVars()
    } catch (error) {
        if (error !== 'cancel') {
            ElMessage.error(`导入失败: ${error}`)
            console.error('Import error:', error)
        }
    }
}
</script>

<style lang="scss" scoped src="../assets/styles/env-var-manager.scss"></style>
