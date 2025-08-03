<template>
    <div class="custom-titlebar" data-tauri-drag-region>
        <div class="titlebar-content" data-tauri-drag-region>
            <!-- 搜索框区域 -->
            <div class="titlebar-search" style="margin-right: 16px;">
                <el-input v-model="searchText" placeholder="搜索变量名/值/全部" clearable size="small" style="width: 320px;"
                    @input="onSearchInput">
                    <template #append>
                        <el-select v-model="searchType" size="small" style="width: 80px;" @change="onSearchInput">
                            <el-option label="全部" value="all" />
                            <el-option label="变量名" value="name" />
                            <el-option label="变量值" value="value" />
                        </el-select>
                    </template>
                </el-input>
            </div>

            <!-- 中间操作区域 -->
            <div class="titlebar-actions">
                <!-- 主题切换 -->
                <el-button @click="toggleTheme($event)" :icon="isDark ? Sunny : Moon" circle class="theme-btn" />

                <!-- 刷新按钮 -->
                <el-button @click="$emit('refresh')" size="small" :icon="Refresh" :loading="loading">
                    刷新
                </el-button>

                <!-- 管理员权限状态 -->
                <el-tag v-if="!isAdmin" type="warning" class="admin-status" round
                    @click="emit('requestAdminPrivileges')">
                    没有以管理员身份运行
                </el-tag>
                <el-tag v-else type="success" class="admin-status">
                    <el-icon>
                        <Check />
                    </el-icon>
                    管理员模式
                </el-tag>
            </div>

            <!-- 窗口控制按钮 -->
            <div class="window-controls" data-tauri-drag-region="false">
                <button class="title-bar-button minimize" @click="minimizeWindow" title="最小化">
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <rect x="2" y="5" width="8" height="2" fill="currentColor" />
                    </svg>
                </button>
                <button class="title-bar-button maximize" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
                    <svg width="12" height="12" viewBox="0 0 12 12" v-if="!isMaximized">
                        <rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1" />
                    </svg>
                    <svg width="12" height="12" viewBox="0 0 12 12" v-else>
                        <path d="M4 1 L10 1 L10 4 L8 4 L8 3 L4 3 Z" fill="none" stroke="currentColor"
                            stroke-width="1" />
                        <path d="M8 4 L10 4 L10 7 L8 7 Z" fill="none" stroke="currentColor" stroke-width="1" />
                        <rect x="2" y="3" width="6" height="6" fill="none" stroke="currentColor" stroke-width="1" />
                    </svg>
                </button>
                <button class="title-bar-button close" @click="closeWindow" title="关闭">
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M2 2 L10 10 M10 2 L2 10" stroke="currentColor" stroke-width="1.5"
                            stroke-linecap="round" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>

import {
    Setting,
    Moon,
    Sunny,
    Warning,
    Check,
    Refresh,
    Search,
    FullScreen,
    SemiSelect
} from '@element-plus/icons-vue'
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

const updateMaximized = async () => {
    isMaximized.value = await appWindow.isMaximized()
}

const minimizeWindow = () => {
    appWindow.minimize()
}

const toggleMaximize = async () => {
    await appWindow.toggleMaximize()
    updateMaximized()
}

const closeWindow = () => {
    appWindow.close()
}

onMounted(() => {
    updateMaximized()
    appWindow.onResized(() => updateMaximized())
})

const props = defineProps({
    isDark: Boolean,
    isAdmin: Boolean,
    loading: Boolean
})

const isDark = ref(false)

const searchText = ref('')
const searchType = ref('all')

const emit = defineEmits(['toggleTheme', 'requestAdminPrivileges', 'refresh', 'search'])

const onSearchInput = () => {
    emit('search', { text: searchText.value, type: searchType.value })
}

/**
 * 主题切换并带视图过渡动画
 * @param {MouseEvent} event
 */
const toggleTheme = (event) => {
    const enableTransitions = () =>
        'startViewTransition' in document &&
        window.matchMedia('(prefers-reduced-motion: no-preference)').matches

    const html = document.documentElement
    if (!enableTransitions() || !event) {
        isDark.value = !isDark.value
        if (isDark.value) {
            html.classList.add('dark')
            localStorage.setItem('theme', 'dark')
        } else {
            html.classList.remove('dark')
            localStorage.setItem('theme', 'light')
        }
        return
    }

    const x = event.clientX
    const y = event.clientY
    const clipPath = [
        `circle(0px at ${x}px ${y}px)`,
        `circle(${Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y))}px at ${x}px ${y}px)`
    ]

    document.startViewTransition(async () => {
        isDark.value = !isDark.value
        if (isDark.value) {
            html.classList.add('dark')
            localStorage.setItem('theme', 'dark')
        } else {
            html.classList.remove('dark')
            localStorage.setItem('theme', 'light')
        }
    }).ready.then(() => {
        html.animate(
            { clipPath: isDark.value ? clipPath.slice().reverse() : clipPath },
            {
                duration: 300,
                easing: 'ease-in',
                pseudoElement: `::view-transition-${isDark.value ? 'old' : 'new'}(root)`
            }
        )
    })
}

// 初始化主题
const initTheme = () => {
    const savedTheme = localStorage.getItem('theme')
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

    isDark.value = savedTheme === 'dark' || (savedTheme === null && systemPrefersDark)

    if (isDark.value) {
        document.documentElement.classList.add('dark')
    }



    const appWindow = getCurrentWindow()
    const isMaximized = ref(false)

    const updateMaximized = async () => {
        isMaximized.value = await appWindow.isMaximized()
    }

    const minimizeWindow = () => {
        appWindow.minimize()
    }

    const toggleMaximize = async () => {
        await appWindow.toggleMaximize()
        updateMaximized()
    }


    onMounted(() => {
        initTheme()
        updateMaximized()
        appWindow.onResized(() => updateMaximized())
    })
    const closeWindow = () => {
        appWindow.close()
    }

    onMounted(() => {
        initTheme()
        updateMaximized()
        appWindow.onResized(() => updateMaximized())
    })
}
</script>

<style lang="scss" scoped>
@use '../assets/styles/variables.scss' as *;

.custom-titlebar {
    height: 40px;
    background: var(--el-bg-color);
    border-bottom: 1px solid var(--el-border-color-lighter);
    user-select: none;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;

    .titlebar-search {
        -webkit-app-region: no-drag;
    }

    .titlebar-content {
        @include flex-between;
        height: 100%;
        padding: 0 var(--spacing-sm);

        .app-info {
            @include flex-start;
            gap: var(--spacing-sm);

            .app-icon {
                color: var(--el-color-primary);
                font-size: var(--font-size-medium);
            }

            .app-title {
                font-weight: var(--font-weight-primary);
                color: var(--el-text-color-primary);
                font-size: var(--font-size-small);
            }
        }

        .titlebar-actions {
            @include flex-start;
            gap: var(--spacing-sm);

            .theme-btn {
                width: 28px;
                height: 28px;
                border: none;
                background: transparent;

                &:hover {
                    background: var(--el-fill-color-light);
                }
            }

            .refresh-btn {
                width: 28px;
                height: 28px;
                border: none;
                background: transparent;

                &:hover {
                    background: var(--el-fill-color-light);
                }
            }
        }

        .window-controls {
            display: flex;
            align-items: center;
            gap: 4px;
            height: 100%;

            .title-bar-button {
                width: 32px;
                height: 32px;
                border: none;
                background: transparent;
                border-radius: 6px;
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--el-text-color-secondary);
                transition: background 0.2s, color 0.2s;
                margin: 0 1px;

                &:hover {
                    background: var(--el-fill-color-light);
                    color: var(--el-text-color-primary);
                }

                &.close:hover {
                    background: var(--el-color-danger);
                    color: #fff;
                }
            }
        }
    }

    // 防止拖拽区域影响按钮点击
    .titlebar-actions,
    .window-controls {
        -webkit-app-region: no-drag;
    }
}
</style>
