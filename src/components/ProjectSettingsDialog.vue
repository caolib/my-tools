<template>
    <el-dialog v-model="visible" title="配置文件路径设置" :close-on-click-modal="false" width="fit-content"
        class="project-settings-dialog">
        <el-form label-position="left" class="project-settings-form">
            <div class="vscode-setting-row">
                <img src="/code.png" class="vscode-icon" alt="vscode" />
                <div class="path-input-group">
                    <el-input :value="displayPath" readonly class="path-input" :title="displayPath">
                        <template #append>
                            <el-button @click="selectVscodeStorage" :icon="FolderOpened"
                                title="选择 storage.json"></el-button>
                        </template>
                    </el-input>
                    <el-button v-if="settingsStore.vscodeStoragePath" @click="clearCustomPath">清除</el-button>
                </div>
            </div>
        </el-form>

        <template #footer>
            <span class="dialog-footer">
                <el-button @click="visible = false">关闭</el-button>
                <el-button type="primary" @click="handleSave" :loading="saving">保存</el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { Folder, FolderOpened } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { homeDir } from '@tauri-apps/api/path'
import { ElMessage } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'

const props = defineProps({
    modelValue: { type: Boolean, default: false }
})
const emit = defineEmits(['update:modelValue', 'saved'])
const settingsStore = useSettingsStore()
const saving = ref(false)

const visible = computed({
    get: () => props.modelValue,
    set: v => emit('update:modelValue', v)
})

const defaultVscodeStorage = ref('')
const displayPath = computed(() => settingsStore.vscodeStoragePath || defaultVscodeStorage.value || '加载中...')

onMounted(async () => {
    try {
        const home = await homeDir()
        defaultVscodeStorage.value = `${home}AppData/Roaming/Code/User/globalStorage/storage.json`.replace(/\\/g, '/')
    } catch {
        defaultVscodeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Code/User/globalStorage/storage.json'
    }
})

const persist = () => {
    settingsStore.$patch({ vscodeStoragePath: settingsStore.vscodeStoragePath })
}

const selectVscodeStorage = async () => {
    try {
        const selected = await open({
            title: '选择 VSCode storage.json',
            multiple: false,
            directory: false,
            filters: [{ name: 'JSON', extensions: ['json'] }]
        })
        if (selected) {
            settingsStore.vscodeStoragePath = selected
            persist()
        }
    } catch (e) {
        ElMessage.error('选择文件失败: ' + e)
    }
}

const clearCustomPath = () => {
    settingsStore.vscodeStoragePath = ''
    persist()
}

const handleSave = async () => {
    saving.value = true
    try {
        settingsStore.$patch({ vscodeStoragePath: settingsStore.vscodeStoragePath })
        ElMessage.success('保存成功')
        emit('saved')
        visible.value = false
    } catch (e) {
        ElMessage.error('保存失败: ' + e)
    } finally {
        saving.value = false
    }
}
</script>

<style scoped>
.project-settings-dialog {
    --settings-path-width: 760px;
}

.vscode-setting-row {
    display: flex;
    align-items: center;
    gap: 8px;
}

.vscode-icon {
    width: 40px;
    height: auto;
    flex-shrink: 0;
    opacity: .9;
}

.path-input-group {
    display: flex;
    gap: 8px;
    align-items: center;
    width: var(--settings-path-width);
    max-width: 100%;
}

.path-input {
    flex: 1;
    min-width: 500px;
}


.mini-text {
    margin-left: 8px;
    font-size: 12px;
    color: var(--el-text-color-secondary);
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}
</style>
