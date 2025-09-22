<template>
    <div class="projects-view">
        <div class="toolbar">
            <el-input v-model="keyword" placeholder="搜索项目 (名称/路径)" clearable size="default" class="search-box"
                @input="applyFilter" />
            <el-button :loading="loading" size="default" @click="loadProjects" :icon="Refresh">刷新</el-button>
            <el-button size="default" @click="openSettings">设置</el-button>
        </div>

        <div class="cards-wrapper" v-loading="loading">
            <div v-for="item in filtered" :key="item.path" class="project-card" @dblclick="openCard(item)"
                @click="selectCard(item)" :class="{ active: selected && selected.path === item.path }"
                title="双击打开，点击选择">
                <div class="card-header">
                    <img src="/code.png" class="card-icon" alt="icon" />
                    <div class="card-title" :title="item.label">{{ item.label }}</div>
                </div>
                <div class="card-body">
                    <div class="path" :title="item.path" @click.stop="openFolder(item.path)">{{ item.path }}</div>
                    <div class="meta">
                        <span class="badge" v-if="item.kind === 'workspace'">工作区</span>
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

const settingsStore = useSettingsStore()
const projects = ref([])
const filtered = ref([])
const keyword = ref('')
const loading = ref(false)
const selected = ref(null)
const settingsVisible = ref(false)

const loadProjects = async () => {
    loading.value = true
    try {
        const storagePath = settingsStore.vscodeStoragePath || null
        const data = await invoke('get_recent_vscode_projects', { storagePath })
        projects.value = Array.isArray(data) ? data : []
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
    if (!k) {
        filtered.value = projects.value
    } else {
        filtered.value = projects.value.filter(p => (p.label || '').toLowerCase().includes(k) || (p.path || '').toLowerCase().includes(k))
    }
}

const selectCard = (item) => { selected.value = item }
const openCard = (item) => { selected.value = item; openProject(item) }
const openProject = async (item) => {
    try {
        await invoke('open_in_vscode', { path: item.path })
    } catch (e) {
        const msg = (e && e.message) ? e.message : String(e)
        ElMessage.error(`打开 VSCode 失败: ${msg}`)
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
        }

        .card-icon {
            width: 24px;
            height: 24px;
            border-radius: 6px;
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
</style>
