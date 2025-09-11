<template>
    <el-dialog v-model="visible" :title="fileName" width="80%" :close-on-click-modal="true"
        :close-on-press-escape="true" class="file-preview-dialog" draggable @close="handleClose">
        <div class="file-preview-content" v-loading="loading">
            <!-- 图片预览 -->
            <div v-if="fileType === 'image'" class="image-preview">
                <viewer :images="[imageDataUrl]" :rebuild="true" class="image-viewer">
                    <img :src="imageDataUrl" @load="loading = false" @error="handleError"
                        style="max-width: 100%; height: auto; cursor: pointer;" />
                </viewer>
            </div>

            <!-- 文本文件预览 -->
            <div v-else-if="fileType === 'text'" class="text-preview">
                <div class="code-container">
                    <pre v-if="!codeLanguage"><code v-html="highlightedContent"></code></pre>
                    <pre v-else><code :class="`language-${codeLanguage}`" v-html="highlightedContent"></code></pre>
                </div>
            </div>

            <!-- PDF 预览占位符 -->
            <div v-else-if="fileType === 'pdf'" class="pdf-preview">
                <div class="preview-placeholder">
                    <el-icon :size="48">
                        <Document />
                    </el-icon>
                    <p>PDF 文件预览功能即将推出</p>
                    <el-button type="primary" @click="openInSystem">在系统默认应用中打开</el-button>
                </div>
            </div>

            <!-- 不支持的文件类型 -->
            <div v-else class="unsupported-preview">
                <div class="preview-placeholder">
                    <el-icon :size="48">
                        <Warning />
                    </el-icon>
                    <p>不支持预览此类型的文件</p>
                    <el-button type="primary" @click="openInSystem">在系统默认应用中打开</el-button>
                </div>
            </div>
        </div>

        <template #footer>
            <span class="dialog-footer">
                <el-button @click="visible = false">关闭</el-button>
                <el-button type="primary" @click="openInSystem">打开文件</el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Document, Warning } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { openPath } from '@tauri-apps/plugin-opener'
import 'viewerjs/dist/viewer.css'
import { component as Viewer } from 'v-viewer'
import hljs from 'highlight.js'
import 'highlight.js/styles/vs2015.css' // 使用 Visual Studio 2015 深色主题

const props = defineProps({
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

const visible = ref(false)
const loading = ref(false)
const fileContent = ref('')
const fileType = ref('')
const codeLanguage = ref('')
const imageDataUrl = ref('')

// 支持的图片格式
const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg', '.ico']

// 支持的文本文件格式
const textExtensions = [
    '.txt', '.md', '.js', '.ts', '.vue', '.html', '.css', '.scss', '.sass', '.less',
    '.json', '.xml', '.yml', '.yaml', '.ini', '.conf', '.log', '.sql', '.py', '.java',
    '.c', '.cpp', '.h', '.hpp', '.cs', '.php', '.rb', '.go', '.rs', '.sh', '.bat',
    '.ps1', '.dockerfile', '.gitignore', '.env'
]

// 获取用于显示的文件路径（处理 Windows 路径）
const filePathForDisplay = computed(() => {
    if (!props.filePath) return ''
    // 将 Windows 路径转换为 file:// URL 格式以便在浏览器中显示
    if (props.filePath.includes(':\\')) {
        return `file:///${props.filePath.replace(/\\/g, '/')}`
    }
    return props.filePath
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
    } else if (ext === '.pdf') {
        return 'pdf'
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
        '.vue': 'html', // Vue 文件使用 HTML 高亮
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

// 读取文本文件内容（带性能优化）
const readTextFile = async (filePath) => {
    try {
        // 首先检查文件大小
        const fileStats = await invoke('get_file_stats', { path: filePath })
        const fileSize = fileStats.size

        // 限制文件大小为 1MB
        const maxSize = 1 * 1024 * 1024 // 1MB
        if (fileSize > maxSize) {
            throw new Error(`文件太大 (${(fileSize / 1024 / 1024).toFixed(2)} MB)，无法预览。建议使用专用编辑器打开。`)
        }

        // 如果文件较大（超过100KB），显示警告并允许用户选择是否继续
        if (fileSize > 100 * 1024) { // 100KB
            const shouldContinue = await ElMessageBox.confirm(
                `文件大小为 ${(fileSize / 1024).toFixed(2)} KB，加载可能较慢。是否继续预览？`,
                '文件较大',
                {
                    confirmButtonText: '继续预览',
                    cancelButtonText: '取消',
                    type: 'warning',
                }
            )
            if (!shouldContinue) {
                throw new Error('用户取消预览')
            }
        }

        const content = await invoke('read_file', { path: filePath })
        return content
    } catch (error) {
        console.error('读取文件失败:', error)
        throw error
    }
}

// 打开文件
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

// 关闭对话框
const handleClose = () => {
    emit('close')
}

// 显示预览
const showPreview = async () => {
    if (!props.filePath || !props.fileName) {
        ElMessage.error('文件路径或文件名不能为空')
        return
    }

    fileType.value = detectFileType(props.fileName)
    codeLanguage.value = getCodeLanguage(props.fileName)

    visible.value = true
    loading.value = true

    try {
        if (fileType.value === 'text') {
            fileContent.value = await readTextFile(props.filePath)
            loading.value = false
        } else if (fileType.value === 'image') {
            imageDataUrl.value = await readImageAsBase64(props.filePath)
            loading.value = false
        } else {
            loading.value = false
        }
    } catch (error) {
        loading.value = false
        ElMessage.error(`预览文件失败: ${error.message || error}`)
    }
}

// 监听文件路径变化
watch(() => [props.filePath, props.fileName], () => {
    if (props.filePath && props.fileName) {
        showPreview()
    }
}, { immediate: true })

// 暴露方法
defineExpose({
    showPreview
})
</script>

<style lang="scss" scoped>
.file-preview-dialog {
    :deep(.el-dialog__body) {
        padding: 20px;
        max-height: 70vh;
        overflow: auto;
    }
}

.file-preview-content {
    min-height: 200px;
}

.image-preview {
    text-align: center;

    .image-viewer {
        display: inline-block;
    }
}

.text-preview {
    .code-container {
        border: 1px solid #333;
        border-radius: 6px;
        overflow: hidden;
        background-color: #1e1e1e;

        pre {
            margin: 0;
            padding: 16px;
            background-color: #1e1e1e;
            color: #d4d4d4;
            font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
            font-size: 14px;
            line-height: 1.4;
            overflow-x: auto;
            white-space: pre-wrap;
            word-wrap: break-word;

            code {
                background: none;
                padding: 0;
                border: none;
                border-radius: 0;
                color: inherit;
            }
        }
    }
}

.preview-placeholder {
    text-align: center;
    padding: 40px;
    color: #666;

    .el-icon {
        margin-bottom: 16px;
        color: #999;
    }

    p {
        margin: 16px 0;
        font-size: 16px;
    }
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

/* 代码高亮样式调整 - 使用深色主题 */
:deep(.hljs) {
    background: #1e1e1e !important;
    color: #d4d4d4 !important;
    padding: 0 !important;

    /* VS Code 深色主题样式 */
    .hljs-keyword {
        color: #569cd6;
    }

    .hljs-string {
        color: #ce9178;
    }

    .hljs-comment {
        color: #6a9955;
    }

    .hljs-number {
        color: #b5cea8;
    }

    .hljs-variable {
        color: #9cdcfe;
    }

    .hljs-function {
        color: #dcdcaa;
    }

    .hljs-class {
        color: #4ec9b0;
    }

    .hljs-title {
        color: #dcdcaa;
    }

    .hljs-type {
        color: #4ec9b0;
    }

    .hljs-attr {
        color: #92c5f8;
    }

    .hljs-property {
        color: #9cdcfe;
    }

    .hljs-built_in {
        color: #4ec9b0;
    }
}
</style>
