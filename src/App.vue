<template>
  <div id="app">
    <Titlebar />
    <router-view />
    <el-backtop :right="10" :bottom="10" />
  </div>
</template>

<script setup>
import { onMounted, onBeforeUnmount } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'
import { useUpdateStore } from '@/stores/update'
import Titlebar from './components/Titlebar.vue'
import { registerAllShortcuts } from '@/utils/shortcutManager'

const router = useRouter()
const settingsStore = useSettingsStore()
const updateStore = useUpdateStore()
let currentWindow = null

// 检查更新
const checkForUpdates = async () => {
  try {
    updateStore.setChecking(true)
    const { check } = await import('@tauri-apps/plugin-updater')
    const update = await check()

    if (update) {
      updateStore.setHasUpdate(true)
      updateStore.setUpdateInfo({
        version: update.version,
        currentVersion: update.currentVersion,
        date: update.date,
        body: update.body
      })
      console.log('发现新版本:', update.version)
    } else {
      console.log('当前已是最新版本')
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setError(error.message || '检查更新失败')
  } finally {
    updateStore.setChecking(false)
  }
}

// 防抖函数，避免频繁保存窗口状态
function debounce(func, wait) {
  let timeout
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout)
      func(...args)
    }
    clearTimeout(timeout)
    timeout = setTimeout(later, wait)
  }
}

// 保存窗口状态
const saveWindowState = debounce(async () => {
  if (!currentWindow) return

  try {
    const [position, size, isMaximized] = await Promise.all([
      currentWindow.outerPosition(),
      currentWindow.outerSize(),
      currentWindow.isMaximized()
    ])

    settingsStore.updateWindowState({
      position: { x: position.x, y: position.y },
      size: { width: size.width, height: size.height },
      isMaximized,
    })
  } catch (error) {
    console.error('保存窗口状态失败:', error)
  }
}, 500)

// 恢复更新前的数据备份
const restoreDataFromBackup = async () => {
  try {
    const { appDataDir } = await import('@tauri-apps/api/path')
    const { readTextFile, exists, remove } = await import('@tauri-apps/plugin-fs')

    const dataPath = await appDataDir()
    const backupFilePath = `${dataPath}/update-backup.json`

    // 检查备份文件是否存在
    if (await exists(backupFilePath)) {
      console.log('发现更新备份文件，正在检查...')

      const backupContent = await readTextFile(backupFilePath)
      const backupData = JSON.parse(backupContent)

      // 检查 localStorage 是否为空（说明数据被更新清除了）
      const currentSettings = localStorage.getItem('wem-settings')

      if (!currentSettings && backupData['wem-settings']) {
        console.log('检测到数据丢失，正在从备份恢复...')

        // 恢复所有备份的数据
        Object.entries(backupData).forEach(([key, value]) => {
          if (key !== 'timestamp' && value) {
            localStorage.setItem(key, value)
            console.log(`已恢复: ${key}`)
          }
        })

        // 删除备份文件
        await remove(backupFilePath)
        console.log('数据恢复成功，备份文件已删除')

        // 显示成功消息
        ElMessage.success({
          message: '检测到更新后数据丢失，已自动恢复您的设置和配置',
          duration: 5000
        })
      } else {
        // 数据完好，删除备份文件
        await remove(backupFilePath)
        console.log('数据完好，无需恢复，备份文件已删除')
      }
    }
  } catch (error) {
    // 恢复失败不应影响应用启动
    console.warn('检查/恢复备份失败:', error)
  }
}

// 恢复窗口状态
const restoreWindowState = async () => {
  if (!currentWindow) return

  try {
    const { windowState } = settingsStore

    // 如果有保存的窗口状态
    if (windowState.position && windowState.size) {
      try {
        // 恢复窗口位置和大小 - 使用 Physical 坐标
        await currentWindow.setPosition({
          type: 'Physical',
          x: Math.round(windowState.position.x || 0),
          y: Math.round(windowState.position.y || 0)
        })
        await currentWindow.setSize({
          type: 'Physical',
          width: Math.round(windowState.size.width || 800),
          height: Math.round(windowState.size.height || 600)
        })

        // 恢复最大化状态
        if (windowState.isMaximized) {
          await currentWindow.maximize()
        }
      } catch (error) {
        console.error('恢复窗口位置/大小失败:', error)
      }
    }
  } catch (error) {
    console.error('恢复窗口状态失败:', error)
  }
}

onMounted(async () => {
  currentWindow = getCurrentWindow()

  // 尝试恢复更新前的数据备份
  await restoreDataFromBackup()

  // 恢复窗口状态
  await restoreWindowState()

  // 注册全局快捷键
  try {
    await registerAllShortcuts()
  } catch (error) {
    console.error('注册全局快捷键失败:', error)
  }

  // 检查更新（启动时）
  // 仅在生产环境自动检查,避免开发时 HMR 导致的回调警告
  if (import.meta.env.PROD) {
    setTimeout(() => {
      checkForUpdates()
    }, 3000) // 延迟3秒检查，避免影响启动速度
  } else {
    console.log('开发模式: 跳过自动检查更新，可通过"关于"对话框手动检查')
  }

  // 监听导航事件（从托盘菜单触发）
  const unlistenNavigate = await currentWindow.listen('navigate', (event) => {
    const path = event.payload
    console.log('导航到:', path)
    router.push(path)
  })

  // 初始化托盘菜单（包含提交类型）
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const { useCommitTypesStore } = await import('@/stores/commitTypes')
    const commitTypesStore = useCommitTypesStore()

    const commitTypes = commitTypesStore.allCommitTypes.map(ct => ({
      value: ct.value,
      label: ct.label,
      icon: ct.icon
    }))

    await invoke('update_tray_menu_with_commit_types', {
      envVarManager: settingsStore.globalShortcuts.envVarManager || '',
      fileSearch: settingsStore.globalShortcuts.fileSearch || '',
      projects: settingsStore.globalShortcuts.projects || '',
      commitGenerator: settingsStore.globalShortcuts.commitGenerator || '',
      commitTypes
    })
  } catch (error) {
    console.error('初始化托盘菜单失败:', error)
  }

  // 监听窗口关闭请求事件
  const unlistenCloseRequested = await currentWindow.onCloseRequested(async (event) => {
    const { closeToTray } = settingsStore

    if (closeToTray) {
      // 最小化到托盘
      event.preventDefault()
      await currentWindow.hide()
    }
    // 如果 closeToTray 为 false，则让窗口正常关闭
  })

  // 监听窗口变化事件
  const unlistenMove = await currentWindow.listen('tauri://move', saveWindowState)
  const unlistenResize = await currentWindow.listen('tauri://resize', saveWindowState)

  // 保存清理函数
  onBeforeUnmount(() => {
    unlistenNavigate()
    unlistenCloseRequested()
    unlistenMove()
    unlistenResize()
  })
})
</script>