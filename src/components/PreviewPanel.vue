<template>
    <div v-if="visible" class="preview-panel"
        :class="{ 'image-mode': fileType === 'image', 'document-mode': fileType === 'text', 'pdf-mode': fileType === 'pdf', 'table-mode': fileType === 'table' }"
        :style="panelStyle" ref="panelRef">
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

            <!-- PDF预览 -->
            <div v-else-if="fileType === 'pdf'" class="pdf-preview">
                <div class="pdf-controls">
                    <el-button-group size="small">
                        <el-button @click="zoomOut" :disabled="pdfScale <= 0.25" title="缩小">
                            <el-icon>
                                <ZoomOut />
                            </el-icon>
                        </el-button>
                        <el-button @click="resetZoom" title="重置缩放">{{ Math.round(pdfScale * 100) }}%</el-button>
                        <el-button @click="zoomIn" :disabled="pdfScale >= 3" title="放大">
                            <el-icon>
                                <ZoomIn />
                            </el-icon>
                        </el-button>
                    </el-button-group>
                    <span class="page-info">{{ pages }} 页</span>
                </div>
                <div class="pdf-container">
                    <div v-if="pdf && pages > 0">
                        <div v-for="page in pages" :key="page" class="pdf-page">
                            <VuePDF :pdf="pdf" :page="page" :scale="pdfScale" text-layer annotation-layer
                                @loaded="onPdfLoaded" />
                        </div>
                    </div>
                </div>
            </div>

            <!-- 文本预览 -->
            <div v-else-if="fileType === 'text'" class="text-preview">
                <div class="code-container" :style="codeContainerStyle">
                    <pre v-if="!codeLanguage" :style="preStyle"><code v-html="highlightedContent"></code></pre>
                    <pre v-else
                        :style="preStyle"><code :class="`language-${codeLanguage}`" v-html="highlightedContent"></code></pre>
                </div>
            </div>

            <!-- 表格预览 -->
            <div v-else-if="fileType === 'table'" class="table-preview">
                <div class="table-controls" v-if="tableWorksheets.length > 1">
                    <el-tabs v-model="currentWorksheet" type="card" size="small">
                        <el-tab-pane v-for="(worksheet, index) in tableWorksheets" :key="index" :label="worksheet.name"
                            :name="index" />
                    </el-tabs>
                </div>
                <div class="table-container">
                    <!-- 数据量警告 -->
                    <div v-if="tableData.length > maxRowsWarning" class="table-warning">
                        <el-alert :title="`数据量较大 (${tableData.length} 行)，建议分页查看以获得更好的性能`" type="warning"
                            :closable="false" show-icon />
                    </div>

                    <el-table :data="currentPageTableData" border stripe size="small" max-height="400"
                        style="width: 100%" :header-cell-style="{ backgroundColor: 'var(--el-fill-color-light)' }">
                        <el-table-column prop="_rowIndex" label="行号" width="60" align="center" fixed="left" />
                        <el-table-column v-for="(header, index) in tableHeaders" :key="index" :prop="header.prop"
                            :label="header.label" :width="header.width || 120" show-overflow-tooltip />
                    </el-table>

                    <!-- 分页组件 -->
                    <div v-if="tableData.length > 0" class="table-pagination">
                        <el-pagination v-model:current-page="tablePagination.currentPage"
                            v-model:page-size="tablePagination.pageSize" :page-sizes="[50, 100, 200, 500, 1000]"
                            :small="true" layout="total, sizes, prev, pager, next, jumper"
                            :total="tablePagination.total"
                            @size-change="(size) => { tablePagination.pageSize = size; tablePagination.currentPage = 1; }"
                            @current-change="(page) => { tablePagination.currentPage = page; }" />
                    </div>

                    <div v-if="tableData.length === 0" class="table-empty">
                        <el-empty description="暂无数据" />
                    </div>
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
import { View, FolderOpened, Close, Warning, Document, Menu, ZoomIn, ZoomOut } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { openPath } from '@tauri-apps/plugin-opener'
import { useSettingsStore } from '@/stores/settings'
import { useFileSearchSettingsStore } from '@/stores/fileSearchSettings'
import hljs from 'highlight.js'
import { VuePDF, usePDF } from '@tato30/vue-pdf'
import '@tato30/vue-pdf/style.css'
import * as XLSX from 'xlsx'

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
}

const loading = ref(false)
const fileContent = ref('')
const fileType = ref('')
const codeLanguage = ref('')
const imageDataUrl = ref('')
const panelRef = ref(null)

// PDF相关变量
const pdfSource = ref('')
const { pdf, pages, info } = usePDF(pdfSource)
const pdfScale = ref(1)

// 表格相关变量
const tableWorksheets = ref({})
const tableWorksheetNames = ref([])
const currentWorksheet = ref('')

// 表格分页变量
const tablePagination = ref({
    currentPage: 1,
    pageSize: 100, // 默认每页100行
    total: 0
})
const maxRowsWarning = 1000 // 超过1000行显示警告
const maxRowsLimit = 10000 // 最大处理10000行

// 面板尺寸和位置 - 根据文件类型动态获取初始值
const panelWidth = ref(400)
const panelHeight = ref(300)
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
const pdfExtensions = ['.pdf']
const tableExtensions = ['.xlsx', '.xls', '.csv', '.tsv', '.ods']

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
    } else if (pdfExtensions.includes(ext)) {
        return 'pdf'
    } else if (tableExtensions.includes(ext)) {
        return 'table'
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

// 读取PDF文件为Blob URL
const readPdfAsBlob = async (filePath) => {
    try {
        // 使用Tauri的fs插件读取文件
        const { readFile } = await import('@tauri-apps/plugin-fs')
        const fileBytes = await readFile(filePath)

        // 创建Blob并生成URL
        const blob = new Blob([fileBytes], { type: 'application/pdf' })
        const blobUrl = URL.createObjectURL(blob)

        return blobUrl
    } catch (error) {
        throw error
    }
}

// 读取表格文件并解析
const readTableFile = async (filePath) => {
    try {
        // 使用Tauri的fs插件读取文件
        const { readFile } = await import('@tauri-apps/plugin-fs')
        const fileBytes = await readFile(filePath)

        // 创建ArrayBuffer
        const arrayBuffer = fileBytes.buffer.slice(
            fileBytes.byteOffset,
            fileBytes.byteOffset + fileBytes.byteLength
        )

        // 使用XLSX解析文件，添加性能优化选项
        const workbook = XLSX.read(arrayBuffer, {
            type: 'buffer',
            cellDates: false, // 不解析日期，提高性能
            cellNF: false,    // 不解析数字格式，提高性能
            cellStyles: false, // 不解析样式，提高性能
            sheetStubs: false  // 不包含空单元格，提高性能
        })

        // 获取工作表名称列表
        tableWorksheetNames.value = workbook.SheetNames

        // 解析所有工作表数据
        tableWorksheets.value = {}
        let hasLargeData = false

        workbook.SheetNames.forEach(sheetName => {
            // 将工作表转换为JSON格式，保留空单元格
            const worksheet = XLSX.utils.sheet_to_json(workbook.Sheets[sheetName], {
                header: 1, // 使用数组格式而不是对象格式
                defval: '', // 空单元格的默认值
                blankrows: false, // 忽略空行
                range: undefined // 不限制范围，后续手动截取
            })

            // 检查数据量并截取
            if (worksheet.length > maxRowsLimit) {
                ElMessage.warning(`工作表 "${sheetName}" 数据量过大 (${worksheet.length} 行)，仅显示前 ${maxRowsLimit} 行`)
                tableWorksheets.value[sheetName] = worksheet.slice(0, maxRowsLimit)
                hasLargeData = true
            } else if (worksheet.length > maxRowsWarning) {
                hasLargeData = true
                tableWorksheets.value[sheetName] = worksheet
            } else {
                tableWorksheets.value[sheetName] = worksheet
            }
        })

        // 显示性能提示
        if (hasLargeData) {
            ElMessage.info('检测到大量数据，已启用分页模式以提升性能')
        }

        // 设置当前活动工作表为第一个
        if (tableWorksheetNames.value.length > 0) {
            currentWorksheet.value = tableWorksheetNames.value[0]
        }

        return true
    } catch (error) {
        throw error
    }
}

// 读取图片文件为 base64
const readImageAsBase64 = async (filePath) => {
    try {
        const base64Data = await invoke('read_image_as_base64', { path: filePath })
        return base64Data
    } catch (error) {
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

// PDF控制方法
const zoomIn = () => {
    if (pdfScale.value < 3) {
        pdfScale.value = Math.min(3, pdfScale.value + 0.25)
    }
}

const zoomOut = () => {
    if (pdfScale.value > 0.25) {
        pdfScale.value = Math.max(0.25, pdfScale.value - 0.25)
    }
}

const resetZoom = () => {
    pdfScale.value = 1
}

// 表格相关计算属性
const currentWorksheetData = computed(() => {
    if (!currentWorksheet.value || !tableWorksheets.value[currentWorksheet.value]) {
        return []
    }
    return tableWorksheets.value[currentWorksheet.value]
})

const tableHeaders = computed(() => {
    if (currentWorksheetData.value.length === 0) return []

    // 使用第一行作为表头
    const firstRow = currentWorksheetData.value[0]
    if (!Array.isArray(firstRow)) return []

    return firstRow.map((header, index) => {
        return {
            prop: `col${index}`,
            label: header || `列${index + 1}`,
            minWidth: '120'
        }
    })
})

const tableData = computed(() => {
    if (currentWorksheetData.value.length <= 1) return []

    // 跳过第一行（表头），将其余行转换为对象格式
    const allRows = currentWorksheetData.value.slice(1).map((row, rowIndex) => {
        const rowData = { _rowIndex: rowIndex + 1 }
        if (Array.isArray(row)) {
            row.forEach((cell, colIndex) => {
                rowData[`col${colIndex}`] = cell || ''
            })
        }
        return rowData
    })

    // 更新总数
    tablePagination.value.total = allRows.length

    return allRows
})

// 当前页的表格数据
const currentPageTableData = computed(() => {
    const start = (tablePagination.value.currentPage - 1) * tablePagination.value.pageSize
    const end = start + tablePagination.value.pageSize
    return tableData.value.slice(start, end)
})

// 表格工作表切换
const switchWorksheet = (sheetName) => {
    currentWorksheet.value = sheetName
    // 重置分页
    tablePagination.value.currentPage = 1
}

const onPdfLoaded = (pdfInfo) => {
    loading.value = false
}

// 关闭预览
const closePreview = () => {
    // 清理PDF Blob URL以避免内存泄漏
    if (pdfSource.value && pdfSource.value.startsWith('blob:')) {
        URL.revokeObjectURL(pdfSource.value)
        pdfSource.value = ''
    }
    emit('close')
}

// 加载文件预览
const loadPreview = async () => {
    if (!props.filePath || !props.fileName) return

    fileType.value = detectFileType(props.fileName)
    codeLanguage.value = getCodeLanguage(props.fileName)

    // 加载对应文件类型的预览配置
    const config = settingsStore.getPreviewConfig(fileType.value)
    panelWidth.value = config.panelWidth
    panelHeight.value = config.panelHeight

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
        } else if (fileType.value === 'pdf') {
            // 对于PDF文件，读取为Blob URL
            const blobUrl = await readPdfAsBlob(props.filePath)
            pdfSource.value = blobUrl
            pdfScale.value = 1
        } else if (fileType.value === 'table') {
            // 对于表格文件，使用XLSX读取
            // 重置分页状态
            tablePagination.value.currentPage = 1
            tablePagination.value.pageSize = 100
            await readTableFile(props.filePath)
        }

        // 对于PDF，加载状态由onPdfLoaded处理
        if (fileType.value !== 'pdf') {
            loading.value = false
        }
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
    // 保存新的面板尺寸到对应文件类型的store配置
    settingsStore.setPreviewPanelSize(fileType.value, panelWidth.value, panelHeight.value)
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
    // PreviewPanel mounted

    // 立即加载当前主题
    loadThemeCSS(currentCodeTheme.value)
})

onUnmounted(() => {
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
})
</script>

<style lang="scss" src="../assets/styles/preview-panel.scss" scoped></style>
