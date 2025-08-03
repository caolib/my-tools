<template>
    <div class="custom-titlebar" data-tauri-drag-region>
        <div class="titlebar-content" data-tauri-drag-region>

            <!-- 中间操作区域 -->
            <div class="titlebar-actions">
                <!-- 主题切换 -->
                <el-button @click="toggleTheme($event)" :icon="isDark ? Sunny : Moon" circle class="theme-btn" />

                <!-- 刷新按钮 -->
                <el-button @click="$emit('refresh')" :loading="loading" class="refresh-btn">
                    <el-icon>
                        <Refresh />
                    </el-icon>
                </el-button>

                <!-- 管理员权限状态 -->
                <el-tag v-if="!isAdmin" type="warning" class="admin-status" round @click="requestAdminPrivileges">
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
            <div class="window-controls">
                <button id="titlebar-minimize" class="control-btn minimize-btn" @click="minimizeWindow">
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                    </svg>
                </button>
                <button id="titlebar-maximize" class="control-btn maximize-btn" @click="toggleMaximize">
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M3 3h6v6H3z" fill="none" stroke="currentColor" stroke-width="1.5" />
                    </svg>
                </button>
                <button id="titlebar-close" class="control-btn close-btn" @click="closeWindow">
                    <svg width="12" height="12" viewBox="0 0 12 12">
                        <path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ref, onMounted } from 'vue'
import {
    Setting,
    Moon,
    Sunny,
    Warning,
    Check,
    Refresh
} from '@element-plus/icons-vue'

defineProps({
    isDark: Boolean,
    isAdmin: Boolean,
    loading: Boolean
})

const isDark = ref(false)

defineEmits(['toggleTheme', 'requestAdminPrivileges', 'refresh'])

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
}

const appWindow = getCurrentWindow()

const minimizeWindow = () => {
    appWindow.minimize()
}

const toggleMaximize = () => {
    appWindow.toggleMaximize()
}

const closeWindow = () => {
    appWindow.close()
}

onMounted(() => {
    initTheme()
})
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
            @include flex-start;
            gap: 0;

            .control-btn {
                width: 40px;
                height: 40px;
                border: none;
                background: transparent;
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--el-text-color-secondary);
                transition: all var(--el-transition-duration);

                &:hover {
                    background: var(--el-fill-color-light);
                    color: var(--el-text-color-primary);
                }

                &.close-btn:hover {
                    background: var(--el-color-danger);
                    color: white;
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
