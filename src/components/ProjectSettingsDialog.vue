<template>
    <el-dialog v-model="visible" title="编辑器设置" :close-on-click-modal="false" width="fit-content"
        class="project-settings-dialog">
        <div class="editors-grid">
            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getVscodeExeInfo().fullPath" :file-name="getVscodeExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">VSCode</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input :value="displayPath" readonly class="path-input" :title="displayPath">
                            <template #append>
                                <el-button @click="selectVscodeStorage" :icon="FolderOpened" title="选择 storage.json" />
                            </template>
                        </el-input>
                        <el-button v-if="settingsStore.vscodeStoragePath" @click="clearCustomPath">清除</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input :value="settingsStore.vscodeExecutablePath" readonly class="path-input"
                            :title="settingsStore.vscodeExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectVscodeExe" :icon="FolderOpened" title="选择 Code.exe" />
                            </template>
                        </el-input>
                        <el-button v-if="settingsStore.vscodeExecutablePath" @click="clearVscodeExe">清除</el-button>
                        <el-button @click="searchVscodeExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>
            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getTraeExeInfo().fullPath" :file-name="getTraeExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">Trae</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input :value="displayTraePath" readonly class="path-input" :title="displayTraePath">
                            <template #append>
                                <el-button @click="selectTraeStorage" :icon="FolderOpened" title="选择 storage.json" />
                            </template>
                        </el-input>
                        <el-button v-if="settingsStore.traeStoragePath" @click="clearTraeCustomPath">清除</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input :value="settingsStore.traeExecutablePath" readonly class="path-input"
                            :title="settingsStore.traeExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectTraeExe" :icon="FolderOpened" title="选择 Trae.exe" />
                            </template>
                        </el-input>
                        <el-button v-if="settingsStore.traeExecutablePath" @click="clearTraeExe">清除</el-button>
                        <el-button @click="searchTraeExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>
        </div>

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
import { invoke } from '@tauri-apps/api/core'
import { Folder, FolderOpened } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { homeDir } from '@tauri-apps/api/path'
import { ElMessage } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'
import { useFileSearchSettingsStore } from '@/stores/fileSearchSettings'
import FileIcon from '@/components/FileIcon.vue'

const props = defineProps({
    modelValue: { type: Boolean, default: false }
})
const emit = defineEmits(['update:modelValue', 'saved'])
const settingsStore = useSettingsStore()
const fileSearchSettingsStore = useFileSearchSettingsStore()
const saving = ref(false)

const visible = computed({
    get: () => props.modelValue,
    set: v => emit('update:modelValue', v)
})

const defaultVscodeStorage = ref('')
const displayPath = computed(() => settingsStore.vscodeStoragePath || defaultVscodeStorage.value || '加载中...')
const defaultTraeStorage = ref('')
const displayTraePath = computed(() => settingsStore.traeStoragePath || defaultTraeStorage.value || '加载中...')

onMounted(async () => {
    try {
        let home = await homeDir()
        if (!home.endsWith('/') && !home.endsWith('\\')) home = home + '/'
        const norm = (p) => p.replace(/\\/g, '/').replace(/\/+/g, '/');
        defaultVscodeStorage.value = norm(`${home}AppData/Roaming/Code/User/globalStorage/storage.json`)
        defaultTraeStorage.value = norm(`${home}AppData/Roaming/Trae/User/globalStorage/storage.json`)
    } catch {
        defaultVscodeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Code/User/globalStorage/storage.json'
        defaultTraeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Trae/User/globalStorage/storage.json'
    }
})

const persist = () => {
    settingsStore.$patch({ vscodeStoragePath: settingsStore.vscodeStoragePath, traeStoragePath: settingsStore.traeStoragePath })
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

const selectTraeStorage = async () => {
    try {
        const selected = await open({
            title: '选择 Trae storage.json',
            multiple: false,
            directory: false,
            filters: [{ name: 'JSON', extensions: ['json'] }]
        })
        if (selected) {
            settingsStore.traeStoragePath = selected
            persist()
        }
    } catch (e) {
        ElMessage.error('选择文件失败: ' + e)
    }
}

const clearTraeCustomPath = () => {
    settingsStore.traeStoragePath = ''
    persist()
}

const selectVscodeExe = async () => {
    try {
        const selected = await open({ title: '选择 VSCode 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.vscodeExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}
const clearVscodeExe = () => { settingsStore.vscodeExecutablePath = ''; persist(); }

const selectTraeExe = async () => {
    try {
        const selected = await open({ title: '选择 Trae 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.traeExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}
const clearTraeExe = () => { settingsStore.traeExecutablePath = ''; persist(); }

async function doExeSearch(rawTerm, exactLower) {
    const host = fileSearchSettingsStore.everythingSettings.host
    const port = fileSearchSettingsStore.everythingSettings.port
    const params = { search: rawTerm, offset: 0, count: 50, case: false, wholeword: false, path: false, regex: false, host, port, path_column: 1, size_column: 1, date_modified_column: 1 }
    console.debug('[ExeSearch] invoke search_everything params:', params)
    const res = await invoke('search_everything', params)
    const list = (res && res.results) ? res.results : []
    console.debug('[ExeSearch] result count:', list.length)
    if (list.length === 0 && rawTerm.toLowerCase() === rawTerm) {
        // 尝试首字母大写形式再搜一次（Code.exe / Trae.exe）
        const alt = rawTerm.replace(/^[a-z]/, c => c.toUpperCase())
        if (alt !== rawTerm) {
            return await doExeSearch(alt, false)
        }
    }
    if (exactLower) {
        const lower = rawTerm.toLowerCase()
        const exact = list.find(r => (r.name || '').toLowerCase() === lower)
        return exact || list[0]
    }
    return list[0]
}

const searchVscodeExe = async () => {
    try {
        const first = await doExeSearch('code.exe', true)
        if (first && first.path) {
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.vscodeExecutablePath = full.replace(/\\/g, '/')
            persist();
            ElMessage.success('已自动填充 VSCode 可执行路径')
        } else {
            ElMessage.warning('未找到 Code.exe')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}
const searchTraeExe = async () => {
    try {
        const first = await doExeSearch('trae.exe', true)
        if (first && first.path) {
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.traeExecutablePath = full.replace(/\\/g, '/')
            persist();
            ElMessage.success('已自动填充 Trae 可执行路径')
        } else {
            ElMessage.warning('未找到 Trae.exe')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const handleSave = async () => {
    saving.value = true
    try {
        settingsStore.$patch({ vscodeStoragePath: settingsStore.vscodeStoragePath, traeStoragePath: settingsStore.traeStoragePath })
        ElMessage.success('保存成功')
        emit('saved')
        visible.value = false
    } catch (e) {
        ElMessage.error('保存失败: ' + e)
    } finally {
        saving.value = false
    }
}

// 规范化 exe 信息（若为目录则补全 exe；若未设置返回命令名）
const normalizeExe = (raw, exeName) => {
    if (!raw || !raw.trim()) return { fullPath: exeName.toLowerCase(), fileName: exeName }
    let p = raw.trim()
    const hasExe = /\\[^\\]+\.exe$/i.test(p) || /\/[^/]+\.exe$/i.test(p)
    if (!hasExe) {
        if (!/[\\/]$/.test(p)) p += '/'
        p += exeName
    }
    return { fullPath: p.replace(/\\/g, '/'), fileName: exeName }
}
const getVscodeExeInfo = () => normalizeExe(settingsStore.vscodeExecutablePath, 'Code.exe')
const getTraeExeInfo = () => normalizeExe(settingsStore.traeExecutablePath, 'Trae.exe')
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

.trae-setting-row {
    display: flex;
    align-items: center;
    gap: 8px;
}

.exe-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 6px;
}

.exe-label {
    width: 100px;
    text-align: right;
    font-size: 13px;
    color: var(--el-text-color-secondary);
    flex-shrink: 0;
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

.editors-grid {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.editor-card {
    background: var(--el-fill-color-blank);
    border: 1px solid var(--el-border-color);
    border-radius: 10px;
    padding: 14px 16px 12px;
    width: 760px;
    max-width: 100%;
    display: flex;
    flex-direction: column;
    gap: 14px;
}

.editor-header {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 16px;
    font-weight: 600;
}

.editor-icon {
    width: 32px;
    height: 32px;
    border-radius: 8px;
}

.field-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
}

.field-block>label {
    font-size: 12px;
    color: var(--el-text-color-secondary);
}

.field-row {
    display: flex;
    gap: 8px;
    align-items: center;
}
</style>
