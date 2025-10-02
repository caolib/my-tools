import { register, unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useSettingsStore } from '@/stores/settings'
import router from '@/router'

// 路由映射
const routeMap = {
    envVarManager: '/env',
    fileSearch: '/file-search',
    projects: '/projects',
    commitGenerator: '/commit-generator'
}

// 获取快捷键标签
const getKeyLabel = (key) => {
    const labels = {
        envVarManager: '环境变量管理',
        fileSearch: '文件搜索',
        projects: '项目管理',
        commitGenerator: '提交生成器'
    }
    return labels[key] || key
}

// 注册所有快捷键
export async function registerAllShortcuts() {
    const settingsStore = useSettingsStore()
    const shortcuts = {
        envVarManager: settingsStore.getGlobalShortcut('envVarManager'),
        fileSearch: settingsStore.getGlobalShortcut('fileSearch'),
        projects: settingsStore.getGlobalShortcut('projects'),
        commitGenerator: settingsStore.getGlobalShortcut('commitGenerator')
    }

    for (const key in shortcuts) {
        const shortcut = shortcuts[key]
        if (shortcut) {
            try {
                // 检查是否已注册，避免重复注册
                const alreadyRegistered = await isRegistered(shortcut)
                if (alreadyRegistered) {
                    console.log(`⚠️ 快捷键已注册，跳过: ${shortcut}`)
                    continue
                }

                await register(shortcut, async () => {
                    // 显示并聚焦窗口
                    const webview = getCurrentWebviewWindow()
                    await webview.show()
                    await webview.setFocus()

                    // 导航到对应路由
                    const route = routeMap[key]
                    if (route) {
                        router.push(route)
                    }
                })
                console.log(`✅ 已注册全局快捷键: ${shortcut} -> ${getKeyLabel(key)}`)
            } catch (error) {
                console.warn(`❌ 注册全局快捷键失败: ${shortcut}`, error)
            }
        }
    }
}

// 注册单个快捷键
export async function registerShortcut(key, shortcut) {
    try {
        // 检查是否已注册
        const alreadyRegistered = await isRegistered(shortcut)
        if (alreadyRegistered) {
            throw new Error('该快捷键已被系统占用')
        }

        await register(shortcut, async () => {
            // 显示并聚焦窗口
            const webview = getCurrentWebviewWindow()
            await webview.show()
            await webview.setFocus()

            // 导航到对应路由
            const route = routeMap[key]
            if (route) {
                router.push(route)
            }
        })
        console.log(`✅ 已注册全局快捷键: ${shortcut} -> ${getKeyLabel(key)}`)
        return true
    } catch (error) {
        console.error(`❌ 注册快捷键失败: ${shortcut}`, error)
        throw error
    }
}

// 取消注册快捷键
export async function unregisterShortcut(shortcut) {
    try {
        if (shortcut) {
            await unregister(shortcut)
            console.log(`🔓 已取消注册快捷键: ${shortcut}`)
        }
    } catch (error) {
        console.warn('取消快捷键失败:', error)
    }
}

// 检查快捷键是否已被占用
export async function checkShortcutAvailable(shortcut) {
    try {
        return !(await isRegistered(shortcut))
    } catch (error) {
        console.error('检查快捷键失败:', error)
        return false
    }
}
