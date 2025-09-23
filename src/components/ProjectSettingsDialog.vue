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
                        <el-input v-model="settingsStore.vscodeStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="displayPath">
                            <template #append>
                                <el-button @click="selectVscodeStorage" :icon="FolderOpened" title="选择 storage.json" />
                            </template>
                        </el-input>
                        <el-button @click="searchVscodeStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.vscodeExecutablePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="settingsStore.vscodeExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectVscodeExe" :icon="FolderOpened" title="选择 Code.exe" />
                            </template>
                        </el-input>
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
                        <el-input v-model="settingsStore.traeStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="displayTraePath">
                            <template #append>
                                <el-button @click="selectTraeStorage" :icon="FolderOpened" title="选择 storage.json" />
                            </template>
                        </el-input>
                        <el-button @click="searchTraeStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.traeExecutablePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="settingsStore.traeExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectTraeExe" :icon="FolderOpened" title="选择 Trae.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchTraeExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>
            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getQoderExeInfo().fullPath" :file-name="getQoderExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">Qoder</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.qoderStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="displayQoderPath">
                            <template #append>
                                <el-button @click="selectQoderStorage" :icon="FolderOpened" title="选择 storage.json" />
                            </template>
                        </el-input>
                        <el-button @click="searchQoderStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.qoderExecutablePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="settingsStore.qoderExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectQoderExe" :icon="FolderOpened" title="选择 Qoder.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchQoderExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>
            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getIdeaExeInfo().fullPath" :file-name="getIdeaExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">IntelliJ IDEA</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.ideaStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="displayIdeaPath">
                            <template #append>
                                <el-button @click="selectIdeaStorage" :icon="FolderOpened"
                                    title="选择 recentProjects.xml" />
                            </template>
                        </el-input>
                        <el-button @click="searchIdeaStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.ideaExecutablePath" size="small" clearable class="path-input"
                            placeholder="留空自动推断" :title="settingsStore.ideaExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectIdeaExe" :icon="FolderOpened" title="选择 idea64.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchIdeaExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>
        </div>

        <template #footer>
            <span class="dialog-footer">
                <el-button @click="searchAll" :loading="searchingAll" type="success">一键搜索</el-button>
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
const searchingAll = ref(false)

const visible = computed({
    get: () => props.modelValue,
    set: v => emit('update:modelValue', v)
})

const defaultVscodeStorage = ref('')
const displayPath = computed(() => settingsStore.vscodeStoragePath || defaultVscodeStorage.value || '加载中...')
const defaultTraeStorage = ref('')
const displayTraePath = computed(() => settingsStore.traeStoragePath || defaultTraeStorage.value || '加载中...')
const defaultQoderStorage = ref('')
const displayQoderPath = computed(() => settingsStore.qoderStoragePath || defaultQoderStorage.value || '加载中...')
const defaultIdeaStorage = ref('')
const displayIdeaPath = computed(() => settingsStore.ideaStoragePath || defaultIdeaStorage.value || '加载中...')

onMounted(async () => {
    try {
        let home = await homeDir()
        if (!home.endsWith('/') && !home.endsWith('\\')) home = home + '/'
        const norm = (p) => p.replace(/\\/g, '/').replace(/\/+/g, '/');
        defaultVscodeStorage.value = norm(`${home}AppData/Roaming/Code/User/globalStorage/storage.json`)
        defaultTraeStorage.value = norm(`${home}AppData/Roaming/Trae/User/globalStorage/storage.json`)
        defaultQoderStorage.value = norm(`${home}AppData/Roaming/Qoder/User/globalStorage/storage.json`)
        defaultIdeaStorage.value = '点击搜索按钮自动查找'
    } catch {
        defaultVscodeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Code/User/globalStorage/storage.json'
        defaultTraeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Trae/User/globalStorage/storage.json'
        defaultQoderStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Qoder/User/globalStorage/storage.json'
        defaultIdeaStorage.value = '点击搜索按钮自动查找'
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

const selectQoderStorage = async () => {
    try {
        const selected = await open({
            title: '选择 Qoder storage.json',
            multiple: false,
            directory: false,
            filters: [{ name: 'JSON', extensions: ['json'] }]
        })
        if (selected) {
            settingsStore.qoderStoragePath = selected
            persist()
        }
    } catch (e) {
        ElMessage.error('选择文件失败: ' + e)
    }
}

const selectIdeaStorage = async () => {
    try {
        const selected = await open({
            title: '选择 IDEA recentProjects.xml',
            multiple: false,
            directory: false,
            filters: [{ name: 'XML', extensions: ['xml'] }]
        })
        if (selected) {
            settingsStore.ideaStoragePath = selected
            persist()
        }
    } catch (e) {
        ElMessage.error('选择文件失败: ' + e)
    }
}

const selectVscodeExe = async () => {
    try {
        const selected = await open({ title: '选择 VSCode 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.vscodeExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

const selectTraeExe = async () => {
    try {
        const selected = await open({ title: '选择 Trae 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.traeExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

const selectQoderExe = async () => {
    try {
        const selected = await open({ title: '选择 Qoder 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.qoderExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

const selectIdeaExe = async () => {
    try {
        const selected = await open({ title: '选择 IDEA 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.ideaExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

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

const searchQoderExe = async () => {
    try {
        const first = await doExeSearch('qoder.exe', true)
        if (first && first.path) {
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.qoderExecutablePath = full.replace(/\\/g, '/')
            persist();
            ElMessage.success('已自动填充 Qoder 可执行路径')
        } else {
            ElMessage.warning('未找到 Qoder.exe')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const searchIdeaExe = async () => {
    try {
        const first = await doExeSearch('idea64.exe', true)
        if (first && first.path) {
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.ideaExecutablePath = full.replace(/\\/g, '/')
            persist();
            ElMessage.success('已自动填充 IDEA 可执行路径')
        } else {
            ElMessage.warning('未找到 idea64.exe')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const handleSave = async () => {
    saving.value = true
    try {
        settingsStore.$patch({
            vscodeStoragePath: settingsStore.vscodeStoragePath,
            traeStoragePath: settingsStore.traeStoragePath,
            qoderStoragePath: settingsStore.qoderStoragePath
        })
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

// 配置文件搜索函数
const searchVscodeStorage = async () => {
    try {
        const results = await doStorageSearch('storage.json path:code')
        if (results && results.length > 0) {
            const first = results[0]
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.vscodeStoragePath = full.replace(/\\/g, '/')
            persist()
            ElMessage.success('已自动填充 VSCode 配置文件路径')
        } else {
            ElMessage.warning('未找到 VSCode storage.json 文件')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const searchTraeStorage = async () => {
    try {
        const results = await doStorageSearch('storage.json path:trae')
        if (results && results.length > 0) {
            const first = results[0]
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.traeStoragePath = full.replace(/\\/g, '/')
            persist()
            ElMessage.success('已自动填充 Trae 配置文件路径')
        } else {
            ElMessage.warning('未找到 Trae storage.json 文件')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const searchQoderStorage = async () => {
    try {
        const results = await doStorageSearch('storage.json path:qoder')
        if (results && results.length > 0) {
            const first = results[0]
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.qoderStoragePath = full.replace(/\\/g, '/')
            persist()
            ElMessage.success('已自动填充 Qoder 配置文件路径')
        } else {
            ElMessage.warning('未找到 Qoder storage.json 文件')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const searchIdeaStorage = async () => {
    try {
        const results = await doStorageSearch('recentProjects.xml path:idea')
        if (results && results.length > 0) {
            const first = results[0]
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore.ideaStoragePath = full.replace(/\\/g, '/')
            persist()
            ElMessage.success('已自动填充 IDEA 配置文件路径')
        } else {
            ElMessage.warning('未找到 IDEA recentProjects.xml 文件')
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

const searchAll = async () => {
    searchingAll.value = true
    try {
        // 同时执行所有搜索
        await Promise.allSettled([
            searchVscodeStorage(),
            searchTraeStorage(),
            searchQoderStorage(),
            searchIdeaStorage(),
            searchVscodeExe(),
            searchTraeExe(),
            searchQoderExe(),
            searchIdeaExe()
        ])
    } catch (e) {
        ElMessage.error('一键搜索失败: ' + (e?.message || e))
    } finally {
        searchingAll.value = false
    }
}

async function doStorageSearch(searchTerm) {
    const host = fileSearchSettingsStore.everythingSettings.host
    const port = fileSearchSettingsStore.everythingSettings.port
    const params = { search: searchTerm, offset: 0, count: 50, case: false, wholeword: false, path: false, regex: false, host, port, path_column: 1, size_column: 1, date_modified_column: 1 }
    console.debug('[StorageSearch] invoke search_everything params:', params)
    const res = await invoke('search_everything', params)
    const list = (res && res.results) ? res.results : []
    console.debug('[StorageSearch] result count:', list.length)
    return list
}

const getVscodeExeInfo = () => normalizeExe(settingsStore.vscodeExecutablePath, 'Code.exe')
const getTraeExeInfo = () => normalizeExe(settingsStore.traeExecutablePath, 'Trae.exe')
const getQoderExeInfo = () => normalizeExe(settingsStore.qoderExecutablePath, 'Qoder.exe')
const getIdeaExeInfo = () => normalizeExe(settingsStore.ideaExecutablePath, 'idea64.exe')
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
