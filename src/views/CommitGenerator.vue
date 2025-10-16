<template>
    <div class="commit-generator">
        <div class="container">
            <div class="form-section">
                <el-form :model="form" label-position="top" :label-width="'auto'">
                    <el-row :gutter="16">
                        <el-col :span="6">
                            <el-form-item>
                                <el-select v-model="form.type" placeholder="提交类型">
                                    <el-option v-for="item in commitTypes" :key="item.value" :label="item.label"
                                        :value="item.value">
                                        <span>{{ form.useEmoji ? item.icon + ' ' : '' }}{{ item.label }}</span>
                                    </el-option>
                                </el-select>
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item>
                                <el-input v-model="form.scope" placeholder="范围" clearable />
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item>
                                <el-input v-model="form.contributors" placeholder="贡献者" clearable />
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item>
                                <el-input v-model="form.issueId" placeholder="问题ID" clearable />
                            </el-form-item>
                        </el-col>
                    </el-row>

                    <el-form-item>
                        <el-input ref="descriptionInputRef" v-model="form.description" autosize type="textarea"
                            clearable placeholder="修复了xxx问题" />
                    </el-form-item>
                </el-form>
            </div>

            <div class="result-section">
                <div class="result-card" @click="copyToClipboard(generatedCommit)">
                    <el-tooltip content="点击复制" placement="top">
                        <p class="copy-content">{{ generatedCommit }}</p>
                    </el-tooltip>
                </div>

                <div class="result-card" @click="copyToClipboard(generatedCommand)">
                    <el-tooltip content="点击复制" placement="top">
                        <p class="copy-content">{{ generatedCommand }}</p>
                    </el-tooltip>
                </div>
            </div>

            <div class="settings-section">
                <div class="settings-content">
                    <el-switch v-model="form.useEmoji" active-text="使用emoji" @change="saveEmojiPreference" />
                    <el-switch v-model="form.autoClassify" active-text="自动分类" @change="saveAutoClassifyPreference" />
                    <el-button @click="showClassifyRulesManager" :icon="Setting">
                        配置规则
                    </el-button>
                    <el-button @click="showCommitTypesManager" :icon="Setting">
                        管理提交类型
                    </el-button>
                    <el-button @click="clearAllFields" type="danger" :icon="Delete">
                        清除输入框信息
                    </el-button>
                </div>
            </div>
        </div>
    </div>

    <!-- 提交类型管理器对话框 -->
    <CommitTypesManager ref="commitTypesManagerRef" />

    <!-- 自动分类规则管理器对话框 -->
    <ClassifyRulesManager ref="classifyRulesManagerRef" />
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Setting, Delete } from '@element-plus/icons-vue'
import { useRouter, useRoute } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'
import { useCommitTypesStore } from '@/stores/commitTypes'
import CommitTypesManager from '@/components/CommitTypesManager.vue'
import ClassifyRulesManager from '@/components/ClassifyRulesManager.vue'

const router = useRouter()
const route = useRoute()
const settingsStore = useSettingsStore()
const commitTypesStore = useCommitTypesStore()
const commitTypesManagerRef = ref(null)
const classifyRulesManagerRef = ref(null)
const descriptionInputRef = ref(null)

const commitTypes = computed(() => commitTypesStore.allCommitTypes)

const form = ref({
    type: 'feat',
    scope: '',
    description: '',
    contributors: '',
    issueId: '',
    useEmoji: settingsStore.commitGenerator.useEmoji,
    autoClassify: settingsStore.commitGenerator.autoClassify
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

// 保存自动分类偏好设置
const saveAutoClassifyPreference = () => {
    settingsStore.setCommitGeneratorAutoClassify(form.value.autoClassify)
}

// 自动分类功能：根据描述文本自动判断提交类型
const autoClassifyCommitType = (description) => {
    if (!form.value.autoClassify || !description.trim()) {
        return
    }

    const lowerDesc = description.toLowerCase().trim()
    const classifyRules = settingsStore.commitGenerator.classifyRules

    // 遍历分类规则，找到匹配的类型
    for (const [type, rules] of Object.entries(classifyRules)) {
        // 确保该类型存在于提交类型列表中
        if (!commitTypesStore.hasCommitType(type)) {
            continue
        }

        // 检查"以...开始"规则
        if (rules.startsWith && rules.startsWith.length > 0) {
            if (rules.startsWith.some(keyword => lowerDesc.startsWith(keyword.toLowerCase()))) {
                form.value.type = type
                return
            }
        }

        // 检查"以...结尾"规则
        if (rules.endsWith && rules.endsWith.length > 0) {
            if (rules.endsWith.some(keyword => lowerDesc.endsWith(keyword.toLowerCase()))) {
                form.value.type = type
                return
            }
        }

        // 检查"包含..."规则（优先级最低）
        if (rules.contains && rules.contains.length > 0) {
            if (rules.contains.some(keyword => lowerDesc.includes(keyword.toLowerCase()))) {
                form.value.type = type
                return
            }
        }
    }
}

// 监听描述输入，自动分类
watch(() => form.value.description, (newDescription) => {
    autoClassifyCommitType(newDescription)
})

// 格式化贡献者信息
const formatContributors = (contributors) => {
    if (!contributors.trim()) {
        return ''
    }

    // 如果已经包含@符号，说明用户已经格式化了，直接返回
    if (contributors.includes('@')) {
        return contributors.trim()
    }

    // 分割贡献者（支持空格、逗号、&等分隔符）
    const names = contributors
        .split(/[\s,&]+/)
        .map(name => name.trim())
        .filter(name => name.length > 0)

    // 为每个名字添加@前缀，并用 & 连接
    return names.map(name => `@${name}`).join(' & ')
}

// 生成提交信息
const generatedCommit = computed(() => {
    const typeConfig = commitTypesStore.getCommitTypeByValue(form.value.type)
    const emoji = form.value.useEmoji && typeConfig ? typeConfig.icon + ' ' : ''
    const scope = form.value.scope.trim() ? `(${form.value.scope.trim()})` : ''

    let commit = `${emoji}${form.value.type}${scope}: ${form.value.description.trim()}`

    // 添加贡献者信息
    if (form.value.contributors.trim()) {
        const formattedContributors = formatContributors(form.value.contributors)
        commit += ` Thanks ${formattedContributors}`
    }

    // 添加问题ID
    if (form.value.issueId.trim()) {
        const issueId = form.value.issueId.trim().startsWith('#') ? form.value.issueId.trim() : `#${form.value.issueId.trim()}`
        commit += ` ${issueId}`
    }

    return commit
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

// 显示分类规则管理器
const showClassifyRulesManager = () => {
    classifyRulesManagerRef.value?.showDialog()
}

// 清除所有输入字段
const clearAllFields = () => {
    form.value.scope = ''
    form.value.description = ''
    form.value.contributors = ''
    form.value.issueId = ''
}
</script>

<style scoped>
@import '@/assets/styles/commit-generator.scss';
</style>
