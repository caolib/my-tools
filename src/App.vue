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
import { useSettingsStore } from '@/stores/settings'
import Titlebar from './components/Titlebar.vue'
import { registerAllShortcuts } from '@/utils/shortcutManager'

const router = useRouter()
const settingsStore = useSettingsStore()
let currentWindow = null

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

  // 恢复窗口状态
  await restoreWindowState()

  // 注册全局快捷键
  try {
    await registerAllShortcuts()
  } catch (error) {
    console.error('注册全局快捷键失败:', error)
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