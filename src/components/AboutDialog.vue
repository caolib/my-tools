<template>
    <el-dialog :modal="false" v-model="visible" align-center class="about-dialog">
        <div class="about-content">
            <!-- 应用图标和名称 -->
            <div class="app-header">
                <img src="/icon.png" alt="My Tools" class="app-icon" />
                <div class="app-info">
                    <h2 class="app-name">My Tools</h2>
                    <p class="app-version">
                        版本 {{ appVersion }}
                        <el-tag v-if="hasUpdate" type="danger" size="small" effect="dark" style="margin-left: 8px;">
                            有新版本
                        </el-tag>
                    </p>
                </div>
            </div>

            <!-- 更新信息 -->
            <div v-if="hasUpdate" class="update-info">
                <el-alert type="success" :closable="false">
                    <template #title>
                        <div class="update-title">发现新版本 v{{ updateInfo?.version }}</div>
                    </template>
                    <div class="update-content" v-if="updateInfo?.body" v-html="parsedUpdateBody"></div>
                </el-alert>
            </div>

            <!-- 下载进度 -->
            <div v-if="downloading" class="download-progress">
                <el-progress :percentage="downloadProgress"
                    :status="downloadProgress === 100 ? 'success' : undefined" />
                <p class="progress-text">
                    {{ downloadProgress === 100 ? '下载完成，正在安装...' : `正在下载更新... ${downloadProgress}%` }}
                </p>
                <p class="progress-tip">
                    <el-icon style="margin-right: 4px;">
                        <InfoFilled />
                    </el-icon>
                    关闭此对话框不会影响下载
                </p>
            </div>

            <!-- 操作按钮 -->
            <div class="action-buttons">
                <el-button v-if="hasUpdate && !downloading" @click="handleUpdate" type="primary" :loading="updating"
                    :disabled="updating" class="action-btn">
                    <el-icon>
                        <Download />
                    </el-icon>
                    立即更新
                </el-button>
                <el-button v-else-if="downloading" type="info" disabled class="action-btn">
                    <el-icon class="is-loading">
                        <Loading />
                    </el-icon>
                    下载中...
                </el-button>
                <el-button v-else @click="handleCheckUpdate" :loading="checking" :disabled="checking"
                    class="action-btn">
                    <el-icon>
                        <Refresh />
                    </el-icon>
                    {{ checking ? '检查中...' : '检查更新' }}
                </el-button>
                <el-button @click="openRepository" type="primary" plain class="action-btn">
                    <el-icon>
                        <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                            <path
                                d="M512 12.64c-282.752 0-512 229.216-512 512 0 226.208 146.688 418.144 350.08 485.632 25.6 4.736 35.008-11.104 35.008-24.64 0-12.192-0.48-52.544-0.704-95.328-142.464 30.976-172.512-60.416-172.512-60.416-23.296-59.168-56.832-74.912-56.832-74.912-46.464-31.776 3.52-31.136 3.52-31.136 51.392 3.616 78.464 52.768 78.464 52.768 45.664 78.272 119.776 55.648 148.992 42.56 4.576-33.088 17.856-55.68 32.512-68.48-113.728-12.928-233.216-56.864-233.216-253.024 0-55.904 19.936-101.568 52.672-137.408-5.312-12.896-22.848-64.96 4.96-135.488 0 0 42.88-13.76 140.8 52.48 40.832-11.36 84.64-17.024 128.16-17.248 43.488 0.192 87.328 5.888 128.256 17.248 97.728-66.24 140.64-52.48 140.64-52.48 27.872 70.528 10.336 122.592 5.024 135.488 32.832 35.84 52.608 81.504 52.608 137.408 0 196.64-119.776 239.936-233.792 252.64 18.368 15.904 34.72 47.04 34.72 94.816 0 68.512-0.608 123.648-0.608 140.512 0 13.632 9.216 29.6 35.168 24.576C877.44 942.592 1024 750.592 1024 524.64c0-282.784-229.248-512-512-512z"
                                fill="currentColor" />
                        </svg>
                    </el-icon>
                    仓库
                </el-button>
                <el-button @click="openIssues" type="default" plain class="action-btn">
                    <el-icon>
                        <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                            <path
                                d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm-32 232c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V296zm32 440a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z"
                                fill="currentColor" />
                        </svg>
                    </el-icon>
                    反馈
                </el-button>
            </div>

            <!-- 版权信息 -->
            <div class="copyright">
                <p>&copy; 2025 caolib. 基于 GPL v3 许可证开源</p>
            </div>
        </div>
    </el-dialog>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Download, Refresh, InfoFilled, Loading } from '@element-plus/icons-vue'
import { useUpdateStore } from '@/stores/update'
import { marked } from 'marked'

const visible = ref(false)
const appVersionData = ref('0.1.0')
const updateStore = useUpdateStore()
const updating = ref(false)
const isComponentMounted = ref(true)

// 计算属性
const hasUpdate = computed(() => updateStore.hasUpdate)
const updateInfo = computed(() => updateStore.updateInfo)
const checking = computed(() => updateStore.checking)
const downloading = computed(() => updateStore.downloading)
const downloadProgress = computed(() => updateStore.downloadProgress)

// 解析 Markdown 格式的更新内容
const parsedUpdateBody = computed(() => {
    if (!updateInfo.value?.body) {
        return ''
    }
    try {
        return marked.parse(updateInfo.value.body)
    } catch (error) {
        console.error('解析 Markdown 失败:', error)
        return updateInfo.value.body
    }
})

// 获取应用版本信息
const getAppVersion = async () => {
    try {
        // 从 Tauri 后端获取应用版本
        const version = await invoke('get_app_version')
        appVersionData.value = version
    } catch (error) {
        console.warn('无法获取应用版本:', error)
        // 降级使用环境变量或默认值
        appVersionData.value = import.meta.env.VITE_APP_VERSION || '0.1.0'
    }
}

const appVersion = computed(() => appVersionData.value)

onMounted(() => {
    isComponentMounted.value = true
    getAppVersion()
})

onBeforeUnmount(() => {
    isComponentMounted.value = false
    // 清理更新状态
    if (checking.value || downloading.value) {
        console.log('组件卸载，清理更新状态')
        updateStore.setChecking(false)
        updateStore.setDownloading(false)
    }
})

const showDialog = () => {
    visible.value = true
}

// 检查更新
const handleCheckUpdate = async () => {
    console.log('开始检查更新...')
    try {
        updateStore.setChecking(true)
        console.log('checking 状态已设置为 true')

        const { check } = await import('@tauri-apps/plugin-updater')
        console.log('updater 插件已加载')

        const update = await check()
        console.log('检查更新结果:', update)

        if (update) {
            updateStore.setHasUpdate(true)
            updateStore.setUpdateInfo({
                version: update.version,
                currentVersion: update.currentVersion,
                date: update.date,
                body: update.body
            })
            console.log('发现新版本:', update.version)
            ElMessage.success(`发现新版本 v${update.version}`)
        } else {
            console.log('当前已是最新版本')
            updateStore.setHasUpdate(false)
            ElMessage.info('当前已是最新版本')
        }
    } catch (error) {
        console.error('检查更新失败:', error)
        ElMessage.error('检查更新失败：' + (error.message || String(error)))
        updateStore.setHasUpdate(false)
    } finally {
        updateStore.setChecking(false)
        console.log('checking 状态已重置为 false')
    }
}

// 执行更新
const handleUpdate = async () => {
    // 防止重复点击
    if (updating.value || downloading.value) {
        console.log('更新已在进行中，忽略重复点击')
        ElMessage.warning('更新正在进行中，请稍候...')
        return
    }

    try {
        await ElMessageBox.confirm(
            '更新将在后台下载并自动安装，完成后将重启应用。关闭此对话框不会影响下载。',
            '确认更新',
            {
                confirmButtonText: '立即更新',
                cancelButtonText: '取消',
                type: 'info'
            }
        )

        updating.value = true
        updateStore.setDownloading(true)
        updateStore.setDownloadProgress(0)
        console.log('开始更新流程...')

        // 备份 localStorage 数据到 Tauri 的 appDataDir，防止更新时丢失
        try {
            const { appDataDir } = await import('@tauri-apps/api/path')
            const { writeTextFile } = await import('@tauri-apps/plugin-fs')

            // 备份所有 Pinia 持久化存储的数据
            const backupData = {
                'wem-settings': localStorage.getItem('wem-settings'),
                'wem-file-search-settings': localStorage.getItem('wem-file-search-settings'),
                'wem-file-types': localStorage.getItem('wem-file-types'),
                'wem-commit-types': localStorage.getItem('wem-commit-types'),
                timestamp: new Date().toISOString()
            }

            const dataPath = await appDataDir()
            await writeTextFile(`${dataPath}/update-backup.json`, JSON.stringify(backupData, null, 2))
            console.log('数据备份成功，备份文件:', `${dataPath}/update-backup.json`)
        } catch (backupError) {
            console.warn('备份数据失败，但继续更新:', backupError)
            // 备份失败不应阻止更新流程
        }

        const { check } = await import('@tauri-apps/plugin-updater')
        const { relaunch } = await import('@tauri-apps/plugin-process')

        const update = await check()

        if (!update) {
            console.log('没有可用更新')
            ElMessage.info('当前已是最新版本')
            updating.value = false
            updateStore.setDownloading(false)
            return
        }

        console.log('开始下载更新:', update.version)

        // 跟踪下载进度
        let downloaded = 0
        let contentLength = 0

        // 监听下载进度
        await update.downloadAndInstall((event) => {
            switch (event.event) {
                case 'Started':
                    contentLength = event.data.contentLength
                    downloaded = 0
                    updateStore.setDownloadProgress(0)
                    console.log('开始下载更新，大小:', contentLength, '字节')
                    break
                case 'Progress':
                    downloaded += event.data.chunkLength
                    const progress = Math.round((downloaded / contentLength) * 100)
                    updateStore.setDownloadProgress(progress)
                    console.log(`下载进度: ${progress}% (${downloaded}/${contentLength} 字节)`)
                    break
                case 'Finished':
                    updateStore.setDownloadProgress(100)
                    console.log('下载完成，正在安装...')
                    break
            }
        })

        console.log('更新下载并安装完成')
        ElMessage.success('更新完成，即将重启应用...')

        // 延迟1秒后重启
        setTimeout(async () => {
            console.log('准备重启应用...')
            await relaunch()
        }, 1000)

    } catch (error) {
        if (error !== 'cancel') {
            console.error('更新失败:', error)
            ElMessage.error('更新失败：' + (error.message || String(error)))
        } else {
            console.log('用户取消更新')
        }
        // 取消或失败时重置状态
        updating.value = false
        updateStore.setDownloading(false)
        updateStore.setDownloadProgress(0)
    }
    // 注意: 成功完成后不重置状态,因为会立即重启
}

// 打开 GitHub 仓库
const openRepository = () => {
    openUrl('https://github.com/caolib/my-tools')
}

// 打开 Issues 页面
const openIssues = () => {
    openUrl('https://github.com/caolib/my-tools/issues')
}

// 暴露方法给父组件
defineExpose({
    showDialog
})
</script>

<style lang="scss" src="@/assets/styles/about-dialog.scss" scoped></style>