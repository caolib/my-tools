<template>
    <el-dialog :modal="false" v-model="visible" align-center class="about-dialog">
        <div class="about-content">
            <!-- 应用图标和名称 -->
            <div class="app-header">
                <img src="/icon.png" alt="My Tools" class="app-icon" />
                <div class="app-info">
                    <h2 class="app-name">My Tools</h2>
                    <p class="app-version">版本 {{ appVersion }}</p>
                </div>
            </div>

            <!-- 应用描述 -->
            <div class="app-description">
                <p>我的各种工具</p>
            </div>

            <!-- 操作按钮 -->
            <div class="action-buttons">
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
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'

const visible = ref(false)
const appVersionData = ref('0.1.0')

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
    getAppVersion()
})

const showDialog = () => {
    visible.value = true
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

<style lang="scss" scoped>
@use "@/assets/styles/about-dialog.scss" as *;
</style>