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
                            placeholder="留空自动搜索" :title="displayPath">
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
                            placeholder="留空自动搜索" :title="settingsStore.vscodeExecutablePath || '未设置'">
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
                            placeholder="留空自动搜索" :title="displayTraePath">
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
                            placeholder="留空自动搜索" :title="settingsStore.traeExecutablePath || '未设置'">
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
                            placeholder="留空自动搜索" :title="displayQoderPath">
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
                            placeholder="留空自动搜索" :title="settingsStore.qoderExecutablePath || '未设置'">
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
                            placeholder="留空自动搜索" :title="displayIdeaPath">
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
                            placeholder="留空自动搜索" :title="settingsStore.ideaExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectIdeaExe" :icon="FolderOpened" title="选择 idea64.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchIdeaExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>

            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getWebstormExeInfo().fullPath" :file-name="getWebstormExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">WebStorm</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.webstormStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动搜索" :title="displayWebstormPath">
                            <template #append>
                                <el-button @click="selectWebstormStorage" :icon="FolderOpened"
                                    title="选择 recentProjects.xml" />
                            </template>
                        </el-input>
                        <el-button @click="searchWebstormStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.webstormExecutablePath" size="small" clearable
                            class="path-input" placeholder="留空自动搜索"
                            :title="settingsStore.webstormExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectWebstormExe" :icon="FolderOpened" title="选择 webstorm64.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchWebstormExe" title="自动搜索">搜索</el-button>
                    </div>
                </div>
            </div>

            <div class="editor-card">
                <div class="editor-header">
                    <FileIcon :file-path="getPycharmExeInfo().fullPath" :file-name="getPycharmExeInfo().fileName"
                        file-type="file" :size="32" />
                    <span class="editor-title">PyCharm</span>
                </div>
                <div class="field-block">
                    <label>配置文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.pycharmStoragePath" size="small" clearable class="path-input"
                            placeholder="留空自动搜索" :title="displayPycharmPath">
                            <template #append>
                                <el-button @click="selectPycharmStorage" :icon="FolderOpened"
                                    title="选择 recentProjects.xml" />
                            </template>
                        </el-input>
                        <el-button @click="searchPycharmStorage" title="自动搜索">搜索</el-button>
                    </div>
                </div>
                <div class="field-block">
                    <label>可执行文件</label>
                    <div class="field-row">
                        <el-input v-model="settingsStore.pycharmExecutablePath" size="small" clearable
                            class="path-input" placeholder="留空自动搜索"
                            :title="settingsStore.pycharmExecutablePath || '未设置'">
                            <template #append>
                                <el-button @click="selectPycharmExe" :icon="FolderOpened" title="选择 pycharm64.exe" />
                            </template>
                        </el-input>
                        <el-button @click="searchPycharmExe" title="自动搜索">搜索</el-button>
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
const defaultWebstormStorage = ref('')
const displayWebstormPath = computed(() => settingsStore.webstormStoragePath || defaultWebstormStorage.value || '加载中...')
const defaultPycharmStorage = ref('')
const displayPycharmPath = computed(() => settingsStore.pycharmStoragePath || defaultPycharmStorage.value || '加载中...')

onMounted(async () => {
    try {
        let home = await homeDir()
        if (!home.endsWith('/') && !home.endsWith('\\')) home = home + '/'
        const norm = (p) => p.replace(/\\/g, '/').replace(/\/+/g, '/');
        defaultVscodeStorage.value = norm(`${home}AppData/Roaming/Code/User/globalStorage/storage.json`)
        defaultTraeStorage.value = norm(`${home}AppData/Roaming/Trae/User/globalStorage/storage.json`)
        defaultQoderStorage.value = norm(`${home}AppData/Roaming/Qoder/User/globalStorage/storage.json`)
        defaultIdeaStorage.value = '点击搜索按钮自动查找'
        defaultWebstormStorage.value = '点击搜索按钮自动查找'
    } catch {
        defaultVscodeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Code/User/globalStorage/storage.json'
        defaultTraeStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Trae/User/globalStorage/storage.json'
        defaultQoderStorage.value = 'C:/Users/<当前用户>/AppData/Roaming/Qoder/User/globalStorage/storage.json'
        defaultIdeaStorage.value = '点击搜索按钮自动查找'
        defaultWebstormStorage.value = '点击搜索按钮自动查找'
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

const selectWebstormStorage = async () => {
    try {
        const selected = await open({
            title: '选择 WebStorm recentProjects.xml',
            multiple: false,
            directory: false,
            filters: [{ name: 'XML', extensions: ['xml'] }]
        })
        if (selected) {
            settingsStore.webstormStoragePath = selected
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

const selectWebstormExe = async () => {
    try {
        const selected = await open({ title: '选择 WebStorm 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.webstormExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

const selectPycharmStorage = async () => {
    try {
        const selected = await open({
            title: '选择 PyCharm recentProjects.xml',
            multiple: false,
            directory: false,
            filters: [{ name: 'XML', extensions: ['xml'] }]
        })
        if (selected) {
            settingsStore.pycharmStoragePath = selected
            persist()
        }
    } catch (e) {
        ElMessage.error('选择文件失败: ' + e)
    }
}

const selectPycharmExe = async () => {
    try {
        const selected = await open({ title: '选择 PyCharm 可执行文件', multiple: false, directory: false })
        if (selected) { settingsStore.pycharmExecutablePath = selected; persist(); }
    } catch (e) { ElMessage.error('选择文件失败: ' + e) }
}

// 通用的配置文件搜索函数
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

// 通用的可执行文件搜索函数（添加 ext:exe 过滤）
async function doExeSearch(searchTerm) {
    const host = fileSearchSettingsStore.everythingSettings.host
    const port = fileSearchSettingsStore.everythingSettings.port
    const params = { search: searchTerm, offset: 0, count: 50, case: false, wholeword: false, path: false, regex: false, host, port, path_column: 1, size_column: 1, date_modified_column: 1 }
    console.debug('[ExeSearch] invoke search_everything params:', params)
    const res = await invoke('search_everything', params)
    const list = (res && res.results) ? res.results : []
    console.debug('[ExeSearch] result count:', list.length)
    return list.length > 0 ? list[0] : null
}

// 创建配置文件搜索函数的工厂函数
// 参数: searchTerm, pathKey, notFoundMsg
const createStorageSearcher = (searchTerm, pathKey, notFoundMsg) => async () => {
    try {
        const results = await doStorageSearch(searchTerm)
        if (results && results.length > 0) {
            const first = results[0]
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore[pathKey] = full.replace(/\\/g, '/')
            persist()
        } else {
            ElMessage.warning(notFoundMsg)
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

// 创建可执行文件搜索函数的工厂函数
// 参数: searchTerm, pathKey, notFoundMsg
const createExeSearcher = (searchTerm, pathKey, notFoundMsg) => async () => {
    try {
        const first = await doExeSearch(searchTerm)
        if (first && first.path) {
            const full = (first.path.endsWith('\\') || first.path.endsWith('/')) ? first.path + first.name : first.path + '/' + first.name
            settingsStore[pathKey] = full.replace(/\\/g, '/')
            persist()
        } else {
            ElMessage.warning(notFoundMsg)
        }
    } catch (e) {
        ElMessage.error('搜索失败: ' + (e?.message || e))
    }
}

// 配置文件搜索函数（使用工厂函数创建）
const searchVscodeStorage = createStorageSearcher('storage.json path:code', 'vscodeStoragePath', '未找到 VSCode storage.json 文件')
const searchTraeStorage = createStorageSearcher('storage.json path:trae', 'traeStoragePath', '未找到 Trae storage.json 文件')
const searchQoderStorage = createStorageSearcher('storage.json path:qoder', 'qoderStoragePath', '未找到 Qoder storage.json 文件')
const searchIdeaStorage = createStorageSearcher('recentProjects.xml path:idea', 'ideaStoragePath', '未找到 IDEA recentProjects.xml 文件')
const searchWebstormStorage = createStorageSearcher('recentProjects.xml path:webstorm', 'webstormStoragePath', '未找到 WebStorm recentProjects.xml 文件')
const searchPycharmStorage = createStorageSearcher('recentProjects.xml path:pycharm', 'pycharmStoragePath', '未找到 PyCharm recentProjects.xml 文件')

// 可执行文件搜索函数（使用工厂函数创建，添加 ext:exe 过滤）
const searchVscodeExe = createExeSearcher('code ext:exe', 'vscodeExecutablePath', '未找到 Code.exe')
const searchTraeExe = createExeSearcher('trae ext:exe', 'traeExecutablePath', '未找到 Trae.exe')
const searchQoderExe = createExeSearcher('qoder ext:exe', 'qoderExecutablePath', '未找到 Qoder.exe')
const searchIdeaExe = createExeSearcher('idea64 ext:exe', 'ideaExecutablePath', '未找到 idea64.exe')
const searchWebstormExe = createExeSearcher('webstorm64 ext:exe', 'webstormExecutablePath', '未找到 webstorm64.exe')
const searchPycharmExe = createExeSearcher('pycharm64 ext:exe', 'pycharmExecutablePath', '未找到 pycharm64.exe')

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



const searchAll = async () => {
    searchingAll.value = true
    try {
        // 同时执行所有搜索
        await Promise.allSettled([
            searchVscodeStorage(),
            searchTraeStorage(),
            searchQoderStorage(),
            searchIdeaStorage(),
            searchWebstormStorage(),
            searchPycharmStorage(),
            searchVscodeExe(),
            searchTraeExe(),
            searchQoderExe(),
            searchIdeaExe(),
            searchWebstormExe(),
            searchPycharmExe()
        ])
    } catch (e) {
        ElMessage.error('一键搜索失败: ' + (e?.message || e))
    } finally {
        searchingAll.value = false
    }
}

const getVscodeExeInfo = () => normalizeExe(settingsStore.vscodeExecutablePath, 'Code.exe')
const getTraeExeInfo = () => normalizeExe(settingsStore.traeExecutablePath, 'Trae.exe')
const getQoderExeInfo = () => normalizeExe(settingsStore.qoderExecutablePath, 'Qoder.exe')
const getIdeaExeInfo = () => normalizeExe(settingsStore.ideaExecutablePath, 'idea64.exe')
const getWebstormExeInfo = () => normalizeExe(settingsStore.webstormExecutablePath, 'webstorm64.exe')
const getPycharmExeInfo = () => normalizeExe(settingsStore.pycharmExecutablePath, 'pycharm64.exe')
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
