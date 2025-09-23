<template>
    <div class="projects-view">
        <div class="toolbar">
            <el-input v-model="keyword" placeholder="搜索项目 (名称/路径)" clearable size="default" class="search-box"
                @input="applyFilter" />

            <div class="filter-group">
                <el-checkbox-group v-model="selectedEditors" @change="applyFilter">
                    <el-checkbox label="vscode">VSCode</el-checkbox>
                    <el-checkbox label="trae">Trae</el-checkbox>
                    <el-checkbox label="qoder">Qoder</el-checkbox>
                    <el-checkbox label="idea">IDEA</el-checkbox>
                </el-checkbox-group>
            </div>

            <el-button :loading="loading" size="default" @click="loadProjects" :icon="Refresh">刷新</el-button>
            <el-button size="default" @click="openSettings">设置</el-button>
        </div>

        <div class="cards-wrapper" v-loading="loading">
            <div v-for="item in filtered" :key="item.path" class="project-card" @click="selectCard(item)"
                :class="{ active: selected && selected.path === item.path }" title="点击选择，点击对应图标用指定编辑器打开">
                <div class="card-header">
                    <div class="icons">
                        <div v-for="src in item.sources.filter(s => selectedEditors.includes(s))" :key="src"
                            class="editor-icon-wrapper"
                            :title="getEditorExeInfo(src).fullPath + '\n点击用 ' + (src === 'trae' ? 'Trae' : src === 'qoder' ? 'Qoder' : src === 'idea' ? 'IntelliJ IDEA' : 'VSCode') + ' 打开'"
                            @click.stop="openWith(item, src)">
                            <FileIcon :file-path="getEditorExeInfo(src).fullPath"
                                :file-name="getEditorExeInfo(src).fileName" file-type="file" :size="24" />
                        </div>
                    </div>
                    <div class="card-title" :title="item.label">{{ item.label }}</div>
                </div>
                <div class="card-body">
                    <div class="path" :title="item.path" @click.stop="openFolder(item.path)">{{ item.path }}</div>
                    <div class="meta">
                        <span class="badge" v-if="item.hasWorkspace">工作区</span>
                    </div>
                </div>
            </div>
            <div v-if="!loading && filtered.length === 0" class="empty-inline">
                <el-empty description="暂无最近项目" />
            </div>
        </div>
        <ProjectSettingsDialog v-model="settingsVisible" @saved="handleSettingsSaved" />
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
// import { openUrl } from '@tauri-apps/plugin-opener' // 不再使用协议方式
import { ElMessage } from 'element-plus'
import { Refresh } from '@element-plus/icons-vue'
import { useSettingsStore } from '@/stores/settings'
import ProjectSettingsDialog from '@/components/ProjectSettingsDialog.vue'
import FileIcon from '@/components/FileIcon.vue'

const settingsStore = useSettingsStore()
// 原始后端返回列表（可能同一路径多条）
const rawProjects = ref([])
// 合并后的项目列表
const projects = ref([])
const filtered = ref([])
const keyword = ref('')
const selectedEditors = ref(['vscode', 'trae', 'qoder', 'idea'])
const loading = ref(false)
const selected = ref(null)
const settingsVisible = ref(false)

const loadProjects = async () => {
    loading.value = true
    try {
        const vscode_storage_path = settingsStore.vscodeStoragePath || null
        const trae_storage_path = settingsStore.traeStoragePath || null
        const qoder_storage_path = settingsStore.qoderStoragePath || null
        const idea_storage_path = settingsStore.ideaStoragePath || null
        const data = await invoke('get_recent_projects', { vscode_storage_path, trae_storage_path, qoder_storage_path, idea_storage_path })
        rawProjects.value = Array.isArray(data) ? data : []

        // 合并：按 path 分组，聚合 sources
        const map = new Map()
        for (const p of rawProjects.value) {
            // 标准化路径：在Windows上转为小写，统一路径分隔符
            const normalizedPath = p.path.toLowerCase().replace(/\//g, '\\')

            if (!map.has(normalizedPath)) {
                map.set(normalizedPath, { path: p.path, label: p.label, sources: [p.source], hasWorkspace: p.kind === 'workspace', kinds: new Set([p.kind]), mtime: p.mtime })
            } else {
                const entry = map.get(normalizedPath)
                if (!entry.sources.includes(p.source)) entry.sources.push(p.source)
                entry.hasWorkspace = entry.hasWorkspace || p.kind === 'workspace'
                entry.kinds.add(p.kind)
                // 取最近 mtime
                if (p.mtime && (!entry.mtime || p.mtime > entry.mtime)) entry.mtime = p.mtime
                // 如果当前 label 更长或不同来源可按需要策略，这里保持原 label
            }
        }

        const mergedProjects = Array.from(map.values()).sort((a, b) => (b.mtime || 0) - (a.mtime || 0) || a.label.localeCompare(b.label))

        // 验证文件夹路径是否存在
        const paths = mergedProjects.map(p => p.path)
        const pathsExist = await invoke('check_paths_exist', { paths })

        // 只保留存在的项目
        projects.value = mergedProjects.filter((_, index) => pathsExist[index])

        const vsCount = rawProjects.value.filter(p => p.source === 'vscode').length
        const traeCount = rawProjects.value.filter(p => p.source === 'trae').length
        const qoderCount = rawProjects.value.filter(p => p.source === 'qoder').length
        const filteredCount = mergedProjects.length - projects.value.length

        console.log('[Projects] Loaded raw total:', rawProjects.value.length, 'VSCode:', vsCount, 'Trae:', traeCount, 'Qoder:', qoderCount, 'Merged:', mergedProjects.length, 'Valid:', projects.value.length, 'Filtered out:', filteredCount)

        applyFilter()
    } catch (e) {
        ElMessage.error('获取最近项目失败')
        console.error(e)
    } finally {
        loading.value = false
    }
}

const applyFilter = () => {
    const k = keyword.value.trim().toLowerCase()
    const selectedEditorsSet = new Set(selectedEditors.value)

    let filteredList = projects.value

    // 先按编辑器筛选
    if (selectedEditors.value.length > 0 && selectedEditors.value.length < 4) {
        filteredList = filteredList.filter(p =>
            p.sources.some(source => selectedEditorsSet.has(source))
        )
    }

    // 再按关键词筛选
    if (k) {
        filteredList = filteredList.filter(p =>
            (p.label || '').toLowerCase().includes(k) ||
            (p.path || '').toLowerCase().includes(k)
        )
    }

    filtered.value = filteredList
}

const selectCard = (item) => { selected.value = item }
const openCard = (item) => { selected.value = item; openProject(item) }
const openWith = async (item, source) => {
    try {
        if (source === 'trae') {
            await invoke('open_in_trae', { path: item.path, exe_path: settingsStore.traeExecutablePath || null })
        } else if (source === 'qoder') {
            await invoke('open_in_qoder', { path: item.path, exe_path: settingsStore.qoderExecutablePath || null })
        } else if (source === 'idea') {
            await invoke('open_in_idea', { path: item.path, exe_path: settingsStore.ideaExecutablePath || null })
        } else {
            await invoke('open_in_vscode', { path: item.path, exe_path: settingsStore.vscodeExecutablePath || null })
        }
    } catch (e) {
        const msg = (e && e.message) ? e.message : String(e)
        ElMessage.error(`打开失败: ${msg}`)
    }
}
const openFolder = async (path) => {
    try {
        await invoke('reveal_in_explorer', { filePath: path })
    } catch (e) {
        ElMessage.error('打开资源管理器失败')
    }
}

const openSettings = () => { settingsVisible.value = true }
const handleSettingsSaved = () => { loadProjects() }

// 返回指定来源编辑器 exe 的完整路径与文件名
const getEditorExeInfo = (source) => {
    if (source === 'vscode') {
        let cfg = settingsStore.vscodeExecutablePath?.trim() || ''
        if (cfg) {
            // 如果配置的是目录则补全 Code.exe
            if (/\\$|\/$/.test(cfg) || /\\Code\.exe$/i.test(cfg) === false && /\/Code\.exe$/i.test(cfg) === false && /\.exe$/i.test(cfg) === false) {
                // 末尾没有 .exe
                if (!/\\$|\/$/.test(cfg)) cfg += '\\'
                cfg += 'Code.exe'
            }
            return { fullPath: cfg, fileName: 'Code.exe' }
        }
        return { fullPath: 'code.exe', fileName: 'Code.exe' }
    } else if (source === 'trae') {
        let cfg = settingsStore.traeExecutablePath?.trim() || ''
        if (cfg) {
            if (/\\$|\/$/.test(cfg) || /\\Trae\.exe$/i.test(cfg) === false && /\/Trae\.exe$/i.test(cfg) === false && /\.exe$/i.test(cfg) === false) {
                if (!/\\$|\/$/.test(cfg)) cfg += '\\'
                cfg += 'Trae.exe'
            }
            return { fullPath: cfg, fileName: 'Trae.exe' }
        }
        return { fullPath: 'trae.exe', fileName: 'Trae.exe' }
    } else if (source === 'qoder') {
        let cfg = settingsStore.qoderExecutablePath?.trim() || ''
        if (cfg) {
            if (/\\$|\/$/.test(cfg) || /\\Qoder\.exe$/i.test(cfg) === false && /\/Qoder\.exe$/i.test(cfg) === false && /\.exe$/i.test(cfg) === false) {
                if (!/\\$|\/$/.test(cfg)) cfg += '\\'
                cfg += 'Qoder.exe'
            }
            return { fullPath: cfg, fileName: 'Qoder.exe' }
        }
        return { fullPath: 'qoder.exe', fileName: 'Qoder.exe' }
    } else if (source === 'idea') {
        let cfg = settingsStore.ideaExecutablePath?.trim() || ''
        if (cfg) {
            if (/\\$|\/$/.test(cfg) || /\\idea64\.exe$/i.test(cfg) === false && /\/idea64\.exe$/i.test(cfg) === false && /\.exe$/i.test(cfg) === false) {
                if (!/\\$|\/$/.test(cfg)) cfg += '\\'
                cfg += 'idea64.exe'
            }
            return { fullPath: cfg, fileName: 'idea64.exe' }
        }
        return { fullPath: 'idea64.exe', fileName: 'idea64.exe' }
    }
    return { fullPath: 'unknown.exe', fileName: 'Unknown.exe' }
}

onMounted(loadProjects)
</script>

<style lang="scss" scoped>
.projects-view {
    padding: 56px 20px 20px; // 顶部留出标题栏空间
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    background: var(--el-bg-color);
    gap: 12px;

    .toolbar {
        display: flex;
        gap: 12px;
        align-items: center;

        .search-box {
            width: 320px;
        }

        .filter-group {
            display: flex;
            align-items: center;
            gap: 8px;
            flex: 1;

            .filter-label {
                font-size: 14px;
                color: var(--el-text-color-regular);
                white-space: nowrap;
            }

            .el-checkbox-group {
                display: flex;
                gap: 12px;
            }
        }

        .toolbar-actions {
            display: flex;
            gap: 8px;
        }
    }

    .cards-wrapper {
        flex: 1;
        display: flex;
        flex-wrap: wrap;
        align-content: flex-start;
        gap: 12px;
        overflow-y: auto;
        padding-bottom: 8px;
    }

    .project-card {
        background: var(--el-fill-color-blank);
        border: 1px solid var(--el-border-color);
        border-radius: 10px;
        padding: 12px 12px 10px;
        display: flex;
        flex-direction: column;
        gap: 6px;
        cursor: default;
        transition: border-color .15s, background .15s, box-shadow .15s;
        position: relative;
        height: 70px;
        /* 固定高度 */
        flex: 0 1 auto;
        /* 允许根据内容自然宽度 */
        min-width: 160px;
        /* 下限，避免过窄 */
        max-width: 520px;
        /* 上限，避免拉太宽 */
        width: fit-content;
        /* 根据内容自适应 */
        overflow: hidden;

        &:hover {
            border-color: var(--el-color-primary);
            box-shadow: 0 2px 6px rgba(0, 0, 0, .08);
        }

        &.active {
            border-color: var(--el-color-primary);
            background: var(--el-color-primary-light-9);
        }

        .card-header {
            display: flex;
            align-items: center;
            gap: 8px;
            position: relative;
        }

        .card-icon {
            width: 24px;
            height: 24px;
            border-radius: 6px;
        }

        .icons {
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .editor-icon-wrapper {
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            width: 24px;
            height: 24px;
            cursor: pointer;
        }

        .editor-icon-wrapper:hover {
            filter: brightness(1.1);
        }

        .src-badge {
            position: absolute;
            bottom: -4px;
            right: -4px;
            background: var(--el-color-primary);
            color: #fff;
            border-radius: 6px;
            padding: 0 3px;
            font-size: 10px;
            line-height: 12px;
            font-weight: 600;
            box-shadow: 0 0 0 1px var(--el-bg-color);
        }

        .src-badge.trae {
            background: #e67e22;
        }

        .src-badge.vscode {
            background: var(--el-color-primary);
        }

        .card-icon.multi {
            cursor: pointer;
            opacity: .9;
            transition: transform .15s, opacity .15s;
        }

        .card-icon.multi:hover {
            transform: scale(1.05);
            opacity: 1;
        }


        .card-body {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .path {
            font-size: 11px;
            color: var(--el-text-color-secondary);
            line-height: 1.3;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            /* 单行 */
            cursor: pointer;

            &:hover {
                color: var(--el-color-primary);
                text-decoration: underline;
            }
        }

        .meta {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 11px;
            color: var(--el-text-color-secondary);
        }

        .badge {
            background: var(--el-color-primary-light-8);
            color: var(--el-color-primary);
            padding: 2px 6px;
            border-radius: 6px;
            font-size: 11px;
        }

        .empty-inline {
            width: 100%;
        }
    }
}

.el-checkbox {
    margin-right: 0;
}
</style>
