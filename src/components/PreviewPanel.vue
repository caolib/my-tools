<template>
    <div v-if="visible" class="preview-panel"
        :class="{ 'image-mode': fileType === 'image', 'document-mode': fileType === 'text' }" :style="panelStyle"
        ref="panelRef">
        <!-- 预览面板头部 -->
        <div class="preview-header">
            <div class="preview-title">
                <el-icon>
                    <View />
                </el-icon>
                <span>{{ fileName || '预览' }}</span>
            </div>
            <div class="preview-actions">
                <!-- 自动换行按钮，仅在文本类型时显示 -->
                <el-button v-if="fileType === 'text'" :type="wordWrapEnabled ? 'primary' : 'info'" link size="small"
                    @click="toggleWordWrap" :title="wordWrapEnabled ? '关闭自动换行' : '开启自动换行'">
                    <el-icon>
                        <component :is="wordWrapEnabled ? 'Document' : 'Menu'" />
                    </el-icon>
                </el-button>

                <el-button type="primary" link size="small" @click="openInSystem" title="在系统默认应用中打开">
                    <el-icon>
                        <FolderOpened />
                    </el-icon>
                </el-button>
                <el-button type="danger" link size="small" @click="closePreview" title="关闭预览">
                    <el-icon>
                        <Close />
                    </el-icon>
                </el-button>
            </div>
        </div>

        <!-- 预览内容 -->
        <div class="preview-content" v-loading="loading">
            <!-- 图片预览 -->
            <div v-if="fileType === 'image'" class="image-preview">
                <img v-if="imageDataUrl" :src="imageDataUrl" @load="loading = false" @error="handleError"
                    style="max-width: 100%; max-height: 100%; object-fit: contain;" />
            </div>

            <!-- 文本预览 -->
            <div v-else-if="fileType === 'text'" class="text-preview">
                <div class="code-container" :style="codeContainerStyle">
                    <pre v-if="!codeLanguage" :style="preStyle"><code v-html="highlightedContent"></code></pre>
                    <pre v-else
                        :style="preStyle"><code :class="`language-${codeLanguage}`" v-html="highlightedContent"></code></pre>
                </div>
            </div>

            <!-- 不支持的文件类型 -->
            <div v-else class="unsupported-preview">
                <div class="preview-placeholder">
                    <el-icon :size="48">
                        <Warning />
                    </el-icon>
                    <p>不支持预览此类型的文件</p>
                    <el-button type="primary" size="small" @click="openInSystem">打开文件</el-button>
                </div>
            </div>
        </div>

        <!-- 调整大小的拖拽柄 -->
        <div class="resize-handles">
            <!-- 左边调整宽度 -->
            <div class="resize-handle resize-left" @mousedown="startResize('left')"></div>
            <!-- 上边调整高度 -->
            <div class="resize-handle resize-top" @mousedown="startResize('top')"></div>
            <!-- 左上角同时调整宽高 -->
            <div class="resize-handle resize-corner" @mousedown="startResize('corner')"></div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'
import { View, FolderOpened, Close, Warning, Document, Menu } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { openPath } from '@tauri-apps/plugin-opener'
import { useSettingsStore } from '@/stores/settings'
import { useFileSearchSettingsStore } from '@/stores/fileSearchSettings'
import hljs from 'highlight.js'

const props = defineProps({
    visible: {
        type: Boolean,
        default: false
    },
    filePath: {
        type: String,
        default: ''
    },
    fileName: {
        type: String,
        default: ''
    }
})

const emit = defineEmits(['close'])

// Settings stores
const settingsStore = useSettingsStore()
const fileSearchSettingsStore = useFileSearchSettingsStore()

// 当前加载的CSS主题
const currentLoadedTheme = ref('')

// 可用的代码高亮主题
const availableThemes = {
    'auto': '跟随应用主题',
    'vs2015': 'VS2015 (深色)',
    'github': 'GitHub (浅色)',
    'atom-one-dark': 'Atom One Dark',
    'atom-one-light': 'Atom One Light',
    'monokai': 'Monokai',
    'tomorrow-night': 'Tomorrow Night',
    'dracula': 'Dracula'
}

// 动态加载CSS主题
const loadThemeCSS = (themeName) => {
    if (currentLoadedTheme.value === themeName) return

    // 移除之前的主题样式
    const existingLink = document.querySelector('link[data-hljs-theme]')
    if (existingLink) {
        existingLink.remove()
    }

    // 加载新主题样式
    const link = document.createElement('link')
    link.rel = 'stylesheet'
    link.href = `https://cdn.jsdelivr.net/npm/highlight.js@11.9.0/styles/${themeName}.css`
    link.setAttribute('data-hljs-theme', themeName)
    document.head.appendChild(link)

    currentLoadedTheme.value = themeName
}

// 自动换行设置
const wordWrapEnabled = ref(settingsStore.previewSettings.wordWrap)

// 切换自动换行
const toggleWordWrap = () => {
    wordWrapEnabled.value = !wordWrapEnabled.value
    settingsStore.setPreviewWordWrap(wordWrapEnabled.value)
    console.log('自动换行已', wordWrapEnabled.value ? '开启' : '关闭')
}

const loading = ref(false)
const fileContent = ref('')
const fileType = ref('')
const codeLanguage = ref('')
const imageDataUrl = ref('')
const panelRef = ref(null)

// 面板尺寸和位置 - 从store加载初始值
const panelWidth = ref(settingsStore.previewSettings.panelWidth)
const panelHeight = ref(settingsStore.previewSettings.panelHeight)
const panelRight = ref(0) // 距离右边的距离
const panelBottom = ref(0) // 距离底部的距离

// 调整大小相关
const isResizing = ref(false)
const resizeType = ref('')
const startX = ref(0)
const startY = ref(0)
const startWidth = ref(0)
const startHeight = ref(0)

// 支持的文件格式
const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg', '.ico']
const textExtensions = [
    '.txt', '.md', '.js', '.ts', '.vue', '.html', '.css', '.scss', '.sass', '.less',
    '.json', '.xml', '.yml', '.yaml', '.ini', '.conf', '.log', '.sql', '.py', '.java',
    '.c', '.cpp', '.h', '.hpp', '.cs', '.php', '.rb', '.go', '.rs', '.sh', '.bat',
    '.ps1', '.dockerfile', '.gitignore', '.env'
]

// 计算面板样式
const panelStyle = computed(() => {
    const baseStyle = {
        width: `${panelWidth.value}px`,
        height: `${panelHeight.value}px`,
        right: `${panelRight.value}px`,
        bottom: `${panelBottom.value}px`,
    }

    return baseStyle
})

// 当前应该使用的代码主题
const currentCodeTheme = computed(() => {
    return settingsStore.getCurrentCodeTheme()
})

// 代码字体设置 - 跟随表格字体设置
const codeFont = computed(() => {
    const tableFont = fileSearchSettingsStore.appearanceSettings.tableFontFamily
    const tableFontSize = fileSearchSettingsStore.appearanceSettings.tableFontSize

    return {
        fontFamily: tableFont === 'inherit' ? 'Consolas, Monaco, "Courier New", monospace' : tableFont,
        fontSize: tableFontSize || '14px'
    }
})

// 代码容器样式
const codeContainerStyle = computed(() => {
    return {
        fontFamily: codeFont.value.fontFamily,
        fontSize: codeFont.value.fontSize,
        backgroundColor: settingsStore.theme === 'dark' ? '#1e1e1e' : '#ffffff',
        color: settingsStore.theme === 'dark' ? '#d4d4d4' : '#333333'
    }
})

// pre 元素样式（控制换行）
const preStyle = computed(() => {
    return {
        whiteSpace: wordWrapEnabled.value ? 'pre-wrap' : 'pre',
        wordWrap: wordWrapEnabled.value ? 'break-word' : 'normal',
        overflowWrap: wordWrapEnabled.value ? 'break-word' : 'normal'
    }
})

// 高亮后的内容
const highlightedContent = computed(() => {
    if (!fileContent.value) return ''

    if (codeLanguage.value) {
        try {
            return hljs.highlight(fileContent.value, { language: codeLanguage.value }).value
        } catch (error) {
            console.warn('代码高亮失败，使用自动检测:', error)
            return hljs.highlightAuto(fileContent.value).value
        }
    } else {
        return hljs.highlightAuto(fileContent.value).value
    }
})

// 根据文件扩展名确定文件类型
const detectFileType = (fileName) => {
    if (!fileName) return 'unknown'

    const ext = fileName.toLowerCase().substring(fileName.lastIndexOf('.'))

    if (imageExtensions.includes(ext)) {
        return 'image'
    } else if (textExtensions.includes(ext)) {
        return 'text'
    }

    return 'unknown'
}

// 根据文件扩展名获取代码语言
const getCodeLanguage = (fileName) => {
    if (!fileName) return ''

    const ext = fileName.toLowerCase().substring(fileName.lastIndexOf('.'))
    const languageMap = {
        '.js': 'javascript',
        '.ts': 'typescript',
        '.vue': 'html',
        '.html': 'html',
        '.css': 'css',
        '.scss': 'scss',
        '.sass': 'sass',
        '.less': 'less',
        '.json': 'json',
        '.xml': 'xml',
        '.yml': 'yaml',
        '.yaml': 'yaml',
        '.py': 'python',
        '.java': 'java',
        '.c': 'c',
        '.cpp': 'cpp',
        '.h': 'c',
        '.hpp': 'cpp',
        '.cs': 'csharp',
        '.php': 'php',
        '.rb': 'ruby',
        '.go': 'go',
        '.rs': 'rust',
        '.sh': 'bash',
        '.bat': 'batch',
        '.ps1': 'powershell',
        '.sql': 'sql',
        '.md': 'markdown'
    }

    return languageMap[ext] || ''
}

// 读取图片文件为 base64
const readImageAsBase64 = async (filePath) => {
    try {
        const base64Data = await invoke('read_image_as_base64', { path: filePath })
        return base64Data
    } catch (error) {
        console.error('读取图片失败:', error)
        throw error
    }
}

// 读取文本文件内容
const readTextFile = async (filePath) => {
    try {
        const fileStats = await invoke('get_file_stats', { path: filePath })
        const fileSize = fileStats.size

        const maxSize = 10 * 1024 * 1024 // 增加到10MB
        const warningSize = 5 * 1024 * 1024 // 5MB时显示警告

        if (fileSize > maxSize) {
            // 大于10MB直接拒绝
            throw new Error(`文件过大 (${(fileSize / 1024 / 1024).toFixed(2)} MB)，超过预览限制 (10 MB)`)
        } else if (fileSize > warningSize) {
            // 5-10MB显示警告但允许预览
            console.warn(`大文件预览警告: ${(fileSize / 1024 / 1024).toFixed(2)} MB，加载可能较慢`)
            ElMessage.warning(`文件较大 (${(fileSize / 1024 / 1024).toFixed(2)} MB)，加载可能需要一些时间`)
        }

        const content = await invoke('read_file', { path: filePath })
        return content
    } catch (error) {
        // 对于UTF-8错误，不抛出异常，而是返回错误信息，让上级处理
        if (error.message && error.message.includes('valid UTF-8')) {
            return null // 返回null表示编码问题，上级可以决定如何处理
        }
        console.error('读取文件失败:', error)
        throw error
    }
}// 打开文件
const openInSystem = async () => {
    try {
        await openPath(props.filePath)
        ElMessage.success('已打开文件')
    } catch (error) {
        ElMessage.error(`无法打开文件: ${error.message || error}`)
    }
}

// 处理图片加载错误
const handleError = () => {
    loading.value = false
    ElMessage.error('图片加载失败')
}

// 关闭预览
const closePreview = () => {
    emit('close')
}

// 加载文件预览
const loadPreview = async () => {
    if (!props.filePath || !props.fileName) return

    fileType.value = detectFileType(props.fileName)
    codeLanguage.value = getCodeLanguage(props.fileName)

    loading.value = true

    try {
        if (fileType.value === 'text') {
            const content = await readTextFile(props.filePath)
            if (content === null) {
                // UTF-8编码问题，但不显示错误，显示提示信息
                fileContent.value = '# 文件编码提示\n\n该文件使用了非UTF-8编码或包含二进制数据，无法正确显示文本内容。\n\n建议使用专门的编辑器或转换工具处理此文件。'
                console.warn('文件编码问题，已显示提示信息')
            } else {
                fileContent.value = content
            }
        } else if (fileType.value === 'image') {
            imageDataUrl.value = await readImageAsBase64(props.filePath)
        }
        loading.value = false
    } catch (error) {
        loading.value = false

        // 处理其他类型的错误
        let errorMessage = error.message || error
        if (errorMessage.includes('文件过大')) {
            ElMessage.error(errorMessage)
        } else {
            ElMessage.error(`预览文件失败: ${errorMessage}`)
        }

        console.error('预览文件失败:', error)
    }
}// 调整面板大小到适合的默认尺寸
const adjustToDefaultSize = () => {
    const windowWidth = window.innerWidth
    const windowHeight = window.innerHeight

    // 使用store中的设置计算默认尺寸
    const { width, height } = settingsStore.getDefaultPreviewSize(fileType.value, windowWidth, windowHeight)

    panelWidth.value = width
    panelHeight.value = height

    // 确保面板不会超出屏幕
    panelBottom.value = 20
    panelRight.value = 20
}

// 开始调整大小
const startResize = (type) => {
    isResizing.value = true
    resizeType.value = type
    startX.value = event.clientX
    startY.value = event.clientY
    startWidth.value = panelWidth.value
    startHeight.value = panelHeight.value

    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopResize)
    event.preventDefault()
}

// 处理调整大小
const handleResize = (event) => {
    if (!isResizing.value) return

    const deltaX = startX.value - event.clientX // 注意方向，向左拖拽是正值
    const deltaY = startY.value - event.clientY // 向上拖拽是正值

    if (resizeType.value === 'left' || resizeType.value === 'corner') {
        const newWidth = startWidth.value + deltaX
        if (newWidth >= 200 && newWidth <= window.innerWidth - 100) {
            panelWidth.value = newWidth
        }
    }

    if (resizeType.value === 'top' || resizeType.value === 'corner') {
        const newHeight = startHeight.value + deltaY
        if (newHeight >= 150 && newHeight <= window.innerHeight - 100) {
            panelHeight.value = newHeight
        }
    }
}

// 停止调整大小
const stopResize = () => {
    isResizing.value = false
    resizeType.value = ''
    // 保存新的面板尺寸到store
    settingsStore.setPreviewPanelSize(panelWidth.value, panelHeight.value)
    console.log('预览面板尺寸已保存到store:', { width: panelWidth.value, height: panelHeight.value })
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
}

// 监听文件变化
watch(() => [props.filePath, props.fileName, props.visible], () => {
    if (props.visible && props.filePath && props.fileName) {
        nextTick(() => {
            adjustToDefaultSize()
            loadPreview()
        })
    }
})

// 监听主题变化，动态加载代码高亮主题
watch(() => currentCodeTheme.value, (newTheme) => {
    loadThemeCSS(newTheme)
}, { immediate: true })

onMounted(() => {
    console.log('PreviewPanel组件已挂载，从store加载的初始尺寸:', {
        width: panelWidth.value,
        height: panelHeight.value,
        previewSettings: settingsStore.previewSettings,
        codeTheme: currentCodeTheme.value,
        codeFont: codeFont.value
    })

    // 立即加载当前主题
    loadThemeCSS(currentCodeTheme.value)
})

onUnmounted(() => {
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
})
</script>

<style lang="scss" scoped>
.preview-panel {
    position: fixed;
    background: var(--el-bg-color);
    border: 1px solid var(--el-border-color);
    border-radius: 8px;
    box-shadow: var(--el-box-shadow-dark);
    z-index: 9999;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 200px;
    min-height: 150px;

    &.image-mode {

        /* 图片模式的默认高度较小 */
        .preview-content {
            text-align: center;
        }
    }

    &.document-mode {

        /* 文档模式的默认高度较大 */
        .preview-content {
            height: 100%;
        }
    }
}

.preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: var(--el-fill-color-lighter);
    border-bottom: 1px solid var(--el-border-color);
    flex-shrink: 0;
}

.preview-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 500;

    span {
        max-width: 200px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
}

.preview-actions {
    display: flex;
    gap: 4px;
    align-items: center;

    /* 换行按钮的特定样式 */
    .el-button[title*="换行"] {
        transition: all 0.2s ease;

        &.el-button--primary {
            color: var(--el-color-primary);
        }

        &.el-button--info {
            color: var(--el-text-color-regular);

            &:hover {
                color: var(--el-color-primary);
            }
        }
    }
}

.preview-content {
    flex: 1;
    overflow: hidden;
    /* 防止双重滚动条 */
    position: relative;

    /* 自定义滚动条样式 */
    * {
        &::-webkit-scrollbar {
            width: 8px;
            height: 8px;
        }

        &::-webkit-scrollbar-track {
            background: var(--el-fill-color-lighter);
            border-radius: 4px;
        }

        &::-webkit-scrollbar-thumb {
            background: var(--el-border-color-darker);
            border-radius: 4px;

            &:hover {
                background: var(--el-border-color-dark);
            }
        }

        &::-webkit-scrollbar-corner {
            background: var(--el-fill-color-lighter);
        }
    }
}

.image-preview {
    height: 100%;
    overflow: auto;
    /* 添加滚动条支持 */
    padding: 16px;
    box-sizing: border-box;

    /* 当图片很小时居中显示，当图片很大时允许滚动 */
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100%;

    img {
        border-radius: 4px;
        max-width: none;
        /* 允许图片显示原始大小 */
        max-height: none;
        /* 允许图片显示原始大小 */
        object-fit: contain;
        display: block;

        /* 最小尺寸确保小图片居中 */
        min-width: auto;
        min-height: auto;
    }
}

.text-preview {
    height: 100%;
    overflow: hidden;
    /* 防止在code-container外部出现滚动条 */

    .code-container {
        height: 100%;
        overflow: auto;
        /* 确保代码容器可以滚动 */
        /* 背景色和字体通过 :style 动态设置 */

        pre {
            margin: 0;
            padding: 16px;
            background: inherit;
            color: inherit;
            font-family: inherit;
            font-size: inherit;
            line-height: 1.4;
            overflow: auto;
            /* 确保pre元素可以横向和纵向滚动 */
            height: 100%;
            box-sizing: border-box;

            /* 这些属性现在通过 :style 动态设置 */
            /* white-space, word-wrap, overflow-wrap 由 codeContainerStyle 控制 */

            /* 确保滚动条在需要时显示 */
            overflow-x: auto;
            overflow-y: auto;

            code {
                background: none;
                padding: 0;
                border: none;
                border-radius: 0;
                color: inherit;
                font-family: inherit;
                font-size: inherit;
                display: block;

                /* 这些属性也通过父元素的动态样式继承 */
                white-space: inherit;
                word-wrap: inherit;
                overflow-wrap: inherit;

                /* 根据换行模式调整宽度 */
                min-width: auto;
                width: auto;
            }
        }
    }
}

.preview-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    gap: 12px;

    .el-icon {
        color: #999;
    }

    p {
        margin: 0;
        font-size: 14px;
    }
}

/* 调整大小的拖拽柄 */
.resize-handles {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;
}

.resize-handle {
    position: absolute;
    pointer-events: auto;
    z-index: 10;

    &.resize-left {
        left: 0;
        top: 0;
        bottom: 0;
        width: 4px;
        cursor: ew-resize;

        &:hover {
            background: var(--el-color-primary);
        }
    }

    &.resize-top {
        top: 0;
        left: 0;
        right: 0;
        height: 4px;
        cursor: ns-resize;

        &:hover {
            background: var(--el-color-primary);
        }
    }

    &.resize-corner {
        top: 0;
        left: 0;
        width: 8px;
        height: 8px;
        cursor: nw-resize;

        &:hover {
            background: var(--el-color-primary);
        }
    }
}

/* 代码高亮样式现在由动态加载的CSS主题文件提供 */
</style>
