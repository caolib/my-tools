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
import { useSettingsStore } from '@/stores/settings'
import Titlebar from './components/Titlebar.vue'

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
      // 恢复窗口位置和大小
      await currentWindow.setPosition(windowState.position)
      await currentWindow.setSize(windowState.size)

      // 恢复最大化状态
      if (windowState.isMaximized) {
        await currentWindow.maximize()
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

  // 监听窗口变化事件
  const unlistenMove = await currentWindow.listen('tauri://move', saveWindowState)
  const unlistenResize = await currentWindow.listen('tauri://resize', saveWindowState)

  // 保存清理函数
  onBeforeUnmount(() => {
    unlistenMove()
    unlistenResize()
  })
})
</script>