<template>
    <el-dialog v-model="dialogVisible" width="80vw">
        <div class="commit-types-manager">
            <div style="margin-bottom: 10px;">
                <el-button type="primary" :icon="Plus" @click="showAddDialog">
                    添加提交类型
                </el-button>
                <el-button @click="handleReset">重置为默认</el-button>
            </div>

            <el-table :data="commitTypes" border style="width: 100%">
                <el-table-column prop="icon" label="图标" width="60" align="center" />
                <el-table-column prop="value" label="类型值" width="120" />
                <el-table-column prop="label" label="标签" />
                <el-table-column label="操作" width="140" align="center">
                    <template #default="{ row }">
                        <div style="display: flex; gap: 4px; justify-content: center;">
                            <el-button link type="primary" size="small" :icon="Edit" @click="showEditDialog(row)">
                                编辑
                            </el-button>
                            <el-button v-if="row.isCustom" link type="danger" size="small" :icon="Delete"
                                @click="handleDelete(row)">
                                删除
                            </el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>

        <!-- 添加/编辑对话框 -->
        <el-dialog v-model="formDialogVisible" :title="isEditing ? '编辑提交类型' : '添加提交类型'" width="400px" append-to-body>
            <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
                <el-form-item label="类型值" prop="value">
                    <el-input v-model="form.value" :disabled="isEditing" placeholder="例如：feat" />
                </el-form-item>
                <el-form-item label="标签" prop="label">
                    <el-input v-model="form.label" placeholder="例如：feat: 新功能" />
                </el-form-item>
                <el-form-item label="图标" prop="icon">
                    <el-input v-model="form.icon" placeholder="例如：✨" @click="openEmojiPicker" />
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button @click="formDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleSubmit">确定</el-button>
            </template>
        </el-dialog>
    </el-dialog>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { useCommitTypesStore } from '@/stores/commitTypes'

const commitTypesStore = useCommitTypesStore()
const dialogVisible = ref(false)
const formDialogVisible = ref(false)
const isEditing = ref(false)
const formRef = ref(null)

const commitTypes = computed(() => commitTypesStore.allCommitTypes)

const form = ref({
    value: '',
    label: '',
    icon: ''
})

const rules = {
    value: [
        { required: true, message: '请输入类型值', trigger: 'blur' }
    ],
    label: [
        { required: true, message: '请输入标签', trigger: 'blur' }
    ],
    icon: [
        { required: true, message: '请输入图标', trigger: 'blur' }
    ]
}

const showDialog = () => {
    dialogVisible.value = true
}

const showAddDialog = () => {
    isEditing.value = false
    form.value = {
        value: '',
        label: '',
        icon: ''
    }
    formDialogVisible.value = true
}

const showEditDialog = (row) => {
    isEditing.value = true
    form.value = {
        value: row.value,
        label: row.label,
        icon: row.icon
    }
    formDialogVisible.value = true
}

const handleSubmit = async () => {
    if (!formRef.value) return

    await formRef.value.validate((valid) => {
        if (valid) {
            const { value, label, icon } = form.value

            let result
            if (isEditing.value) {
                result = commitTypesStore.updateCommitType(value, label, icon)
            } else {
                result = commitTypesStore.addCommitType(value, label, icon)
            }

            if (result.success) {
                ElMessage.success(result.message)
                formDialogVisible.value = false

                // 触发托盘菜单更新
                updateTrayMenu()
            } else {
                ElMessage.error(result.message)
            }
        }
    })
}

const handleDelete = (row) => {
    ElMessageBox.confirm(
        `确定要删除提交类型 "${row.label}" 吗？`,
        '确认删除',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        }
    ).then(() => {
        const result = commitTypesStore.deleteCommitType(row.value)
        if (result.success) {
            ElMessage.success(result.message)

            // 触发托盘菜单更新
            updateTrayMenu()
        } else {
            ElMessage.error(result.message)
        }
    }).catch(() => {
        // 取消删除
    })
}

const handleReset = () => {
    ElMessageBox.confirm(
        '确定要重置为默认提交类型吗？这将删除所有自定义类型。',
        '确认重置',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        }
    ).then(() => {
        commitTypesStore.resetToDefault()
        ElMessage.success('已重置为默认提交类型')

        // 触发托盘菜单更新
        updateTrayMenu()
    }).catch(() => {
        // 取消重置
    })
}

// 打开表情符号选择器（Windows: Win+.）
const openEmojiPicker = () => {
    // 模拟 Win+. 快捷键打开 Windows 表情符号面板
    const event = new KeyboardEvent('keydown', {
        key: '.',
        code: 'Period',
        metaKey: true,  // Windows 键
        bubbles: true
    })
    document.dispatchEvent(event)
}

// 更新托盘菜单
const updateTrayMenu = async () => {
    try {
        const { invoke } = await import('@tauri-apps/api/core')
        const { useSettingsStore } = await import('@/stores/settings')
        const settingsStore = useSettingsStore()

        await invoke('update_tray_menu_with_commit_types', {
            envVarManager: settingsStore.globalShortcuts.envVarManager || '',
            fileSearch: settingsStore.globalShortcuts.fileSearch || '',
            projects: settingsStore.globalShortcuts.projects || '',
            commitGenerator: settingsStore.globalShortcuts.commitGenerator || '',
            commitTypes: commitTypes.value.map(ct => ({
                value: ct.value,
                label: ct.label,
                icon: ct.icon
            }))
        })
    } catch (error) {
        console.error('更新托盘菜单失败:', error)
    }
}

defineExpose({
    showDialog
})
</script>
