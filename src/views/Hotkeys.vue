<template>
    <div class="hotkeys-page">
        <div class="hotkeys-content">
            <!-- 当前快捷键显示 -->
            <div class="shortcut-display">
                <div class="display-content" :class="{ active: currentShortcut }">
                    <template v-if="currentShortcut">
                        <div class="shortcut-text">{{ currentShortcut }}</div>
                        <div class="shortcut-time">{{ currentTime }}</div>
                    </template>
                    <template v-else>
                        <div class="waiting-text">按下已注册的快捷键...</div>
                        <div class="waiting-hint">已注册 {{ registeredShortcuts.length }} 个快捷键</div>
                    </template>
                </div>
            </div>

            <!-- 手动注册快捷键 -->
            <el-card class="register-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Setting />
                        </el-icon>
                        <span>注册快捷键</span>
                    </div>
                </template>

                <el-form inline>
                    <el-form-item label="快捷键">
                        <el-input v-model="newShortcut" placeholder="例如: Ctrl+Shift+A" style="width: 200px"
                            @keydown="handleKeyCapture" clearable />
                    </el-form-item>
                    <el-form-item>
                        <el-button type="primary" @click="registerNewShortcut" :loading="registering"
                            :disabled="!newShortcut">
                            注册
                        </el-button>
                    </el-form-item>
                </el-form>

                <el-alert title="快捷键格式说明" type="info" :closable="false" class="tips-alert">
                    <div>支持的修饰键：Ctrl、Shift、Alt、Command (Mac)</div>
                    <div>示例：Ctrl+Shift+A、Alt+Space、F11</div>
                    <div>提示：在输入框中直接按键可自动识别</div>
                </el-alert>
            </el-card>

            <!-- 已注册的快捷键 -->
            <el-card class="shortcuts-card" shadow="hover" v-if="registeredShortcuts.length > 0">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Setting />
                        </el-icon>
                        <span>已注册的快捷键 ({{ registeredShortcuts.length }})</span>
                        <el-button size="small" @click="unregisterAllShortcuts" link type="danger">
                            清空所有
                        </el-button>
                    </div>
                </template>
                <div class="shortcuts-grid">
                    <el-tag v-for="(shortcut, index) in registeredShortcuts" :key="index" type="success" effect="plain"
                        size="large" closable @close="unregisterShortcut(shortcut)">
                        {{ shortcut }}
                    </el-tag>
                </div>
            </el-card>

            <!-- 快捷键历史列表 -->
            <el-card class="history-card" shadow="hover">
                <template #header>
                    <div class="card-header">
                        <el-icon class="header-icon">
                            <Clock />
                        </el-icon>
                        <span>监听到的快捷键 ({{ history.length }})</span>
                        <el-button size="small" @click="clearHistory" :disabled="history.length === 0" link>
                            清空
                        </el-button>
                    </div>
                </template>

                <div class="history-list" v-if="history.length > 0">
                    <el-scrollbar max-height="500px">
                        <div v-for="(item, index) in history" :key="index" class="history-item">
                            <div class="item-index">{{ history.length - index }}</div>
                            <div class="item-content">
                                <el-tag type="primary" effect="dark" size="large">
                                    {{ item.shortcut }}
                                </el-tag>
                                <span class="item-time">{{ item.time }}</span>
                            </div>
                        </div>
                    </el-scrollbar>
                </div>

                <el-empty v-else description="按下任意快捷键，将会显示在这里" :image-size="120">
                    <template #image>
                        <el-icon :size="80" color="#909399">
                            <Setting />
                        </el-icon>
                    </template>
                </el-empty>
            </el-card>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { ElMessage } from 'element-plus'
import { Clock, Setting } from '@element-plus/icons-vue'
import { register, unregister, unregisterAll, isRegistered } from '@tauri-apps/plugin-global-shortcut'

// 响应式数据
const currentShortcut = ref('')
const currentTime = ref('')
const history = ref([])
const registeredShortcuts = ref([])
const newShortcut = ref('')
const registering = ref(false)

let shortcutTimeoutId = null

// 格式化时间
const formatTime = (date) => {
    return date.toLocaleString('zh-CN', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false
    })
}

// 显示快捷键触发
const showShortcut = (shortcut) => {
    console.log('收到快捷键:', shortcut)

    currentShortcut.value = shortcut
    currentTime.value = formatTime(new Date())

    // 添加到历史记录（最新的在前面）
    history.value.unshift({
        shortcut,
        time: currentTime.value
    })

    // 限制历史记录数量
    if (history.value.length > 100) {
        history.value = history.value.slice(0, 100)
    }

    // 3秒后清除显示
    if (shortcutTimeoutId) {
        clearTimeout(shortcutTimeoutId)
    }
    shortcutTimeoutId = setTimeout(() => {
        currentShortcut.value = ''
        currentTime.value = ''
    }, 3000)
}

// 清空历史
const clearHistory = () => {
    history.value = []
    ElMessage.success('历史记录已清空')
}

// 键盘捕获辅助 - 自动识别按键组合
const handleKeyCapture = (event) => {
    // 阻止默认行为
    event.preventDefault()

    const modifiers = []
    if (event.ctrlKey || event.metaKey) modifiers.push('Ctrl')
    if (event.shiftKey) modifiers.push('Shift')
    if (event.altKey) modifiers.push('Alt')

    // 获取按键
    let key = event.key

    // 只按修饰键不处理
    if (key === 'Control' || key === 'Shift' || key === 'Alt' || key === 'Meta') {
        return
    }

    // 转换特殊键名
    if (key === ' ') key = 'Space'
    if (key.length === 1) key = key.toUpperCase()

    // 处理功能键
    if (key.startsWith('F') && key.length <= 3) {
        // F1-F12
        newShortcut.value = modifiers.length > 0
            ? modifiers.join('+') + '+' + key
            : key
        return
    }

    if (modifiers.length > 0) {
        newShortcut.value = modifiers.join('+') + '+' + key
    } else {
        newShortcut.value = key
    }
}

// 注册新快捷键
const registerNewShortcut = async () => {
    if (!newShortcut.value.trim()) {
        ElMessage.warning('请输入快捷键')
        return
    }

    const shortcutStr = newShortcut.value.trim()

    // 检查是否已注册
    if (registeredShortcuts.value.includes(shortcutStr)) {
        ElMessage.warning('该快捷键已注册')
        return
    }

    registering.value = true

    try {
        // 检查系统是否已注册
        const alreadyRegistered = await isRegistered(shortcutStr)
        if (alreadyRegistered) {
            ElMessage.warning('该快捷键已被系统注册')
            registering.value = false
            return
        }

        // 注册快捷键
        const currentShortcutStr = shortcutStr
        await register(currentShortcutStr, (event) => {
            if (event.state === 'Pressed') {
                showShortcut(currentShortcutStr)
            }
        })

        registeredShortcuts.value.push(currentShortcutStr)
        ElMessage.success(`快捷键 ${currentShortcutStr} 注册成功`)
        newShortcut.value = ''
        console.log(`✅ 成功注册快捷键: ${currentShortcutStr}`)
    } catch (error) {
        console.error('注册快捷键失败:', error)
        ElMessage.error(`注册失败: ${error.message || error}`)
    } finally {
        registering.value = false
    }
}

// 取消注册单个快捷键
const unregisterShortcut = async (shortcut) => {
    try {
        await unregister(shortcut)
        registeredShortcuts.value = registeredShortcuts.value.filter(s => s !== shortcut)
        ElMessage.success(`快捷键 ${shortcut} 已取消注册`)
        console.log(`✅ 已取消注册快捷键: ${shortcut}`)
    } catch (error) {
        console.error('取消注册失败:', error)
        ElMessage.error(`取消注册失败: ${error.message || error}`)
    }
}

// 清空所有快捷键
const unregisterAllShortcuts = async () => {
    try {
        await unregisterAll()
        registeredShortcuts.value = []
        ElMessage.success('已清空所有快捷键')
        console.log('✅ 已清空所有快捷键')
    } catch (error) {
        console.error('清空快捷键失败:', error)
        ElMessage.error(`清空失败: ${error.message || error}`)
    }
}

// 注册示例快捷键
const registerExampleShortcuts = async () => {
    // 注册几个示例快捷键
    const shortcuts = [
        'Ctrl+Shift+H',
        'Alt+Space',
        'F11'
    ]

    let successCount = 0

    for (const shortcutStr of shortcuts) {
        try {
            const currentShortcut = shortcutStr
            await register(currentShortcut, (event) => {
                if (event.state === 'Pressed') {
                    showShortcut(currentShortcut)
                }
            })
            registeredShortcuts.value.push(currentShortcut)
            successCount++
            console.log(`✅ 成功注册示例快捷键: ${currentShortcut}`)
        } catch (error) {
            console.warn(`❌ 注册快捷键失败: ${shortcutStr}`, error)
        }
    }

    if (successCount > 0) {
        ElMessage.success(`已注册 ${successCount} 个示例快捷键，你可以手动添加更多`)
    }
}

// 生命周期
onMounted(async () => {
    await registerExampleShortcuts()
})

onBeforeUnmount(async () => {
    // 清理定时器
    if (shortcutTimeoutId) {
        clearTimeout(shortcutTimeoutId)
    }

    // 取消所有注册的快捷键
    try {
        await unregisterAll()
        console.log('已清理所有快捷键')
    } catch (error) {
        console.error('清理快捷键失败:', error)
    }
})
</script>

<style lang="scss" scoped>
@use "../assets/styles/variables.scss" as *;

.hotkeys-page {
    padding: var(--spacing-xl);
    margin-top: 20px;

    .hotkeys-content {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-lg);

        // 快捷键显示区
        .shortcut-display {
            width: 100%;
            height: 300px;
            background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-7) 100%);
            border-radius: var(--el-border-radius-base);
            display: flex;
            align-items: center;
            justify-content: center;
            position: relative;
            overflow: hidden;
            box-shadow: var(--el-box-shadow);

            &::before {
                content: '';
                position: absolute;
                top: -50%;
                left: -50%;
                width: 200%;
                height: 200%;
                background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
                animation: pulse 3s ease-in-out infinite;
            }

            .display-content {
                z-index: 1;
                text-align: center;
                transition: all 0.3s ease;

                &.active {
                    transform: scale(1.1);

                    .shortcut-text {
                        animation: bounceIn 0.5s ease;
                    }
                }

                .shortcut-text {
                    font-size: 3.5rem;
                    font-weight: 700;
                    color: var(--el-color-primary);
                    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
                    margin-bottom: var(--spacing-md);
                    font-family: 'Consolas', 'Monaco', monospace;
                }

                .shortcut-time {
                    font-size: 1.2rem;
                    color: var(--el-text-color-regular);
                    font-weight: 500;
                }

                .waiting-text {
                    font-size: 1.8rem;
                    color: var(--el-text-color-secondary);
                    opacity: 0.8;
                    margin-bottom: var(--spacing-sm);
                }

                .waiting-hint {
                    font-size: 1rem;
                    color: var(--el-text-color-placeholder);
                    opacity: 0.6;
                }
            }
        }

        // 注册卡片
        .register-card {
            .card-header {
                display: flex;
                align-items: center;
                gap: var(--spacing-sm);

                .header-icon {
                    font-size: 1.2rem;
                    color: var(--el-color-primary);
                }

                span {
                    flex: 1;
                    font-weight: 600;
                    font-size: 1.1rem;
                }
            }

            .el-form {
                margin-bottom: var(--spacing-md);
            }

            .tips-alert {
                :deep(.el-alert__content) {
                    div {
                        line-height: 1.8;
                        font-size: 0.875rem;
                    }
                }
            }
        }

        // 快捷键卡片
        .shortcuts-card {
            .card-header {
                display: flex;
                align-items: center;
                gap: var(--spacing-sm);

                .header-icon {
                    font-size: 1.2rem;
                    color: var(--el-color-success);
                }

                span {
                    flex: 1;
                    font-weight: 600;
                    font-size: 1.1rem;
                }
            }

            .shortcuts-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
                gap: var(--spacing-sm);

                .el-tag {
                    justify-content: center;
                    padding: var(--spacing-sm) var(--spacing-md);
                    font-family: 'Consolas', 'Monaco', monospace;
                    cursor: pointer;
                    transition: all var(--el-transition-duration);

                    &:hover {
                        transform: scale(1.05);
                    }
                }
            }
        }

        // 历史记录卡片
        .history-card {
            .card-header {
                display: flex;
                align-items: center;
                gap: var(--spacing-sm);

                .header-icon {
                    font-size: 1.2rem;
                    color: var(--el-color-primary);
                }

                span {
                    flex: 1;
                    font-weight: 600;
                    font-size: 1.1rem;
                }
            }

            .history-list {
                .history-item {
                    padding: var(--spacing-md) var(--spacing-lg);
                    border-bottom: 1px solid var(--el-border-color-lighter);
                    transition: all var(--el-transition-duration);
                    display: flex;
                    align-items: center;
                    gap: var(--spacing-md);
                    animation: bounceIn 0.3s ease-out;

                    &:last-child {
                        border-bottom: none;
                    }

                    &:hover {
                        background-color: var(--el-fill-color-light);
                        transform: translateX(4px);
                    }

                    .item-index {
                        width: 40px;
                        height: 40px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        background: linear-gradient(135deg, var(--el-color-primary-light-9), var(--el-color-primary-light-7));
                        border-radius: 50%;
                        font-weight: 600;
                        font-size: 0.9rem;
                        color: var(--el-color-primary);
                        flex-shrink: 0;
                    }

                    .item-content {
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        gap: var(--spacing-md);
                        flex: 1;

                        .item-time {
                            font-size: 0.875rem;
                            color: var(--el-text-color-secondary);
                            font-family: 'Courier New', monospace;
                        }
                    }
                }
            }
        }
    }
}

// 动画
@keyframes pulse {

    0%,
    100% {
        transform: translate(-50%, -50%) scale(1);
        opacity: 0.3;
    }

    50% {
        transform: translate(-50%, -50%) scale(1.2);
        opacity: 0.1;
    }
}

@keyframes bounceIn {
    0% {
        transform: scale(0.3);
        opacity: 0;
    }

    50% {
        transform: scale(1.1);
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

// 响应式
@media (max-width: 768px) {
    .hotkeys-page {
        padding: var(--spacing-md);

        .shortcut-display {
            height: 200px;

            .display-content {
                .shortcut-text {
                    font-size: 2rem;
                }

                .shortcut-time {
                    font-size: 1rem;
                }
            }
        }
    }
}
</style>
