<template>
    <div class="commit-generator">
        <div class="container">
            <div class="form-section">
                <el-form :model="form" label-position="top" :label-width="'auto'">
                    <el-row :gutter="16">
                        <el-col :span="12">
                            <el-form-item>
                                <el-select v-model="form.type" placeholder="请选择提交类型">
                                    <el-option v-for="item in commitTypes" :key="item.value" :label="item.label"
                                        :value="item.value">
                                        <span>{{ form.useEmoji ? item.icon + ' ' : '' }}{{ item.label }}</span>
                                    </el-option>
                                </el-select>
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item>
                                <el-input v-model="form.scope" placeholder="影响范围，例如：ui, core" clearable />
                            </el-form-item>
                        </el-col>
                    </el-row>

                    <el-form-item>
                        <el-input ref="descriptionInputRef" v-model="form.description" autosize type="textarea"
                            clearable placeholder="修复了xxx问题" />
                    </el-form-item>

                    <el-row :gutter="16">
                        <el-col :span="12">
                            <el-form-item>
                                <el-switch v-model="form.useEmoji" active-text="使用 Emoji 表情"
                                    @change="saveEmojiPreference" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="12" style="text-align: right;">
                            <el-button @click="showCommitTypesManager" :icon="Setting">
                                管理提交类型
                            </el-button>
                        </el-col>
                    </el-row>
                </el-form>
            </div>

            <div class="result-section">
                <div class="result-card" @click="copyToClipboard(generatedCommit)" title="点击复制">
                    <p class="copy-content">{{ generatedCommit }}</p>
                </div>

                <div class="result-card" @click="copyToClipboard(generatedCommand)" title="点击复制">
                    <p class="copy-content">{{ generatedCommand }}</p>
                </div>
            </div>
        </div>
    </div>

    <!-- 提交类型管理器对话框 -->
    <CommitTypesManager ref="commitTypesManagerRef" />
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Setting } from '@element-plus/icons-vue'
import { useRouter, useRoute } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'
import { useCommitTypesStore } from '@/stores/commitTypes'
import CommitTypesManager from '@/components/CommitTypesManager.vue'

const router = useRouter()
const route = useRoute()
const settingsStore = useSettingsStore()
const commitTypesStore = useCommitTypesStore()
const commitTypesManagerRef = ref(null)
const descriptionInputRef = ref(null)

const commitTypes = computed(() => commitTypesStore.allCommitTypes)

const form = ref({
    type: 'feat',
    scope: '',
    description: '',
    useEmoji: settingsStore.commitGenerator.useEmoji
})

// 从路由参数设置类型
const updateTypeFromRoute = () => {
    if (route.query.type) {
        const type = route.query.type
        if (commitTypesStore.hasCommitType(type)) {
            form.value.type = type
        }
    }
}

// 监听路由变化
watch(() => route.query.type, (newType) => {
    if (newType && commitTypesStore.hasCommitType(newType)) {
        form.value.type = newType
    }
}, { immediate: true })

onMounted(() => {
    updateTypeFromRoute()

    // 自动聚焦到描述输入框
    setTimeout(() => {
        descriptionInputRef.value?.focus()
    }, 100)
})

// 保存 emoji 偏好设置
const saveEmojiPreference = () => {
    settingsStore.setCommitGeneratorUseEmoji(form.value.useEmoji)
}

// 生成提交信息
const generatedCommit = computed(() => {
    const typeConfig = commitTypesStore.getCommitTypeByValue(form.value.type)
    const emoji = form.value.useEmoji && typeConfig ? typeConfig.icon + ' ' : ''
    const scope = form.value.scope.trim() ? `(${form.value.scope.trim()})` : ''

    return `${emoji}${form.value.type}${scope}: ${form.value.description.trim()}`
})

// 生成 Git 命令
const generatedCommand = computed(() => {
    return `git commit -m "${generatedCommit.value}"`
})

// 复制到剪贴板
const copyToClipboard = async (text) => {
    try {
        await navigator.clipboard.writeText(text)
        ElMessage.success('已复制到剪贴板')

        // 执行复制后的行为
        await handleAfterCopyBehavior()
    } catch (err) {
        console.error('复制失败:', err)
        ElMessage.error('复制失败，请手动复制')
    }
}

// 处理复制后的行为
const handleAfterCopyBehavior = async () => {
    const behavior = settingsStore.afterCopyCommitBehavior

    if (behavior === 'none') {
        return
    }

    try {
        const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow')
        const { exit } = await import('@tauri-apps/plugin-process')
        const webview = getCurrentWebviewWindow()

        if (behavior === 'minimize') {
            // 最小化到托盘
            await webview.hide()
        } else if (behavior === 'quit') {
            // 退出应用
            await exit(0)
        }
    } catch (error) {
        console.error('执行复制后行为失败:', error)
    }
}

// 显示提交类型管理器
const showCommitTypesManager = () => {
    commitTypesManagerRef.value?.showDialog()
}
</script>

<style scoped>
@import '@/assets/styles/commit-generator.scss';
</style>
