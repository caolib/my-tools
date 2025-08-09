<template>
    <div class="env-var-card">
        <div class="card-header">
            <span class="var-name" v-html="renderName(envVar.name)"></span>
            <div class="card-actions">
                <el-button @click="handleEditClick" size="small" :icon="Edit" text round :disabled="disableEdit"
                    v-if="!disableEdit">编辑</el-button>
                <el-tooltip v-else content="请以管理员身份运行" placement="top">
                    <el-button size="small" :icon="Edit" text round disabled>编辑</el-button>
                </el-tooltip>
                <el-popconfirm title="确定要删除该变量吗？" confirm-button-text="确定" cancel-button-text="取消"
                    @confirm="$emit('delete', envVar)">
                    <template #reference>
                        <el-button size="small" :disabled="disableEdit" :icon="Delete" round text
                            type="danger">删除</el-button>
                    </template>
                </el-popconfirm>
            </div>
        </div>
        <div class="var-value">
            <template v-if="isSemicolonSeparatedValue">
                <div v-if="!editingPath" class="path-list-clickable"
                    :style="disableEdit && envVar.name === 'Path' ? 'cursor:not-allowed;opacity:0.5;' : 'cursor:pointer;'"
                    @click="$emit('edit', envVar)">
                    <ul class="path-list">
                        <li v-for="(item, idx) in pathList" :key="idx" class="path-item">
                            <div v-if="isPathClickable(item)" class="clickable-path-item" :title="item">
                                <span class="path-text" @click.stop="handlePathItemClick(item)">
                                    <span v-html="renderPathItem(item)"></span>
                                    <el-icon class="path-external-icon">
                                        <FolderOpened />
                                    </el-icon>
                                </span>
                            </div>
                            <div v-else class="non-clickable-path-item">
                                <span class="path-text" v-html="renderPathItem(item)"></span>
                            </div>
                        </li>
                    </ul>
                </div>
                <div v-else>
                    <div class="path-edit-container">
                        <div style="display: flex; flex-direction: column; gap: 8px; margin-bottom: 12px;">
                            <div style="display: flex; gap: 8px; align-items: center;">
                                <el-button size="small" type="primary" @click="savePath">保存</el-button>
                                <el-button size="small" @click="cancelEditPath">取消</el-button>
                                <el-button size="small" @click="deduplicatePaths" :icon="RefreshRight">路径去重</el-button>
                                <el-button size="small" @click="checkPathsExistence" :icon="Warning">路径检查</el-button>
                            </div>
                            <el-alert v-if="isDirty" title="有未保存的更改，请点击保存" type="warning" show-icon
                                style="margin-bottom: 8px;" />
                        </div>
                        <div v-for="(item, index) in editList" :key="`item-${index}`" class="path-edit-item">
                            <!-- 输入框区域 -->
                            <div class="input-wrapper">
                                <el-input v-model="editList[index]" class="path-input" placeholder="输入路径..."
                                    size="small" />
                            </div>
                            <!-- 操作菜单 -->
                            <el-dropdown @command="handleCommand" trigger="click">
                                <el-button size="small" text class="menu-btn">
                                    <el-icon>
                                        <MoreFilled />
                                    </el-icon>
                                </el-button>
                                <template #dropdown>
                                    <el-dropdown-menu>
                                        <el-dropdown-item :command="`up-${index}`" :disabled="index === 0">
                                            <el-icon>
                                                <ArrowUp />
                                            </el-icon>
                                            上移
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`down-${index}`"
                                            :disabled="index === editList.length - 1">
                                            <el-icon>
                                                <ArrowDown />
                                            </el-icon>
                                            下移
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`move-to-top-${index}`" :disabled="index === 0"
                                            divided>
                                            <el-icon>
                                                <Top />
                                            </el-icon>
                                            移到顶部
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`move-to-bottom-${index}`"
                                            :disabled="index === editList.length - 1">
                                            <el-icon>
                                                <Bottom />
                                            </el-icon>
                                            移到底部
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`insert-above-${index}`" divided>
                                            <el-icon>
                                                <Plus />
                                            </el-icon>
                                            在上面插入
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`insert-below-${index}`">
                                            <el-icon>
                                                <Plus />
                                            </el-icon>
                                            在下面插入
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`delete-${index}`" divided>
                                            <el-icon>
                                                <Delete />
                                            </el-icon>
                                            删除
                                        </el-dropdown-item>
                                    </el-dropdown-menu>
                                </template>
                            </el-dropdown>
                        </div>
                    </div>
                </div>
            </template>
            <template v-else>
                <div v-if="isClickableValue" class="clickable-value">
                    <span class="value-text" @click.stop="handleValueClick" v-html="renderValue(envVar.value)"></span>
                </div>
                <div v-else class="normal-value" v-html="renderValue(envVar.value)"></div>
            </template>
        </div>
    </div>

    <!-- 路径检查对话框 -->
    <PathCheckDialog v-model="showPathCheckDialog" :title="pathCheckDialogData.title"
        :stats-title="pathCheckDialogData.statsTitle" :stats="pathCheckDialogData.stats"
        :paths-title="pathCheckDialogData.pathsTitle" :path-items="pathCheckDialogData.pathItems"
        :confirm-text="pathCheckDialogData.confirmText" @confirm="handleDialogConfirm" @cancel="handleDialogCancel" />
</template>

<script setup>
import { Edit, Delete, MoreFilled, ArrowUp, ArrowDown, Plus, Top, Bottom, Link, FolderOpened, RefreshRight, Warning } from '@element-plus/icons-vue'
import { computed, ref, watch } from 'vue'
import { ElMessage, ElAlert } from 'element-plus'
import { openUrl, openPath } from '@tauri-apps/plugin-opener'
import { invoke } from '@tauri-apps/api/core'
import PathCheckDialog from './PathCheckDialog.vue'

// 高亮工具函数
function escapeHtml(str) {
    if (str === null || str === undefined) return ''
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/'/g, '&#39;')
}

function highlightHtml(text, keyword) {
    if (!text) return ''
    if (!keyword) return escapeHtml(text)
    const src = String(text)
    const k = String(keyword)
    const srcLower = src.toLowerCase()
    const kLower = k.toLowerCase()
    let i = 0
    let from = 0
    let out = ''
    while ((i = srcLower.indexOf(kLower, from)) !== -1) {
        out += escapeHtml(src.slice(from, i))
        const match = src.slice(i, i + k.length)
        out += `<mark class="hl-match">${escapeHtml(match)}</mark>`
        from = i + k.length
    }
    out += escapeHtml(src.slice(from))
    return out
}

function shouldHighlightName() {
    return !!props.highlight && (props.highlightType === 'all' || props.highlightType === 'name')
}
function shouldHighlightValue() {
    return !!props.highlight && (props.highlightType === 'all' || props.highlightType === 'value')
}

function renderName(name) {
    return shouldHighlightName() ? highlightHtml(name, props.highlight) : escapeHtml(name)
}
function renderValue(value) {
    return shouldHighlightValue() ? highlightHtml(value, props.highlight) : escapeHtml(value)
}
function renderPathItem(item) {
    return shouldHighlightValue() ? highlightHtml(item, props.highlight) : escapeHtml(item)
}

const props = defineProps({
    envVar: {
        type: Object,
        required: true
    },
    isAdmin: {
        type: Boolean,
        default: true
    },
    disableEdit: {
        type: Boolean,
        default: false
    },
    highlight: {
        type: String,
        default: ''
    },
    highlightType: {
        type: String,
        default: 'all'
    }
})
const emit = defineEmits(['edit', 'delete'])

const pathList = computed(() => {
    if (isSemicolonSeparatedValue.value) {
        return props.envVar.value.split(';').filter(Boolean)
    }
    return []
})

// 检查是否为分号分隔的值
const isSemicolonSeparatedValue = computed(() => {
    const value = props.envVar.value?.trim()
    if (!value) return false

    // 检查是否包含分号且不只是末尾的一个分号
    return value.includes(';') && value.split(';').filter(Boolean).length > 1
})

// 检查是否为链接
const isLink = (value) => {
    if (!value?.trim()) return false
    return value.match(/^https?:\/\/.+/i)
}

// 检查是否为文件路径
const isPath = (value) => {
    if (!value?.trim()) return false
    // 检查是否为文件路径 (Windows 路径格式)
    return value.match(/^[a-zA-Z]:[\\\/]/) || value.match(/^\\\\/) || value.match(/^\/[^\/]/)
}

// 检查值是否为可点击的路径或链接
const isClickableValue = computed(() => {
    if (isSemicolonSeparatedValue.value) return false // 分号分隔的值有特殊处理

    const value = props.envVar.value?.trim()
    if (!value) return false

    // 检查是否为 HTTP/HTTPS 链接
    if (isLink(value)) {
        return true
    }

    // 检查是否为文件路径
    if (isPath(value)) {
        return true
    }

    return false
})

// Path 编辑相关
const editingPath = ref(false)
const editList = ref([])

// 跟踪编辑内容是否有变动
const isDirty = ref(false)

// 对话框相关
const showPathCheckDialog = ref(false)
const pathCheckDialogData = ref({
    title: '',
    statsTitle: '',
    stats: [],
    pathsTitle: '',
    pathItems: [],
    confirmText: ''
})
const currentDialogAction = ref('')
const currentDialogData = ref(null)

// 处理下拉菜单命令
const handleCommand = (command) => {
    console.log('🎯 收到菜单命令:', command)

    if (command.startsWith('up-')) {
        const index = parseInt(command.replace('up-', ''))
        moveUp(index)
    } else if (command.startsWith('down-')) {
        const index = parseInt(command.replace('down-', ''))
        moveDown(index)
    } else if (command.startsWith('move-to-top-')) {
        const index = parseInt(command.replace('move-to-top-', ''))
        moveToTop(index)
    } else if (command.startsWith('move-to-bottom-')) {
        const index = parseInt(command.replace('move-to-bottom-', ''))
        moveToBottom(index)
    } else if (command.startsWith('insert-above-')) {
        const index = parseInt(command.replace('insert-above-', ''))
        insertAbove(index)
    } else if (command.startsWith('insert-below-')) {
        const index = parseInt(command.replace('insert-below-', ''))
        insertBelow(index)
    } else if (command.startsWith('delete-')) {
        const index = parseInt(command.replace('delete-', ''))
        removePathItem(index)
    }
}

// 上移
const moveUp = (index) => {
    if (index > 0) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.splice(index - 1, 0, item)
        console.log('⬆️ 上移项目:', index)
    }
}

// 下移
const moveDown = (index) => {
    if (index < editList.value.length - 1) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.splice(index + 1, 0, item)
        console.log('⬇️ 下移项目:', index)
    }
}

// 移到顶部
const moveToTop = (index) => {
    if (index > 0) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.unshift(item)
        console.log('🔝 移到顶部:', index)
    }
}

// 移到底部
const moveToBottom = (index) => {
    if (index < editList.value.length - 1) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.push(item)
        console.log('🔽 移到底部:', index)
    }
}

// 在上面插入
const insertAbove = (index) => {
    editList.value.splice(index, 0, '')
    console.log('⬆️➕ 在上面插入:', index)
}

// 在下面插入
const insertBelow = (index) => {
    editList.value.splice(index + 1, 0, '')
    isDirty.value = true
}

// 移除Path项目
const removePathItem = (index) => {
    editList.value.splice(index, 1)
    console.log('🗑️ 删除项目:', index)
    isDirty.value = true
}

// 处理编辑按钮点击
const handleEditClick = () => {
    // 如果是分号分隔的值（如 Path 变量），则启动内置编辑器
    if (isSemicolonSeparatedValue.value) {
        startEditPath()
    } else {
        // 对于普通变量，触发父组件的编辑对话框
        emit('edit', props.envVar)
    }
}

function startEditPath() {
    editList.value = [...pathList.value]
    editingPath.value = true
    isDirty.value = false
}

function cancelEditPath() {
    editingPath.value = false
    editList.value = []
    isDirty.value = false
}

function addPath() {
    editList.value.push('')
    isDirty.value = true
}

async function savePath() {
    const newValue = editList.value.filter(Boolean).join(';')
    emit('edit', { ...props.envVar, value: newValue })
    editingPath.value = false
    isDirty.value = false
}

// 如果外部 Path 变化，自动退出编辑
watch(() => props.envVar.value, () => {
    if (editingPath.value) {
        editingPath.value = false
        isDirty.value = false
    }
})

// 监听 editList 变化，判断是否有未保存更改
watch(editList, (newVal) => {
    if (!editingPath.value) return
    // 只要内容和原始 pathList 不一致就提示
    isDirty.value = newVal.join(';') !== pathList.value.join(';')
}, { deep: true })

// 处理值的点击事件
async function handleValueClick() {
    const value = props.envVar.value?.trim()
    if (!value) return

    try {
        // 检查是否为链接
        if (isLink(value)) {
            await openUrl(value)
            ElMessage.success('已在默认浏览器中打开链接')
            return
        }

        // 检查是否为文件路径
        if (isPath(value)) {
            await openPath(value)
            ElMessage.success('已在文件管理器中打开路径')
            return
        }
    } catch (error) {
        console.error('打开失败:', error)
        ElMessage.error('无法打开该路径或链接')
    }
}

// 检查路径项是否可点击
function isPathClickable(path) {
    return isPath(path)
}

// 处理 Path 项目的点击事件
async function handlePathItemClick(path) {
    if (!path?.trim()) return

    try {
        await openPath(path)
        ElMessage.success(`已在文件管理器中打开: ${path}`)
    } catch (error) {
        console.error('打开路径失败:', error)
        ElMessage.error(`无法打开路径: ${path}`)
    }
}

// 对话框处理函数
const handleDialogConfirm = () => {
    if (currentDialogAction.value === 'deduplicate') {
        performDeduplicate()
    } else if (currentDialogAction.value === 'checkExistence') {
        performPathExistenceCheck()
    }
}

const handleDialogCancel = () => {
    currentDialogAction.value = ''
    currentDialogData.value = null
}

// 执行去重操作
const performDeduplicate = () => {
    const data = currentDialogData.value
    if (!data) return

    let removedCount = 0
    data.duplicatePaths.forEach(item => {
        // 从后往前删除，避免索引变化
        for (let i = item.indices.length - 1; i > 0; i--) {
            const indexToRemove = item.indices[i] - removedCount
            editList.value.splice(indexToRemove, 1)
            removedCount++
        }
    })

    isDirty.value = true
    ElMessage.success(`已删除 ${removedCount} 个重复路径`)
}

// 执行路径存在性检查删除操作
const performPathExistenceCheck = () => {
    const data = currentDialogData.value
    if (!data) return

    // 从后往前删除，避免索引变化
    data.nonExistentIndices.sort((a, b) => b - a).forEach(index => {
        editList.value.splice(index, 1)
    })

    isDirty.value = true
    ElMessage.success(`已删除 ${data.nonExistentPaths.length} 个不存在的路径`)
}

// 路径去重功能
async function deduplicatePaths() {
    if (!editList.value.length) {
        ElMessage.warning('没有路径可以去重')
        return
    }

    // 查找重复项
    const pathCounts = {}
    const duplicatePaths = []

    editList.value.forEach((path, index) => {
        const trimmedPath = path.trim()
        if (!trimmedPath) return

        if (!pathCounts[trimmedPath]) {
            pathCounts[trimmedPath] = []
        }
        pathCounts[trimmedPath].push(index)
    })

    // 找出有重复的路径
    for (const [path, indices] of Object.entries(pathCounts)) {
        if (indices.length > 1) {
            duplicatePaths.push({
                path,
                indices,
                count: indices.length
            })
        }
    }

    if (duplicatePaths.length === 0) {
        ElMessage.success('没有发现重复的路径')
        return
    }

    // 准备对话框数据
    const totalPaths = editList.value.filter(item => item.trim()).length
    const totalDuplicates = duplicatePaths.reduce((sum, item) => sum + (item.count - 1), 0)

    pathCheckDialogData.value = {
        title: '路径去重',
        statsTitle: '路径去重检查结果',
        stats: [
            { label: '总路径', count: totalPaths, type: 'info' },
            { label: '重复', count: totalDuplicates, type: 'warning' }
        ],
        pathsTitle: '发现以下重复路径',
        pathItems: duplicatePaths.map(item => ({
            path: item.path,
            clickable: isPath(item.path),
            tag: { text: `共${item.count}次`, type: 'warning' }
        })),
        confirmText: '确定要删除重复的路径吗？（保留第一个）'
    }

    currentDialogAction.value = 'deduplicate'
    currentDialogData.value = { duplicatePaths }
    showPathCheckDialog.value = true
}

// 路径检查功能
async function checkPathsExistence() {
    if (!editList.value.length) {
        ElMessage.warning('没有路径可以检查')
        return
    }

    // 过滤出可能是路径的项目
    const pathsToCheck = []
    const pathIndices = []

    editList.value.forEach((item, index) => {
        const trimmedItem = item.trim()
        if (trimmedItem && isPath(trimmedItem)) {
            pathsToCheck.push(trimmedItem)
            pathIndices.push(index)
        }
    })

    if (pathsToCheck.length === 0) {
        ElMessage.info('没有发现可检查的路径')
        return
    }

    try {
        // 调用后端检查路径是否存在
        const results = await invoke('check_paths_exist', { paths: pathsToCheck })

        // 找出不存在的路径
        const nonExistentPaths = []
        const nonExistentIndices = []
        const existentPaths = []

        results.forEach((exists, index) => {
            if (!exists) {
                nonExistentPaths.push({
                    path: pathsToCheck[index],
                    index: pathIndices[index]
                })
                nonExistentIndices.push(pathIndices[index])
            } else {
                existentPaths.push(pathsToCheck[index])
            }
        })

        if (nonExistentPaths.length === 0) {
            ElMessage.success('所有路径都存在')
            return
        }

        // 准备对话框数据
        pathCheckDialogData.value = {
            title: '路径检查',
            statsTitle: '路径检查结果',
            stats: [
                { label: '存在', count: existentPaths.length, type: 'success' },
                { label: '不存在', count: nonExistentPaths.length, type: 'danger' }
            ],
            pathsTitle: '以下路径不存在',
            pathItems: nonExistentPaths.map(item => ({
                path: item.path,
                clickable: true,
                icon: Warning,
                iconClass: 'warning'
            })),
            confirmText: '确定要删除这些不存在的路径吗？'
        }

        currentDialogAction.value = 'checkExistence'
        currentDialogData.value = { nonExistentPaths, nonExistentIndices }
        showPathCheckDialog.value = true

    } catch (error) {
        console.error('检查路径失败:', error)
        ElMessage.error('检查路径时发生错误')
    }
}
</script>

<style lang="scss" scoped src="../assets/styles/env-var-card.scss"></style>